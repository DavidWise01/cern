'use strict';
const assert=require('node:assert/strict');
const ps=[[0,1,2],[0,2,1],[1,0,2],[1,2,0],[2,0,1],[2,1,0]];
const parity=p=>((p[0]>p[1])+(p[0]>p[2])+(p[1]>p[2]))%2?-1:1;
const rotations=[];for(const p of ps)for(const a of [-1,1])for(const b of [-1,1])for(const c of [-1,1])if(parity(p)*a*b*c===1)rotations.push({p,s:[a,b,c]});
const identity={p:[0,1,2],s:[1,1,1]};
const eq=(a,b)=>JSON.stringify(a)===JSON.stringify(b);
const compose=(a,b)=>({p:b.p.map(i=>a.p[i]),s:b.p.map((j,i)=>b.s[i]*a.s[j])});
const act=(v,r)=>r.p.map((j,i)=>r.s[i]*v[j]);
const axes='RGB',pairs=['RG','GR','RB','BR','GB','BG'];
const mapLabel=(word,r)=>word.split('').map(ch=>axes[r.p.indexOf(axes.indexOf(ch))]).join('');
const order=r=>{let v=identity;for(let n=1;n<=24;n++){v=compose(v,r);if(eq(v,identity))return n;}throw Error('missing return')};
const octants=[];for(const x of [-1,1])for(const y of [-1,1])for(const z of [-1,1])octants.push([x,y,z]);
const counts={1:0,2:0,3:0,4:0},orbitCounts={1:0,2:0,3:0,4:0};let cases=0,steps=0;
for(const r of rotations){
 const p=order(r);assert(p in counts);counts[p]++;
 for(const start of octants){let current=start;let period=0;do{current=act(current,r);period++}while(!eq(current,start)&&period<=4);assert(period<=4);assert.equal(p%period,0);orbitCounts[period]++;}
 for(const layer of ['visible','ultraviolet','infrared'])for(const rank of [0,1,20160,40319])for(const primary of axes)for(const pair of pairs)for(const octant of octants){
  let o=octant,c=primary,d=pair;
  for(let k=0;k<p;k++){o=act(o,r);c=mapLabel(c,r);d=mapLabel(d,r);steps++;}
  assert.deepEqual(o,octant);assert.equal(c,primary);assert.equal(d,pair);assert.deepEqual(act([0,0,0],r),[0,0,0]);cases++;
 }
}
assert.deepEqual(counts,{1:1,2:9,3:8,4:6});
assert.deepEqual(orbitCounts,{1:24,2:72,3:48,4:48});
assert.equal(cases,41472);assert.equal(steps,115776);
console.log(JSON.stringify({status:'PASS',rotations:rotations.length,order_distribution:counts,octant_orbit_distribution:orbitCounts,address_cases:cases,stepwise_updates:steps,note:'Representative ranks; all ranks follow by immutable-rank definition. This does not demonstrate empirical physics.'},null,2));
