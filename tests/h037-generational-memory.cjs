'use strict';
const assert=require('node:assert/strict');
const axes=['R','G','B'],pairs=['RG','GR','RB','BR','GB','BG'],layers=['visible','ultraviolet','infrared'];
const ps=[[0,1,2],[0,2,1],[1,0,2],[1,2,0],[2,0,1],[2,1,0]];
const parity=p=>((p[0]>p[1])+(p[0]>p[2])+(p[1]>p[2]))%2?-1:1;
const rotations=[];for(const p of ps)for(const a of [-1,1])for(const b of [-1,1])for(const c of [-1,1])if(parity(p)*a*b*c===1)rotations.push({p,s:[a,b,c]});
const I={p:[0,1,2],s:[1,1,1]};
const compose=(a,b)=>({p:b.p.map(i=>a.p[i]),s:b.p.map((j,i)=>b.s[i]*a.s[j])});
const inverse=r=>({p:[0,1,2].map(j=>r.p.indexOf(j)),s:[0,1,2].map(j=>r.s[r.p.indexOf(j)])});
const act=(v,r)=>r.p.map((j,i)=>r.s[i]*v[j]||0);
const remap=(w,r)=>w.split('').map(ch=>axes[r.p.indexOf(axes.indexOf(ch))]).join('');
const tokens=layers.flatMap(layer=>axes.flatMap(primary=>pairs.flatMap(pair=>[-1,1].flatMap(x=>[-1,1].flatMap(y=>[-1,1].map(z=>({layer,primary,pair,octant:[x,y,z],rank:40319})))))));
const apply=(t,r)=>({...t,primary:remap(t.primary,r),pair:remap(t.pair,r),octant:act(t.octant,r)});
function check(path){let r=I;for(const k of path)r=compose(r,rotations[k]);const back=inverse(r);
assert.deepEqual(compose(r,back),I);assert.deepEqual(compose(back,r),I);assert.deepEqual(act([0,0,0],r),[0,0,0]);
for(const t of tokens){let cur=t;for(const k of path)cur=apply(cur,rotations[k]);assert.deepEqual(cur,apply(t,r));assert.deepEqual(apply(cur,back),t);}
return JSON.stringify(r);}
assert.equal(rotations.length,24);
let checked=0,collision=null;const seen=new Map();
for(let a=0;a<24;a++)for(let b=0;b<24;b++)for(let c=0;c<24;c++){const path=[a,b,c],key=check(path);checked++;if(!collision&&seen.has(key)&&seen.get(key).join()!=path.join())collision={first:seen.get(key),second:path,final_rotation:key};if(!seen.has(key))seen.set(key,path);}
assert.equal(checked,13824);assert.equal(seen.size,24);assert(collision);
for(let n=1;n<=12;n++)check(Array.from({length:n},(_,i)=>(i*7+n*11)%24));
console.log(JSON.stringify({status:'PASS',sequences:checked,address_checks:checked*tokens.length,unique_terminal_rotations:seen.size,collision,sampled_lengths:'1..12',rule:'Same final geometry is not necessarily same ordered ancestry.'},null,2));
