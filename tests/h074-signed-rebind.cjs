'use strict';
const assert=require('node:assert/strict');
const crypto=require('node:crypto');
const sha=x=>crypto.createHash('sha256').update(JSON.stringify(x)).digest('hex');
const authority=crypto.generateKeyPairSync('ed25519');
const registry={jane:sha('state-jane'),toph:sha('state-toph')};
const roots={A:['rA'],B:['rB'],C:['rC'],Amirror:['rA']};
const ledger=[];
function append(type,data){const seq=ledger.length,prev=seq?ledger.at(-1).hash:'0'.repeat(64),body={seq,prev,type,data:structuredClone(data)};ledger.push({...body,hash:sha(body)});}
function verifyLedger(log){let prev='0'.repeat(64);return log.every((e,i)=>{const valid=e.seq===i&&e.prev===prev&&e.hash===sha({seq:i,prev,type:e.type,data:e.data});prev=e.hash;return valid;});}
let checkpoint='004C',epoch=2;
const bound=new Map(),used=new Set();
function independent(ids){const seen=new Set();for(const id of ids){const rs=roots[id];if(!rs||rs.some(r=>seen.has(r)))continue;rs.forEach(r=>seen.add(r));}return seen.size;}
function authorize(agent,nonce){const payload={agent,stateDigest:registry[agent],checkpoint,epoch,nonce,scope:'rebind'};return{payload,signature:crypto.sign(null,Buffer.from(JSON.stringify(payload)),authority.privateKey).toString('base64')};}
function redeem(ticket,attestors,evidence){const p=ticket?.payload;if(!p||!registry[p.agent])return 'INVALID_AGENT';if(!crypto.verify(null,Buffer.from(JSON.stringify(p)),authority.publicKey,Buffer.from(ticket.signature||'','base64')))return 'BAD_SIGNATURE';if(p.scope!=='rebind'||p.checkpoint!==checkpoint||p.epoch!==epoch)return 'STALE_SCOPE';if(p.stateDigest!==registry[p.agent]||evidence?.stateDigest!==p.stateDigest||evidence?.checkpoint!==checkpoint||evidence?.epoch!==epoch)return 'STATE_MISMATCH';if(independent(attestors)<3)return 'INSUFFICIENT_ROOTS';const id=sha(p);if(used.has(id))return 'REPLAY';if(bound.get(p.agent)?.epoch===epoch)return 'ALREADY_BOUND';used.add(id);bound.set(p.agent,{checkpoint,epoch,stateDigest:p.stateDigest});append('SIGNED_REBIND',{agent:p.agent,checkpoint,epoch,ticketHash:id});return 'REBOUND';}
function access(agent){const b=bound.get(agent);return b&&b.epoch===epoch&&b.checkpoint===checkpoint?'ALLOW':'BLOCK';}
function revoke(next){append('REVOCATION',{previous:checkpoint,next,epoch});checkpoint=next;epoch++;}
const ev=agent=>({stateDigest:registry[agent],checkpoint,epoch});
append('H073_PARENT',{checkpoint,epoch});
const jane=authorize('jane','nonce-jane');const toph=authorize('toph','nonce-toph');
assert.equal(access('jane'),'BLOCK');
assert.equal(redeem({...jane,payload:{...jane.payload,agent:'toph'}},['A','B','C'],ev('toph')),'BAD_SIGNATURE');
assert.equal(redeem(jane,['A','Amirror','B'],ev('jane')),'INSUFFICIENT_ROOTS');
assert.equal(redeem(jane,['A','B','C'],{...ev('jane'),stateDigest:'wrong'}),'STATE_MISMATCH');
assert.equal(redeem(jane,['A','B','C'],ev('jane')),'REBOUND');
assert.equal(access('jane'),'ALLOW');assert.equal(access('toph'),'BLOCK');
assert.equal(redeem(jane,['A','B','C'],ev('jane')),'REPLAY');
assert.equal(redeem(toph,['A','B','C'],ev('toph')),'REBOUND');
assert.equal(access('toph'),'ALLOW');
revoke('005D');assert.equal(access('jane'),'BLOCK');assert.equal(access('toph'),'BLOCK');
assert.equal(redeem(toph,['A','B','C'],ev('toph')),'STALE_SCOPE');
const fresh=authorize('jane','nonce-jane-next');assert.equal(redeem(fresh,['A','B','C'],ev('jane')),'REBOUND');
assert.equal(access('jane'),'ALLOW');assert.equal(access('toph'),'BLOCK');
assert(verifyLedger(ledger));const bad=structuredClone(ledger);bad[1].data.agent='toph';assert(!verifyLedger(bad));
assert.equal(ledger.length,5);
console.log(JSON.stringify({status:'PASS',checks:15,checkpoint,epoch,redeemedTickets:3,ledgerEntries:ledger.length,priorEpochBlocked:true,signatures:'Ed25519',caveat:'Single-process synthetic authority and evidence; no live agents or secure key custody.'},null,2));
module.exports={redeem,authorize,access};
