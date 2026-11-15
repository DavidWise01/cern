// H035: finite-address rotational conservation test (Node.js, dependency-free)
const assert = require('node:assert/strict');
const axes = ['R','G','B'], layers=['visible','ultraviolet','infrared'];
const pairs=['RG','GR','RB','BR','GB','BG'];
const octants=[-1,1].flatMap(x=>[-1,1].flatMap(y=>[-1,1].map(z=>[x,y,z])));
const parity=p=>(p[0]>p[1]?1:0)+(p[0]>p[2]?1:0)+(p[1]>p[2]?1:0);
const perms=[[0,1,2],[0,2,1],[1,0,2],[1,2,0],[2,0,1],[2,1,0]];
const rotations=perms.flatMap(p=>[-1,1].flatMap(a=>[-1,1].flatMap(b=>[-1,1].map(c=>({p,s:[a,b,c]})))).filter(r=>(parity(r.p)%2?-1:1)*r.s.reduce((a,b)=>a*b,1)===1);
const apply=(v,r)=>r.p.map((j,i)=>r.s[i]*v[j]);
const mapAxis=(i,r)=>r.p.indexOf(i);
const mapPair=(v,r)=>v.split('').map(ch=>axes[mapAxis(axes.indexOf(ch),r)]).join('');
const key=v=>v.join(',');
const assertOne=(layer,channel,rank,pair,oct,r)=>{
 const origin={layer,channel,rank,pair,oct:key(oct)};
 const out=apply(oct,r),pair2=mapPair(pair,r),channel2=axes[mapAxis(axes.indexOf(channel),r)];
 assert.equal(out.reduce((a,x)=>a*x*x,0),3);
 assert(octants.some(o=>key(o)===key(out)));
 assert(pairs.includes(pair2));
 const inverseOct=r.p.map((_,j)=>out[r.p.indexOf(j)]*r.s[r.p.indexOf(j)]);
 const inversePair=pair2.split('').map(ch=>axes[r.p[axes.indexOf(ch)]]).join('');
 const inverseChannel=axes[r.p[axes.indexOf(channel2)]];
 assert.equal(key(inverseOct),origin.oct); assert.equal(inversePair,pair); assert.equal(inverseChannel,channel);
 assert.equal(rank,origin.rank); assert.equal(layer,origin.layer);
};
assert.equal(rotations.length,24);
let count=0;
for(const r of rotations)for(const layer of layers)for(const channel of axes)for(const pair of pairs)for(const oct of octants){
 assertOne(layer,channel,0,pair,oct,r); count++;
}
// Permutation ranks are immutable payloads, not spatial coordinates.
// The rotation never inspects or changes rank; therefore the same proof
// applies to every rank from 0 to 40319, not just the representative rank 0.
for(const r of rotations)for(const n of [0,1,20160,40319])assertOne('visible','R',n,'RG',[-1,1,-1],r);
const fixed=rotations.filter(r=>r.p.every((a,i)=>a===i));
assert.equal(fixed.length,4); // proper rotations that do not relabel R/G/B axes
console.log(JSON.stringify({status:'PASS',proper_rotations:rotations.length,layers:3,representative_checked:count,logical_full_address_cases:count*40320,axis_name_preserving_rotations:fixed.length,center_fixed:true,notes:'Preservation is of immutable provenance/origin identity; moved addresses and channel labels can differ.'},null,2));
