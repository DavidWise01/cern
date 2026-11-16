'use strict';
const assert=require('node:assert/strict');
const crypto=require('node:crypto');
const sha=x=>crypto.createHash('sha256').update(JSON.stringify(x)).digest('hex');
const keys=Object.fromEntries(['A','B','C','D','A_mirror'].map(x=>[x,crypto.generateKeyPairSync('ed25519')]));
const roots={A:['R1'],B:['R2'],C:['R3'],D:['R4'],A_mirror:['R1']};
const parent=sha({checkpoint:'002',parent:'001'});
const receipt=sha({campaign:'H071',score:0.61,model:'m4'});
const branchA=sha({seq:3,parent,receipt,branch:'003A'});
const branchB=sha({seq:3,parent,receipt,branch:'003B'});
const deadline=100; // synthetic monotonic observation clock, NOT a physical time guarantee
function signed(root,branch,arrival){const payload={root,branch,parent,receipt,arrival};return{payload,signature:crypto.sign(null,Buffer.from(JSON.stringify(payload)),keys[root].privateKey).toString('base64')}}
function verify(e){const p=e?.payload;if(!p||!roots[p.root]||![branchA,branchB].includes(p.branch)||p.parent!==parent||p.receipt!==receipt||!Number.isSafeInteger(p.arrival)||p.arrival<0)return false;return crypto.verify(null,Buffer.from(JSON.stringify(p)),keys[p.root].publicKey,Buffer.from(e.signature||'','base64'))}
function disjointCount(ids){let best=0;function visit(i,used,n){if(i===ids.length){best=Math.max(best,n);return}visit(i+1,used,n);const r=roots[ids[i]];if(r.every(x=>!used.has(x)))visit(i+1,new Set([...used,...r]),n+1)}visit(0,new Set(),0);return best}
function evaluate(events,now){const valid=events.filter(e=>verify(e)&&e.payload.arrival<=now),invalid=events.filter(e=>!verify(e));const branches=new Set(valid.map(e=>e.payload.branch));const w=new Map();for(const e of valid){if(!w.has(e.payload.root))w.set(e.payload.root,new Set());w.get(e.payload.root).add(e.payload.branch)}const equivocation=[...w].filter(([,bs])=>bs.size>1).map(([r])=>r);
 if(branches.size>1||equivocation.length)return {state:'FORK_FREEZE',independent:0,valid:valid.length,invalid:invalid.length,equivocation,knownBranches:[...branches].sort()};
 const votes=[...new Set(valid.filter(e=>e.payload.branch===branchA).map(e=>e.payload.root))],independent=disjointCount(votes);
 return {state:now<deadline?'OBSERVING':independent>=3?'FINALITY_ELIGIBLE':'INSUFFICIENT_WITNESSES',independent,valid:valid.length,invalid:invalid.length,equivocation,knownBranches:[...branches].sort()};
}
const A=signed('A',branchA,5),B=signed('B',branchA,15),C=signed('C',branchA,25);
assert.equal(evaluate([A,B,C],99).state,'OBSERVING');
assert.equal(evaluate([A,B,C],100).state,'FINALITY_ELIGIBLE');
assert.equal(evaluate([A,B],100).state,'INSUFFICIENT_WITNESSES');
assert.equal(evaluate([A,B,signed('A_mirror',branchA,26)],100).independent,2);
assert.equal(evaluate([A,B,C,signed('D',branchB,110)],100).state,'FINALITY_ELIGIBLE'); // unseen fork, not omniscience
assert.equal(evaluate([A,B,C,signed('D',branchB,110)],110).state,'FORK_FREEZE');
assert.deepEqual(evaluate([A,B,C,signed('A',branchB,111)],111).equivocation,['A']);
const lateC=signed('C',branchA,105);assert.equal(evaluate([A,B,lateC],100).state,'INSUFFICIENT_WITNESSES');assert.equal(evaluate([A,B,lateC],105).state,'FINALITY_ELIGIBLE');
const forged=structuredClone(C);forged.payload.branch=branchB;assert.equal(evaluate([A,B,forged],100).state,'INSUFFICIENT_WITNESSES');
const log=[];function append(type,data){const seq=log.length,prev=seq?log.at(-1).hash:'0'.repeat(64),item={seq,prev,type,data:structuredClone(data)};log.push({...item,hash:sha(item)})}
function verifyLog(xs){let prev='0'.repeat(64);return xs.every((x,i)=>{const ok=x.seq===i&&x.prev===prev&&x.hash===sha({seq:i,prev,type:x.type,data:x.data});prev=x.hash;return ok})}
const fork=signed('D',branchB,110);
append('H070_PARENT',{receipt,branchA,branchB});append('DECISION_AT_DEADLINE',{now:100,...evaluate([A,B,C],100)});append('LATE_COMPETING_BRANCH',{arrival:110,witness:'D',branch:branchB});append('TRUST_REVOKED',{now:110,...evaluate([A,B,C,fork],110)});
assert.equal(log[1].data.state,'FINALITY_ELIGIBLE');assert.equal(log[3].data.state,'FORK_FREEZE');
assert(verifyLog(log));const changed=structuredClone(log);changed[1].data.state='FORK_FREEZE';assert(!verifyLog(changed));
console.log(JSON.stringify({status:'PASS',deadline,threeRoots:'FINALITY_ELIGIBLE',lateFork:'FORK_FREEZE',historicalDecisionPreserved:true,mirrorVotes:2,forgedSignatureRejected:true,lateSameBranchEligible:true,ledgerEvents:log.length,warning:'Synthetic observation time and locally generated keys. Eligible finality is conditional, never proof that no unseen branch exists.'},null,2));
module.exports={evaluate,verify,disjointCount};