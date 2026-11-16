'use strict';
const assert=require('node:assert/strict'),crypto=require('node:crypto');
const samples=[['x-a','A','alpha',[]],['x-b','B','beta',[]],['x-c','C','gamma',[]],['y-b','B','beta',[]],['y-crop','CROP','crop(gamma)',['x-c']],['y-new','Y','delta',[]],['z-new','Z','epsilon',[]],['z-new2','Z2','zeta',[]],['q-unknown',null,'opaque',[]]].map(([id,source,content,parents])=>({id,source,content,parents}));
const hash=x=>crypto.createHash('sha256').update(JSON.stringify(x)).digest('hex');
function partition(records){
 const byId=new Map();for(const s of records){if(byId.has(s.id))throw Error('duplicate ID');byId.set(s.id,s)}
 const cache=new Map();
 function ancestry(id,trace=new Set()){if(trace.has(id))throw Error('cycle');if(!byId.has(id))throw Error('missing parent');if(cache.has(id))return cache.get(id);const s=byId.get(id),next=new Set(trace);next.add(id),out=new Set(s.source?[s.source]:[]);for(const parent of s.parents)for(const root of ancestry(parent,next))out.add(root);cache.set(id,out);return out}
 const quarantine=records.filter(s=>!s.source&&!s.parents.length).map(s=>s.id),known=records.filter(s=>!quarantine.includes(s.id));
 const parents=new Map(known.map(s=>[s.id,s.id]));
 function find(id){while(parents.get(id)!==id)id=parents.get(id);return id}
 function merge(a,b){a=find(a);b=find(b);if(a!==b)parents.set(b,a)}
 for(let i=0;i<known.length;i++)for(let j=i+1;j<known.length;j++){const a=known[i],b=known[j];if([...ancestry(a.id)].some(r=>ancestry(b.id).has(r))||hash(a.content)===hash(b.content))merge(a.id,b.id)}
 const grouped=new Map();for(const r of known){const key=find(r.id);if(!grouped.has(key))grouped.set(key,[]);grouped.get(key).push(r.id)}
 const groups=[...grouped.values()].map(x=>x.sort()).sort((a,b)=>a[0].localeCompare(b[0]));let best=null;
 for(let mask=1;mask<(1<<groups.length)-1;mask++){const train=groups.filter((_,i)=>mask&(1<<i)).flat().sort(),test=groups.filter((_,i)=>!(mask&(1<<i))).flat().sort(),delta=Math.abs(train.length-test.length),key=JSON.stringify(train);if(!best||delta<best.delta||(delta===best.delta&&key<best.key))best={train,test,delta,key}}
 if(!best)throw Error('not enough independent groups');
 for(const group of groups)assert(new Set(group.map(id=>best.train.includes(id)?'TRAIN':'TEST')).size===1);
 for(const a of best.train)for(const b of best.test){assert(![...ancestry(a)].some(root=>ancestry(b).has(root)));assert.notEqual(hash(byId.get(a).content),hash(byId.get(b).content))}
 return {groups,train:best.train,test:best.test,quarantine,balance_delta:best.delta};
}
const out=partition(samples);assert.equal(out.groups.length,6);assert.equal(out.balance_delta,0);assert.deepEqual(out.quarantine,['q-unknown']);assert(out.groups.some(g=>g.join(',')==='x-b,y-b'));assert(out.groups.some(g=>g.join(',')==='x-c,y-crop'));
assert(partition([...samples,{id:'new-copy',source:'fresh',content:'alpha',parents:[]}]).groups.some(g=>g.join(',')==='new-copy,x-a'));
assert.throws(()=>partition([...samples,{...samples[0]}]),/duplicate ID/);
assert.throws(()=>partition([...samples,{id:'bad',source:'P',content:'bad',parents:['missing']}]),/missing parent/);
assert.throws(()=>partition([...samples,{id:'a',source:'P',content:'a',parents:['b']},{id:'b',source:'Q',content:'b',parents:['a']}]),/cycle/);
assert.throws(()=>partition([{id:'one',source:'A',content:'x',parents:[]},{id:'two',source:'A',content:'y',parents:[]}]),/not enough independent groups/);
const log=[];function append(kind,data){const index=log.length,previous=index?log.at(-1).digest:'0'.repeat(64),e={index,previous,kind,data};log.push({...e,digest:hash(e)})}
function verify(records){let prev='0'.repeat(64);return records.every((r,i)=>{const valid=r.index===i&&r.previous===prev&&r.digest===hash({index:i,previous:prev,kind:r.kind,data:r.data});prev=r.digest;return valid})}
append('H062_PARENT',{known_overlap:true});append('GROUPS',out.groups);append('PARTITION',{train:out.train,test:out.test});append('QUARANTINE',out.quarantine);assert(verify(log));const fake=structuredClone(log);fake[2].data.train=[];assert(!verify(fake));
console.log(JSON.stringify({status:'PASS',input:9,groups:out.groups,train:out.train,test:out.test,quarantine:out.quarantine,balance_delta:out.balance_delta,append_only_records:log.length,limitation:'Only zero known crossover under declared ancestry and exact-content matches.'},null,2));
module.exports={partition};
