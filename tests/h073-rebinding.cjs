'use strict';
const assert=require('node:assert/strict'),crypto=require('node:crypto');
const digest=x=>crypto.createHash('sha256').update(JSON.stringify(x)).digest('hex');
const roots={A:['R1'],B:['R2'],C:['R3'],Amirror:['R1'],D:['R4']};
const old={id:'003A',digest:digest('checkpoint-003A'),status:'REVOKED'};
const replacement={id:'004C',digest:digest('checkpoint-004C'),status:'ACTIVE',receipt:digest('receipt-004C')};
const agents=new Map([
 ['jane',{pinned:'003A',state:digest('jane-state'),allowed:false,required:['policy-v2','proof-C']}],
 ['toph',{pinned:'003A',state:digest('toph-state'),allowed:false,required:['policy-v2','proof-C']}]
]);
const ledger=[];
function append(type,data){const sequence=ledger.length,previous=ledger.length?ledger.at(-1).hash:'0'.repeat(64);const entry={sequence,previous,type,data:structuredClone(data)};ledger.push({...entry,hash:digest(entry)});}
function verifyLedger(entries){let previous='0'.repeat(64);return entries.every((e,i)=>{const ok=e.sequence===i&&e.previous===previous&&e.hash===digest({sequence:i,previous,type:e.type,data:e.data});previous=e.hash;return ok;});}
function independent(witnesses){const unique=[...new Set(witnesses)],chosen=[];function go(i,used){if(i===unique.length)return chosen.length;let best=go(i+1,used);const rr=roots[unique[i]]||[];if(rr.length&&rr.every(r=>!used.has(r))){chosen.push(unique[i]);best=Math.max(best,go(i+1,new Set([...used,...rr])));chosen.pop();}return best;}return go(0,new Set());}
function issueRequest(agent,checkpoint,observedDigest,attestors,review,authorization){
 const a=agents.get(agent);if(!a)return 'UNKNOWN_AGENT';
 if(checkpoint.id!==replacement.id||checkpoint.status!=='ACTIVE'||checkpoint.digest!==replacement.digest||checkpoint.receipt!==replacement.receipt)return 'INVALID_REPLACEMENT';
 if(a.allowed)return 'ALREADY_BOUND';
 if(independent(attestors)<3)return 'INSUFFICIENT_WITNESSES';
 if(observedDigest!==a.state||!review||review.stateDigest!==a.state||review.checkpointDigest!==replacement.digest||!a.required.every(x=>review.evidence.includes(x)))return 'STATE_REVALIDATION_FAILED';
 if(!authorization||authorization.agent!==agent||authorization.from!=='003A'||authorization.to!=='004C'||authorization.stateDigest!==a.state||authorization.approved!==true)return 'AUTHORIZATION_REQUIRED';
 a.pinned=replacement.id;a.allowed=true;
 append('EXPLICIT_REBIND',{agent,from:old.id,to:replacement.id,stateDigest:a.state,attestors:[...new Set(attestors)],evidence:review.evidence,authorizationId:authorization.id});
 return 'REBOUND';
}
function access(agent){const a=agents.get(agent);return a&&a.allowed&&a.pinned===replacement.id?'ALLOW':'DENY_STALE';}
function validReview(agent){const a=agents.get(agent);return{stateDigest:a.state,checkpointDigest:replacement.digest,evidence:['policy-v2','proof-C']};}
function validAuthorization(agent){return{id:'authorization-'+agent,agent,from:'003A',to:'004C',stateDigest:agents.get(agent).state,approved:true};}
append('H072_REVOCATION',{old:old.id,consumers:['jane','toph']});append('REPLACEMENT_VERIFIED',{replacement,witnesses:['A','B','C']});
assert.equal(access('jane'),'DENY_STALE');assert.equal(access('toph'),'DENY_STALE');
assert.equal(issueRequest('jane',replacement,agents.get('jane').state,['A','Amirror','B'],validReview('jane'),validAuthorization('jane')),'INSUFFICIENT_WITNESSES');
assert.equal(issueRequest('jane',replacement,'wrong-digest',['A','B','C'],validReview('jane'),validAuthorization('jane')),'STATE_REVALIDATION_FAILED');
assert.equal(issueRequest('jane',replacement,agents.get('jane').state,['A','B','C'],{...validReview('jane'),evidence:['policy-v2']},validAuthorization('jane')),'STATE_REVALIDATION_FAILED');
assert.equal(issueRequest('jane',replacement,agents.get('jane').state,['A','B','C'],validReview('jane'),null),'AUTHORIZATION_REQUIRED');
assert.equal(issueRequest('jane',replacement,agents.get('jane').state,['A','B','C'],validReview('jane'),{...validAuthorization('jane'),to:'003B'}),'AUTHORIZATION_REQUIRED');
assert.equal(issueRequest('jane',{...replacement,digest:'forged'},agents.get('jane').state,['A','B','C'],validReview('jane'),validAuthorization('jane')),'INVALID_REPLACEMENT');
assert.equal(issueRequest('jane',replacement,agents.get('jane').state,['A','B','C'],validReview('jane'),validAuthorization('jane')),'REBOUND');
assert.equal(access('jane'),'ALLOW');assert.equal(access('toph'),'DENY_STALE');
assert.equal(issueRequest('jane',replacement,agents.get('jane').state,['A','B','C'],validReview('jane'),validAuthorization('jane')),'ALREADY_BOUND');
assert.equal(issueRequest('toph',replacement,agents.get('toph').state,['A','B','C'],validReview('toph'),validAuthorization('toph')),'REBOUND');
assert.equal(access('toph'),'ALLOW');assert(verifyLedger(ledger));
const modified=structuredClone(ledger);modified[0].data.old='003B';assert(!verifyLedger(modified));
assert.equal(ledger.filter(e=>e.type==='EXPLICIT_REBIND').length,2);
console.log(JSON.stringify({status:'PASS',scenarios:12,originalCheckpoint:old.id,replacement:replacement.id,quorum:3,rebound:['jane','toph'],auditRecords:ledger.length,mirrorRejected:true,replayRejected:true,oldHistoryPreserved:true,caveat:'Synthetic witness registry, review evidence and authorization objects; not externally authenticated.'},null,2));
module.exports={independent,issueRequest,access};
