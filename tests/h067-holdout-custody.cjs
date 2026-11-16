'use strict';
const assert=require('node:assert/strict');
const fs=require('node:fs'),os=require('node:os'),path=require('node:path');
const crypto=require('node:crypto'),{spawnSync}=require('node:child_process');
const digest=x=>crypto.createHash('sha256').update(JSON.stringify(x)).digest('hex');
const stable=x=>JSON.stringify(x);
const sampleScores={m4:0.61,m1:0.69}; // Custodian-only fixture in a real deployment.
const campaign='H067-FINAL-2026-11';
function verifyRequest(request,pub){
 const {payload,signature}=request;
 if(!payload||typeof signature!=='string'||payload.campaign!==campaign||typeof payload.modelDigest!=='string'||payload.modelDigest.length!==64||!Object.hasOwn(sampleScores,payload.modelId))throw Error('BAD_REQUEST');
 if(!crypto.verify(null,Buffer.from(stable(payload)),pub,Buffer.from(signature,'base64')))throw Error('BAD_SIGNATURE');
}
function custodian(stateDir,requestFile,publicKeyFile,crash=false){
 const req=JSON.parse(fs.readFileSync(requestFile,'utf8'));
 const pub=fs.readFileSync(publicKeyFile);
 verifyRequest(req,pub);
 fs.mkdirSync(stateDir,{recursive:true});
 const gate=path.join(stateDir,'claim.json');
 // O_EXCL provides an atomic one-time claim across processes on a local FS.
 // Claim is durable BEFORE the score is released. Crash means no re-issue.
 const claim={campaign,requestHash:digest(req),modelDigest:req.payload.modelDigest};
 let fd;
 try{fd=fs.openSync(gate,'wx',0o600)}catch(e){if(e.code==='EEXIST')throw Error('ALREADY_CLAIMED');throw e}
 try{fs.writeSync(fd,stable(claim));fs.fsyncSync(fd)}finally{fs.closeSync(fd)}
 // Sync parent directory where supported: intended for a real local filesystem.
 try{const df=fs.openSync(stateDir,'r');try{fs.fsyncSync(df)}finally{fs.closeSync(df)}}catch{}
 if(crash)process.exit(79);
 const receipt={campaign,modelId:req.payload.modelId,modelDigest:req.payload.modelDigest,requestHash:digest(req),score:sampleScores[req.payload.modelId],claimHash:digest(claim)};
 const temp=path.join(stateDir,'receipt.tmp-'+process.pid);
 fs.writeFileSync(temp,stable(receipt),{flag:'wx',mode:0o600});
 const rf=fs.openSync(temp,'r');try{fs.fsyncSync(rf)}finally{fs.closeSync(rf)}
 fs.renameSync(temp,path.join(stateDir,'receipt.json'));
 return receipt;
}
if(process.argv[2]==='--custodian'){
 try{console.log(JSON.stringify(custodian(process.argv[3],process.argv[4],process.argv[5],process.argv[6]==='--crash')))}
 catch(e){console.error('REJECTED: '+e.message);process.exitCode=2}
}else{
 const dir=fs.mkdtempSync(path.join(os.tmpdir(),'cern-h067-'));
 try{
  const {publicKey,privateKey}=crypto.generateKeyPairSync('ed25519');
  const pk=path.join(dir,'pub.pem');fs.writeFileSync(pk,publicKey.export({type:'spki',format:'pem'}));
  const payload={campaign,modelId:'m4',modelDigest:digest('frozen-model-m4'),nonce:'single-evaluation-right'};
  const signed=p=>({payload:p,signature:crypto.sign(null,Buffer.from(stable(p)),privateKey).toString('base64')});
  const reqPath=path.join(dir,'request.json');fs.writeFileSync(reqPath,stable(signed(payload)));
  function call(state,request=reqPath,crash=false){return spawnSync(process.execPath,[__filename,'--custodian',state,request,pk,...(crash?['--crash']:[])],{encoding:'utf8'});}
  const state=path.join(dir,'campaign');
  const first=call(state);assert.equal(first.status,0,first.stderr);
  const receipt=JSON.parse(first.stdout);assert.equal(receipt.score,.61);assert.equal(receipt.modelId,'m4');
  assert(fs.existsSync(path.join(state,'claim.json')));assert(fs.existsSync(path.join(state,'receipt.json')));
  const replay=call(state);assert.equal(replay.status,2);assert.match(replay.stderr,/ALREADY_CLAIMED/);
  const alternate=path.join(dir,'alternate.json');fs.writeFileSync(alternate,stable(signed({...payload,modelId:'m1',modelDigest:digest('frozen-model-m1'),nonce:'new-nonce'})));
  const secondModel=call(state,alternate);assert.equal(secondModel.status,2);assert.match(secondModel.stderr,/ALREADY_CLAIMED/);
  const forged=path.join(dir,'forged.json');fs.writeFileSync(forged,stable({...signed(payload),payload:{...payload,modelId:'m1'}}));
  const invalid=call(path.join(dir,'fresh'),forged);assert.equal(invalid.status,2);assert.match(invalid.stderr,/BAD_SIGNATURE/);assert(!fs.existsSync(path.join(dir,'fresh','claim.json')));
  const crashState=path.join(dir,'crash');const crashed=call(crashState,reqPath,true);assert.equal(crashed.status,79);
  assert(fs.existsSync(path.join(crashState,'claim.json')));assert(!fs.existsSync(path.join(crashState,'receipt.json')));
  const afterCrash=call(crashState);assert.equal(afterCrash.status,2);assert.match(afterCrash.stderr,/ALREADY_CLAIMED/);
  const corruptState=path.join(dir,'corrupt');fs.mkdirSync(corruptState);fs.writeFileSync(path.join(corruptState,'claim.json'),'partial');
  const afterPartial=call(corruptState);assert.equal(afterPartial.status,2);assert.match(afterPartial.stderr,/ALREADY_CLAIMED/);
  const summary={status:'PASS',caseCount:7,firstResult:receipt.score,acceptedRequests:1,replayRejected:true,alternateModelRejected:true,invalidSignatureRejected:true,crashFailsClosed:true,partialClaimFailsClosed:true,modelDigestBound:true,receiptMatchesClaim:receipt.claimHash===digest(JSON.parse(fs.readFileSync(path.join(state,'claim.json'),'utf8'))),limitations:'Local filesystem O_EXCL, simulated independent custodian in a subprocess. Not a permission-isolated server, distributed transaction, or externally anchored audit.'};
  assert(summary.receiptMatchesClaim);console.log(JSON.stringify(summary,null,2));
 }finally{fs.rmSync(dir,{recursive:true,force:true})}
}