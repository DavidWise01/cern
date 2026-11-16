'use strict';
const assert=require('node:assert/strict'),crypto=require('node:crypto');
const sha=x=>crypto.createHash('sha256').update(JSON.stringify(x)).digest('hex');
const kp=()=>crypto.generateKeyPairSync('ed25519');
const keys=Object.fromEntries(['A','B','C','D','A-mirror'].map(x=>[x,kp()]));
const roots={A:['R1'],B:['R2'],C:['R3'],D:['R4'],'A-mirror':['R1']};
const receipt=sha({campaign:'H070-FINAL',model:'m4',claim:'one-use',score:.61});
const gen={seq:1,parent:null,branch:'001',receipt:null};
const c2={seq:2,parent:sha(gen),branch:'002',receipt};
const c3a={seq:3,parent:sha(c2),branch:'003A',receipt};
const c3b={seq:3,parent:sha(c2),branch:'003B',receipt};
const graph=new Map([gen,c2,c3a,c3b].map(c=>[sha(c),c]));
const sign=(w,cp)=>{const payload={witness:w,checkpoint:sha(cp),receipt:cp.receipt};return{payload,signature:crypto.sign(null,Buffer.from(JSON.stringify(payload)),keys[w].privateKey).toString('base64')}};
function lineage(head,store=graph){const seen=new Set();let h=head,last=Infinity;while(h){if(seen.has(h))throw Error('cycle');seen.add(h);const c=store.get(h);if(!c||sha(c)!==h)throw Error('missing-or-corrupt-parent');if(c.seq>=last)throw Error('nonmonotone-height');last=c.seq;h=c.parent;}return [...seen]}
function evaluate(packets,store=graph){
 const valid=[],invalid=[];
 for(const p of packets){const w=p?.payload?.witness,c=store.get(p?.payload?.checkpoint);
  if(!keys[w]||!roots[w]||!c||p.payload.receipt!==c.receipt){invalid.push(w||'?');continue}
  if(!crypto.verify(null,Buffer.from(JSON.stringify(p.payload)),keys[w].publicKey,Buffer.from(p.signature||'','base64'))){invalid.push(w);continue}
  try{lineage(p.payload.checkpoint,store)}catch{invalid.push(w);continue}
  valid.push(p);
 }
 const forks=new Map(),sameSigner=new Map();
 for(const p of valid){const c=store.get(p.payload.checkpoint),key=c.seq+':'+c.parent;if(!forks.has(key))forks.set(key,new Set());forks.get(key).add(p.payload.checkpoint);
  const w=p.payload.witness;if(!sameSigner.has(w))sameSigner.set(w,new Set());sameSigner.get(w).add(p.payload.checkpoint);}
 const conflict=[...forks.values()].some(x=>x.size>1);
 const equivocators=[...sameSigner].filter(([,v])=>v.size>1).map(([w])=>w);
 if(conflict||equivocators.length)return{state:'FORK_FREEZE',equivocators,seen:valid.length,invalid};
 const ids=[...new Set(valid.map(p=>p.payload.checkpoint))];let max=0;
 for(const cp of ids){const witnesses=[...new Set(valid.filter(p=>p.payload.checkpoint===cp).map(p=>p.payload.witness))];
  function search(i,used,n){if(i===witnesses.length){max=Math.max(max,n);return}search(i+1,used,n);const rr=roots[witnesses[i]];if(rr.every(r=>!used.has(r)))search(i+1,new Set([...used,...rr]),n+1);}
  search(0,new Set(),0);
 }
 return{state:max>=3?'ANCHORED':'INSUFFICIENT_WITNESSES',independent:max,seen:valid.length,invalid};
}
const A=sign('A',c3a),B=sign('B',c3a),C=sign('C',c3a);
assert.equal(evaluate([A,B,C]).state,'ANCHORED');
assert.equal(evaluate([A,C]).state,'INSUFFICIENT_WITNESSES');
assert.equal(evaluate([A,C,B]).state,'ANCHORED');
assert.equal(evaluate([A,C,sign('A-mirror',c3a)]).independent,2);
assert.equal(evaluate([A,C,sign('B',c3b)]).state,'FORK_FREEZE');
assert.equal(evaluate([A,B,C,sign('D',c3b)]).state,'FORK_FREEZE');
assert.deepEqual(evaluate([A,sign('A',c3b),B,C]).equivocators,['A']);
const forged=structuredClone(A);forged.payload.checkpoint=sha(c3b);assert.equal(evaluate([forged,B,C]).state,'INSUFFICIENT_WITNESSES');
const bad={seq:4,parent:'no-such-parent',branch:'004',receipt},badGraph=new Map(graph);badGraph.set(sha(bad),bad);assert.throws(()=>lineage(sha(bad),badGraph),/missing-or-corrupt-parent/);
const cycle={seq:4,parent:sha(c3a),branch:'004',receipt},corrupted=new Map(graph);corrupted.set(sha(cycle),{...cycle,parent:sha(cycle)});assert.throws(()=>lineage(sha(cycle),corrupted),/missing-or-corrupt-parent/);
assert.notEqual(sha(c3a),sha(c3b));assert.equal(c3a.parent,c3b.parent);assert.equal(c3a.receipt,c3b.receipt);
const journal=[];function append(kind,data){const index=journal.length,prev=index?journal.at(-1).digest:'0'.repeat(64),item={index,prev,kind,data};journal.push({...item,digest:sha(item)})}
function verified(log){let prev='0'.repeat(64);return log.every((x,i)=>{const ok=x.index===i&&x.prev===prev&&x.digest===sha({index:i,prev,kind:x.kind,data:x.data});prev=x.digest;return ok})}
append('H069_PARENT',{receipt});append('BRANCH_A',{hash:sha(c3a)});append('BRANCH_B',{hash:sha(c3b)});append('FORK_FREEZE',evaluate([A,C,sign('B',c3b)]));assert(verified(journal));const copy=structuredClone(journal);copy[2].data.hash='forged';assert(!verified(copy));
console.log(JSON.stringify({status:'PASS',receipt,branchA:sha(c3a),branchB:sha(c3b),delayed:'INSUFFICIENT -> ANCHORED',competing:'FORK_FREEZE',equivocation:'FORK_FREEZE',mirror:'COUNT ONCE',auditEntries:journal.length,limit:'Locally generated keys and declared ancestry; not a distributed consensus system.'},null,2));
module.exports={lineage,evaluate};