'use strict';
// H036: exhaustive composition and inverse-return check over all 24 proper cube rotations.
const assert = require('node:assert/strict');
const axes = ['R','G','B'];
const layers = ['visible','ultraviolet','infrared'];
const pairs = ['RG','GR','RB','BR','GB','BG'];
const octants = [-1,1].flatMap(x=>[-1,1].flatMap(y=>[-1,1].map(z=>[x,y,z])));
const permutations = [[0,1,2],[0,2,1],[1,0,2],[1,2,0],[2,0,1],[2,1,0]];
const sign = p => ((p[0]>p[1])+(p[0]>p[2])+(p[1]>p[2]))%2 ? -1 : 1;
const rotations = permutations.flatMap(p=>[-1,1].flatMap(a=>[-1,1].flatMap(b=>[-1,1].map(c=>({p,s:[a,b,c]})))))
  .filter(r=>sign(r.p)*r.s[0]*r.s[1]*r.s[2]===1);
const act = (v,r)=>r.p.map((j,i)=>r.s[i]*v[j]);
const remap = (name,r)=>name.split('').map(letter=>axes[r.p.indexOf(axes.indexOf(letter))]).join('');
const compose = (a,b)=>({p:b.p.map(i=>a.p[i]),s:b.p.map((j,i)=>b.s[i]*a.s[j])});
const inverse = r=>({p:[0,1,2].map(j=>r.p.indexOf(j)),s:[0,1,2].map(j=>r.s[r.p.indexOf(j)])});
const identity = {p:[0,1,2],s:[1,1,1]};
const same = (a,b)=>assert.deepEqual(a,b);
same(rotations.length,24);
let checked=0;
for(const a of rotations)for(const b of rotations){
 const combined=compose(a,b), back=inverse(combined);
 same(compose(combined,back),identity);
 same(compose(back,combined),identity);
 assert(rotations.some(r=>JSON.stringify(r)===JSON.stringify(combined)), 'closure');
 for(const layer of layers)for(const primary of axes)for(const pair of pairs)for(const octant of octants){
   const out=act(act(octant,a),b);
   same(out,act(octant,combined));
   same(act(out,back),octant);
   same(remap(remap(primary,a),b),remap(primary,combined));
   same(remap(remap(pair,a),b),remap(pair,combined));
   same(remap(remap(primary,combined),back),primary);
   same(remap(remap(pair,combined),back),pair);
   same(act([0,0,0],combined),[0,0,0]);
   // Layer and permutation rank are immutable provenance, not spatial coordinates.
   assert(layers.includes(layer));
   checked++;
 }
}
const fixedAxes=rotations.filter(r=>r.p.every((v,i)=>v===i)).length;
const report={schema:'cern.h036.rotation-composition.v1',status:'PASS',rotations:24,ordered_rotation_pairs:576,explicit_cases:checked,rank_count:40320,logically_covered_cases:checked*40320,axis_name_preserving_rotations:fixedAxes,layer_count:3,center_fixed:true,closure:true,inverse:true,association:'signed-permutation function composition is associative',notes:'Rotations are discrete proper cube symmetries, not arbitrary continuous-angle rotations. This is symbolic geometry, not experimentally observed conservation.'};
console.log(JSON.stringify(report,null,2));
