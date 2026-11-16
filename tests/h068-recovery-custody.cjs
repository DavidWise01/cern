'use strict';
// H068: signed receipt redelivery, never reexecute after a durable claim.
const assert=require('node:assert/strict');
const fs=require('node:fs'),os=require('node:os'),path=require('node:path');
const crypto=require('node:crypto'),{spawnSync}=require('node:child_process');
const sha=x=>crypto.createHash('sha256').update(JSON.stringify(x)).digest('hex');
const canon=x=>JSON.stringify(x);
function persistFile(file,content,exclusive=false){
 const fd=fs.openSync(file,exclusive?'wx':'w',0o600);
 try{fs.writeSync(fd,content);fs.fsyncSync(fd)}finally{fs.closeSync(fd)}
}
function syncDir(dir){try{const fd=fs.openSync(dir,'r');try{fs.fsyncSync(fd)}finally{fs.closeSync(fd)}}catch{}}
function readJson(f){return JSON.parse(fs.readFileSync(f,'utf8'))}
function assertReceipt(receipt,claim,custodianPub){
 if(!receipt||!receipt.payload||!receipt.signature)throw Error('RECEIPT_INVALID');
 if(receipt.payload.requestHash!==claim.requestHash||receipt.payload.campaign!==claim.campaign||receipt.payload.modelDigest!==claim.modelDigest)throw Error('RECEIPT_MISMATCH');
 if(!crypto.verify(null,Buffer.from(canon(receipt.payload)),custodianPub,Buffer.from(receipt.signature,'base64')))throw Error('RECEIPT_BAD_SIGNATURE');
}
function run(state,reqPath,developerPubFile,custodianPrivateFile,custodianPubFile,mode){
 const req=readJson(reqPath),developerPub=fs.readFileSync(developerPubFile),custodianPub=fs.readFileSync(custodianPubFile);
 if(!req.payload||req.payload.campaign!=='H068-FINAL'||!crypto.verify(null,Buffer.from(canon(req.payload)),developerPub,Buffer.from(req.signature||'','base64')))throw Error('BAD_REQUEST');
 const requestHash=sha(req),claimFile=path.join(state,'claim.json'),receiptFile=path.join(state,'receipt.json');
 fs.mkdirSync(state,{recursive:true});
 if(fs.existsSync(claimFile)){
  let claim;try{claim=readJson(claimFile)}catch{throw Error('FROZEN_INCOMPLETE_CLAIM')}
  if(claim.campaign!==req.payload.campaign||claim.requestHash!==requestHash||claim.modelDigest!==req.payload.modelDigest)throw Error('CLAIM_MISMATCH');
  if(!fs.existsSync(receiptFile))throw Error('FROZEN_NO_RECEIPT');
  let receipt;try{receipt=readJson(receiptFile);assertReceipt(receipt,claim,custodianPub)}catch{throw Error('FROZEN_INVALID_RECEIPT')}
  return {status:'REDELIVERED',receipt};
 }
 if(fs.existsSync(receiptFile))throw Error('FROZEN_ORPHAN_RECEIPT');
 const claim={campaign:req.payload.campaign,requestHash,modelDigest:req.payload.modelDigest};
 try{persistFile(claimFile,canon(claim),true)}catch(e){if(e.code==='EEXIST')throw Error('CLAIM_RACE_RETRY_RECOVERY');throw e}
 syncDir(state);
 if(mode==='after-claim')process.exit(79);
 // Synthetic sealed evaluator; only reached after first durable claim.
 const result={campaign:claim.campaign,requestHash,modelDigest:claim.modelDigest,score:0.61,runToken:sha(claim)};
 const privateKey=fs.readFileSync(custodianPrivateFile);
 const receipt={payload:result,signature:crypto.sign(null,Buffer.from(canon(result)),privateKey).toString('base64')};
 const temp=path.join(state,'receipt.tmp-'+process.pid);
 persistFile(temp,canon(receipt),true);
 fs.renameSync(temp,receiptFile);syncDir(state);
 if(mode==='after-receipt')process.exit(80);
 return {status:'FIRST_RESULT',receipt};
}
if(process.argv[2]==='--worker'){
 try{console.log(JSON.stringify(run(...process.argv.slice(3))))}catch(e){console.error('REJECTED: '+e.message);process.exitCode=2}
}else{
 const dir=fs.mkdtempSync(path.join(os.tmpdir(),'cern-h068-'));
 try{
  const dev=crypto.generateKeyPairSync('ed25519'),cust=crypto.generateKeyPairSync('ed25519');
  const devPub=path.join(dir,'dev.pub'),custPub=path.join(dir,'cust.pub'),custPriv=path.join(dir,'cust.priv');
  persistFile(devPub,dev.publicKey.export({format:'pem',type:'spki'}));
  persistFile(custPub,cust.publicKey.export({format:'pem',type:'spki'}));
  persistFile(custPriv,cust.privateKey.export({format:'pem',type:'pkcs8'}));
  function signed(p){return{payload:p,signature:crypto.sign(null,Buffer.from(canon(p)),dev.privateKey).toString('base64')}}
  const payload={campaign:'H068-FINAL',modelId:'m4',modelDigest:sha('frozen-model-v4'),nonce:'one-shot'};
  const request=path.join(dir,'request.json');persistFile(request,canon(signed(payload)));
  function call(state,req=request,mode='normal'){return spawnSync(process.execPath,[__filename,'--worker',state,req,devPub,custPriv,custPub,mode],{encoding:'utf8'})}
  const normal=path.join(dir,'normal'),first=call(normal);assert.equal(first.status,0,first.stderr);
  const firstResult=JSON.parse(first.stdout);assert.equal(firstResult.status,'FIRST_RESULT');assert.equal(firstResult.receipt.payload.score,.61);
  const replay=call(normal);assert.equal(replay.status,0,replay.stderr);assert.equal(JSON.parse(replay.stdout).status,'REDELIVERED');
  assert.deepEqual(JSON.parse(replay.stdout).receipt,firstResult.receipt);
  const alternate=path.join(dir,'alternate.json');persistFile(alternate,canon(signed({...payload,modelId:'m1',modelDigest:sha('different-model'),nonce:'different'})));
  const mismatch=call(normal,alternate);assert.equal(mismatch.status,2);assert.match(mismatch.stderr,/CLAIM_MISMATCH/);
  const crashClaim=path.join(dir,'crash-claim');assert.equal(call(crashClaim,request,'after-claim').status,79);
  const recoveryFrozen=call(crashClaim);assert.equal(recoveryFrozen.status,2);assert.match(recoveryFrozen.stderr,/FROZEN_NO_RECEIPT/);
  assert(!fs.existsSync(path.join(crashClaim,'receipt.json')));
  const crashReceipt=path.join(dir,'crash-receipt');assert.equal(call(crashReceipt,request,'after-receipt').status,80);
  const recovery=call(crashReceipt);assert.equal(recovery.status,0,recovery.stderr);assert.equal(JSON.parse(recovery.stdout).status,'REDELIVERED');assert.equal(JSON.parse(recovery.stdout).receipt.payload.score,.61);
  const corrupt=path.join(dir,'corrupt');fs.mkdirSync(corrupt);fs.copyFileSync(path.join(normal,'claim.json'),path.join(corrupt,'claim.json'));const forged=readJson(path.join(normal,'receipt.json'));forged.payload.score=.99;persistFile(path.join(corrupt,'receipt.json'),canon(forged));
  const corruptResult=call(corrupt);assert.equal(corruptResult.status,2);assert.match(corruptResult.stderr,/FROZEN_INVALID_RECEIPT/);
  const orphan=path.join(dir,'orphan');fs.mkdirSync(orphan);fs.copyFileSync(path.join(normal,'receipt.json'),path.join(orphan,'receipt.json'));const orphanResult=call(orphan);assert.equal(orphanResult.status,2);assert.match(orphanResult.stderr,/FROZEN_ORPHAN_RECEIPT/);
  const forgedRequest=path.join(dir,'forged-request.json');const falsified=signed(payload);falsified.payload.modelId='m1';persistFile(forgedRequest,canon(falsified));const bad=call(path.join(dir,'fresh'),forgedRequest);assert.equal(bad.status,2);assert.match(bad.stderr,/BAD_REQUEST/);
  console.log(JSON.stringify({status:'PASS',checks:9,first_score:.61,receipt_redelivery:true,alternate_model_rejected:true,claim_crash_frozen:true,receipt_crash_redelivered:true,corrupt_receipt_frozen:true,orphan_receipt_frozen:true,forged_request_rejected:true,limitations:'Local FS and simulated custody; no secure principal isolation, distributed locking, or externally anchored receipt'},null,2));
 }finally{fs.rmSync(dir,{recursive:true,force:true})}
}