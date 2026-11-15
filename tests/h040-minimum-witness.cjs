'use strict';
// H040: Minimal geometric observations for proper cube rotation identification.
const assert=require('node:assert/strict');
const axes='RGB';
const permutations=[[0,1,2],[0,2,1],[1,0,2],[1,2,0],[2,0,1],[2,1,0]];
const parity=p=>((p[0]>p[1])+(p[0]>p[2])+(p[1]>p[2]))%2?-1:1;
const rotations=[];
for(const p of permutations)for(const a of [-1,1])for(const b of [-1,1])for(const c of [-1,1])
 if(parity(p)*a*b*c===1)rotations.push({p,s:[a,b,c]});
const corners=[];
for(const x of [-1,1])for(const y of [-1,1])for(const z of [-1,1])corners.push([x,y,z]);
const eq=(a,b)=>JSON.stringify(a)===JSON.stringify(b);
const apply=(v,r)=>r.p.map((j,i)=>r.s[i]*v[j]);
const label=(v,r)=>axes[r.p.indexOf(axes.indexOf(v))];
const stabilizer=observations=>rotations.filter(r=>observations.every(v=>eq(apply(v,r),v)));
const observedOrbit=observations=>new Set(rotations.map(r=>JSON.stringify(observations.map(v=>apply(v,r))))).size;
assert.equal(rotations.length,24);
const singleCounts={3:0},pair={opposite:0,nonopposite:0,oppositeStabilizer:3,nonoppositeStabilizer:1},triple={distinct:0,stabilizerOne:0};
for(const a of corners){
 assert.equal(stabilizer([a]).length,3);assert.equal(observedOrbit([a]),8);singleCounts[3]++;
 for(const b of corners){if(eq(a,b))continue;
  const opposite=b.every((v,i)=>v===-a[i]);
  const s=stabilizer([a,b]).length,o=observedOrbit([a,b]);
  assert.equal(s,opposite?3:1);assert.equal(o,opposite?8:24);
  pair[opposite?'opposite':'nonopposite']++;
  for(const c of corners){if(eq(c,a)||eq(c,b))continue;
   assert.equal(stabilizer([a,b,c]).length,1);
   assert.equal(observedOrbit([a,b,c]),24);
   triple.distinct++;triple.stabilizerOne++;
  }
 }
}
// A single corner + one *observed primary axis* is another minimal two-feature witness.
let oneCornerAxis=0;
for(const a of corners)for(const primary of axes){
 const mapped=new Set(rotations.map(r=>JSON.stringify([apply(a,r),label(primary,r)])));
 assert.equal(mapped.size,24);
 const s=rotations.filter(r=>eq(apply(a,r),a)&&label(primary,r)===primary);
 assert.equal(s.length,1);
 oneCornerAxis++;
}
assert.equal(pair.opposite,8);assert.equal(pair.nonopposite,48);
assert.equal(triple.distinct,336);assert.equal(oneCornerAxis,24);
console.log(JSON.stringify({status:'PASS',group:24,single_corner:{cases:8,stabilizer:3,orbit:8},ordered_distinct_pairs:{total:56,opposite:8,opposite_stabilizer:3,nonopposite:48,nonopposite_stabilizer:1},ordered_distinct_triples:{total:336,stabilizer:1},corner_plus_primary_axis:{cases:24,stabilizer:1},scope:'Exact discrete proper cube rotations; distinct observations must share a coherent frame and be traceable to separate geometric locations. Independent provenance is a different question.'},null,2));
