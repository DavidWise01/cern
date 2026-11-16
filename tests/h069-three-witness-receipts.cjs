'use strict';
const assert=require('node:assert/strict');
const crypto=require('node:crypto');
const sha=b=>crypto.createHash('sha256').update(b).digest('hex');
const canonical=x=>JSON.stringify(x);
const keys=()=>crypto.generateKeyPairSync('ed25519');
const custodian=keys();
const witnesses={A:keys(),Amirror:keys(),B:keys(),C:keys(),D:keys(),unknown:keys()};
const registry={A:{roots:['R1'],key:witnesses.A.publicKey},Amirror:{roots:['R1'],key:witnesses.Amirror.publicKey},B:{roots:['R2'],key:witnesses.B.publicKey},C:{roots:['R3'],key:witnesses.C.publicKey},D:{roots:['R1','R4'],key:witnesses.D.publicKey}};
const original={campaign:'H069-FINAL',claimHash:sha('claimed-once'),modelDigest:sha('frozen-m4'),score:0.61,runToken:sha('one-run')};
const alternate={...original,score:.99};
function receipt(payload){const bytes=canonical(payload);return {payload,signature:crypto.sign(null,Buffer.from(bytes),custodian.privateKey).toString('base64')}}
function verifyReceipt(r){return !!r&&r.payload?.campaign==='H069-FINAL'&&r.payload.claimHash===original.claimHash&&r.payload.modelDigest===original.modelDigest&&r.payload.runToken===original.runToken&&typeof r.payload.score==='number'&&crypto.verify(null,Buffer.from(canonical(r.payload)),custodian.publicKey,Buffer.from(r.signature||'','base64'))}
const good=receipt(original),conflict=receipt(alternate);
function attest(witnessId,r){const payload={witnessId,campaign:'H069-FINAL',receiptDigest:sha(canonical(r))};return {payload,signature:crypto.sign(null,Buffer.from(canonical(payload)),witnesses[witnessId].privateKey).toString('base64'),receipt:r}}
function audit(packets){const accepted=[],rejected=[],byWitness=new Map();
 for(const p of packets){const id=p?.payload?.witnessId,entry=registry[id];
  if(!entry){rejected.push({id,reason:'unknown-witness'});continue}
  const validSig=crypto.verify(null,Buffer.from(canonical(p.payload)),entry.key,Buffer.from(p.signature||'','base64'));
  if(!validSig||!verifyReceipt(p.receipt)||p.payload.campaign!=='H069-FINAL'||p.payload.receiptDigest!==sha(canonical(p.receipt))){rejected.push({id,reason:'invalid-signature-or-receipt'});continue}
  accepted.push(p);if(!byWitness.has(id))byWitness.set(id,new Set());byWitness.get(id).add(p.payload.receiptDigest);
 }
 const equivocations=[...byWitness].filter(([,digests])=>digests.size>1).map(([id])=>id);
 const receipts=[...new Set(accepted.map(p=>p.payload.receiptDigest))];
 if(equivocations.length||receipts.length>1)return {state:'CONFLICT_FREEZE',independent:0,receipts:receipts.length,rejected,equivocations};
 const unique=[...new Map(accepted.map(p=>[p.payload.witnessId,p])).values()];
 function maximalDisjoint(items){let best=[];function recurse(i,chosen,roots){if(i===items.length){if(chosen.length>best.length)best=chosen;return}recurse(i+1,chosen,roots);const id=items[i].payload.witnessId,rs=registry[id].roots;if(rs.every(x=>!roots.has(x)))recurse(i+1,[...chosen,id],new Set([...roots,...rs]));}recurse(0,[],new Set());return best}
 const roots=maximalDisjoint(unique);
 return {state:roots.length>=3?'ANCHORED':'INSUFFICIENT_INDEPENDENT_ROOTS',independent:roots.length,selected:roots,receipts:receipts.length,rejected,equivocations};
}
const a=attest('A',good),mirror=attest('Amirror',good),b=attest('B',good),c=attest('C',good),d=attest('D',good);
assert.deepEqual({state:audit([a,b,c]).state,count:audit([a,b,c]).independent},{state:'ANCHORED',count:3});
assert.equal(audit([a,mirror,b]).independent,2);
assert.equal(audit([a,mirror,b,c]).independent,3);
assert.equal(audit([a,b,d]).state,'INSUFFICIENT_INDEPENDENT_ROOTS');
assert.equal(audit([a,b,c,attest('D',conflict)]).state,'CONFLICT_FREEZE');
assert.deepEqual(audit([a,attest('A',conflict),b,c]).equivocations,['A']);
const tampered=structuredClone(a);tampered.payload.receiptDigest=sha('forged');assert.equal(audit([tampered,b,c]).state,'INSUFFICIENT_INDEPENDENT_ROOTS');
assert.equal(audit([attest('unknown',good),a,b]).state,'INSUFFICIENT_INDEPENDENT_ROOTS');
const changed=structuredClone(c);changed.receipt.payload.score=.9;assert.equal(audit([a,b,changed]).state,'INSUFFICIENT_INDEPENDENT_ROOTS');
assert.equal(audit([a,a,b,c]).state,'ANCHORED');
const history=[];function append(kind,data){const index=history.length,previous=index?history.at(-1).digest:'0'.repeat(64),entry={index,previous,kind,data};history.push({...entry,digest:sha(canonical(entry))})}
function verifyHistory(log){let prev='0'.repeat(64);return log.every((e,i)=>{const ok=e.index===i&&e.previous===prev&&e.digest===sha(canonical({index:i,previous:prev,kind:e.kind,data:e.data}));prev=e.digest;return ok})}
append('H068_PARENT',{claimHash:original.claimHash});append('RECEIPT',{digest:sha(canonical(good))});append('WITNESS_QUORUM',audit([a,b,c]));append('CONFLICT_COUNTEREXAMPLE',audit([a,b,c,attest('D',conflict)]));assert(verifyHistory(history));const broken=structuredClone(history);broken[2].data.state='REJECTED';assert(!verifyHistory(broken));
console.log(JSON.stringify({status:'PASS',assertion_cases:11,valid:audit([a,b,c]),mirror:audit([a,mirror,b]),overlap:audit([a,b,d]),conflict:audit([a,b,c,attest('D',conflict)]),equivocation:audit([a,attest('A',conflict),b,c]),append_only_events:history.length,limits:'Synthetic key registry and signed receipt fixtures; keys share local process, no external independent storage or authentication.'},null,2));
module.exports={audit};