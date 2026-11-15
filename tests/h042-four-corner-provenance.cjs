'use strict';
// H042: Four-point cube-rotation error correction with independent ancestry gate.
const assert = require('node:assert/strict');
const permutations=[[0,1,2],[0,2,1],[1,0,2],[1,2,0],[2,0,1],[2,1,0]];
const parity=p=>((p[0]>p[1])+(p[0]>p[2])+(p[1]>p[2]))%2?-1:1;
const rotations=[];
for(const p of permutations)for(const a of [-1,1])for(const b of [-1,1])for(const c of [-1,1])
 if(parity(p)*a*b*c===1)rotations.push({p,s:[a,b,c]});
const corners=[];for(const x of [-1,1])for(const y of [-1,1])for(const z of [-1,1])corners.push([x,y,z]);
const encode=(source,r)=>source.map(v=>JSON.stringify(r.p.map((j,i)=>r.s[i]*v[j])));
const d=(a,b)=>a.reduce((n,x,i)=>n+(x!==b[i]),0);
const comb=(arr,k)=>{const out=[];const rec=(start,p)=>{if(p.length===k){out.push(p);return;}for(let i=start;i<arr.length;i++)rec(i+1,[...p,arr[i]])};rec(0,[]);return out};
const safeSets=comb(corners,4).filter(points=>!points.some((v,i)=>points.some((w,j)=>j>i&&v.every((x,k)=>x===-w[k]))));
assert.equal(safeSets.length,16);
let substitutions=0,accepted=0,rejected=0,ambiguous=0,provenanceDenied=0,unknownDenied=0;
const decode=(codewords,received)=>{const matches=codewords.map((word,i)=>({i,errors:d(word,received)})).filter(x=>x.errors<=1);return matches.length===1?matches[0]:null};
const rootsFor=(caseType,bad)=>Array.from({length:4},(_,i)=>{
 if(caseType==='independent')return 'root'+i;
 if(caseType==='shared')return i===0||i===1?'root0':'root'+i;
 if(caseType==='allShared')return 'mirror';
 if(caseType==='unknown')return i===2?null:'root'+i;
 throw Error('case');
});
function ancestryGate(roots,excluded){
 const retained=roots.filter((_,i)=>i!==excluded);
 return retained.length>=3&&retained.every(Boolean)&&new Set(retained).size>=3;
}
for(const points of safeSets){
 const words=rotations.map(r=>encode(points,r));
 let md=4;for(let i=0;i<24;i++)for(let j=i+1;j<24;j++)md=Math.min(md,d(words[i],words[j]));
 assert.equal(md,3);
 for(let source=0;source<24;source++)for(let bad=0;bad<4;bad++)for(const alternative of corners){
  const replacement=JSON.stringify(alternative);if(replacement===words[source][bad])continue;
  const received=[...words[source]];received[bad]=replacement;
  const decoded=decode(words,received);
  assert(decoded);assert.equal(decoded.i,source);assert.equal(decoded.errors,1);
  // The decoder identifies rotation, but not which source was wrong merely
  // from geometry unless the differing coordinate is located.
  const incorrect=words[source].findIndex((value,i)=>value!==received[i]);
  assert.equal(incorrect,bad);
  substitutions++;
  if(ancestryGate(rootsFor('independent',bad),bad))accepted++;else throw Error('independent gate failure');
  if(!ancestryGate(rootsFor('allShared',bad),bad))provenanceDenied++;else throw Error('shared-roots accepted');
 }
}
assert.equal(substitutions,10752);assert.equal(accepted,10752);assert.equal(provenanceDenied,10752);
// Two additional adversarial cases: known duplicate roots and unknown ancestry.
// The gate is evaluated after excluding the suspect slot.
const independent=['A','B','C','D'],duplicates=['A','A','C','D'],unknown=['A','B',null,'D'];
assert(ancestryGate(independent,3));
assert(!ancestryGate(duplicates,3));
assert(!ancestryGate(unknown,3));
assert(ancestryGate(duplicates,0)); // suspect duplicate removed: B slot's A, C and D remain independent
assert(!ancestryGate(unknown,0));
console.log(JSON.stringify({status:'PASS',safe_four_corner_sets:16,substitution_recovery_trials:substitutions,independent_root_acceptances:accepted,shared_root_denials:provenanceDenied,duplicate_denial:true,unknown_ancestry_denial:true,rule:'Accept only if unique rotation within one error AND three intact observations have distinct known root ancestry.'},null,2));
