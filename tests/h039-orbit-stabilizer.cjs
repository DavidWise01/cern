'use strict';
const assert=require('node:assert/strict');
const axes='RGB',pairs=['RG','GR','RB','BR','GB','BG'],layers=['visible','ultraviolet','infrared'];
const perms=[[0,1,2],[0,2,1],[1,0,2],[1,2,0],[2,0,1],[2,1,0]];
const parity=p=>((p[0]>p[1])+(p[0]>p[2])+(p[1]>p[2]))%2?-1:1;
const rotations=[];
for(const p of perms)for(const a of [-1,1])for(const b of [-1,1])for(const c of [-1,1])
 if(parity(p)*a*b*c===1)rotations.push({p,s:[a,b,c]});
const oct=v=>JSON.stringify(v),apply=(v,r)=>r.p.map((j,i)=>r.s[i]*v[j]);
const label=(w,r)=>w.split('').map(ch=>axes[r.p.indexOf(axes.indexOf(ch))]).join('');
const corners=[];for(const x of [-1,1])for(const y of [-1,1])for(const z of [-1,1])corners.push([x,y,z]);
assert.equal(rotations.length,24);let oc=0,lc=0;
for(const v of corners){
 const orbit=new Set(rotations.map(r=>oct(apply(v,r))));
 const stabilizer=rotations.filter(r=>oct(apply(v,r))===oct(v));
 assert.equal(orbit.size,8);assert.equal(stabilizer.length,3);
 assert.equal(orbit.size*stabilizer.length,24);oc+=24;
 for(const spectrum of layers)for(const primary of axes)for(const secondary of pairs){
  const origin={spectrum,primary,secondary,octant:v,rank:40319};
  const results=rotations.map(r=>({spectrum,primary:label(primary,r),secondary:label(secondary,r),octant:apply(v,r),rank:40319}));
  const fullOrbit=new Set(results.map(oct));
  const fullStabilizer=results.filter(a=>oct(a)===oct(origin));
  assert.equal(fullOrbit.size,24);assert.equal(fullStabilizer.length,1);
  assert.equal(fullOrbit.size*fullStabilizer.length,24);lc+=24;
 }
}
console.log(JSON.stringify({status:'PASS',rotations:24,octants:8,octant_checks:oc,labeled_checks:lc,octant_orbit:8,octant_stabilizer:3,labeled_orbit:24,labeled_stabilizer:1,rank_coverage:'all 40320 ranks by rank independence'}));
