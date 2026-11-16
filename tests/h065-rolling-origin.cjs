'use strict';
const assert=require('node:assert/strict'),crypto=require('node:crypto');
const hash=x=>crypto.createHash('sha256').update(JSON.stringify(x)).digest('hex');
const day=n=>`2026-01-${String(n).padStart(2,'0')}T00:00:00Z`;
const cutoffDays=[7,14,21],horizonDays=7;
const r=(id,d,source,content,parents=[],event=d)=>({id,availableAt:d===null?null:day(d),eventAt:event===null?null:day(event),source,content,parents});
const records=[
r('a',3,'A','alpha'),r('b',5,'B','beta'),r('c',9,'C','gamma'),r('d',12,'D','delta'),
r('b-copy',12,'B-MIRROR','beta'),r('e',16,'E','epsilon'),r('late',17,'L','late-known',[],1),r('f',20,'F','zeta'),
r('g',24,'G','eta'),r('h',26,'H','theta'),r('c-transform',24,'C-DERIVED','gamma cropped',['c']),r('unknown',null,null,'opaque')];
function fold(samples,originDay){
const byId=new Map();for(const x of samples){if(byId.has(x.id))throw Error('duplicate ID');byId.set(x.id,x)}
const memo=new Map(),active=new Set();
function ancestors(id){if(active.has(id))throw Error('cycle');if(memo.has(id))return memo.get(id);const x=byId.get(id);if(!x)throw Error('missing parent');active.add(id);const out=new Set(x.parents.length?[]:[x.source]);for(const parent of x.parents){const p=byId.get(parent);if(!p)throw Error('missing parent');if(x.availableAt&&p.availableAt&&x.availableAt<p.availableAt)throw Error('causality violation');for(const a of ancestors(parent))out.add(a)}active.delete(id);memo.set(id,out);return out}
for(const x of samples)ancestors(x.id);
const cutoff=day(originDay),end=day(originDay+horizonDays),known=samples.filter(x=>x.source&&x.availableAt&&x.eventAt),quarantine=samples.filter(x=>!x.source||!x.availableAt||!x.eventAt).map(x=>x.id);
const uf=new Map(known.map(x=>[x.id,x.id]));const find=id=>{while(uf.get(id)!==id)id=uf.get(id);return id};const unite=(a,b)=>{a=find(a);b=find(b);if(a!==b)uf.set(b,a)};
for(let i=0;i<known.length;i++)for(let j=i+1;j<known.length;j++){const a=known[i],b=known[j];if([...ancestors(a.id)].some(root=>ancestors(b.id).has(root))||(a.content!==null&&a.content===b.content))unite(a.id,b.id)}
const components=new Map();for(const x of known){const key=find(x.id);if(!components.has(key))components.set(key,[]);components.get(key).push(x)}
const result={cutoff,end,train:[],test:[],quarantine:[...quarantine],future:[],crossing:[]};
for(const group of components.values()){
const pre=group.filter(x=>x.availableAt<=cutoff),test=group.filter(x=>x.availableAt>cutoff&&x.availableAt<=end),future=group.filter(x=>x.availableAt>end);
if(pre.length&&test.length){result.quarantine.push(...pre.map(x=>x.id),...test.map(x=>x.id));result.crossing.push(group.map(x=>x.id).sort());result.future.push(...future.map(x=>x.id));}
else{result.train.push(...pre.map(x=>x.id));result.test.push(...test.map(x=>x.id));result.future.push(...future.map(x=>x.id));}}
for(const key of ['train','test','quarantine','future'])result[key].sort();result.crossing.sort((a,b)=>a.join().localeCompare(b.join()));
for(const id of result.train)assert(byId.get(id).availableAt<=cutoff);
for(const id of result.test)assert(byId.get(id).availableAt>cutoff&&byId.get(id).availableAt<=end);
assert.equal(new Set([...result.train,...result.test,...result.quarantine,...result.future]).size,samples.length);
const groups=new Map();for(const x of known){const key=find(x.id);if(!groups.has(key))groups.set(key,new Set());groups.get(key).add(result.train.includes(x.id)?'train':result.test.includes(x.id)?'test':result.future.includes(x.id)?'future':'quarantine')}
for(const slots of groups.values())assert(!(slots.has('train')&&slots.has('test')));
return result;}
const folds=cutoffDays.map(c=>fold(records,c));
assert.deepEqual(folds[0].train,['a']);assert.deepEqual(folds[0].test,['c','d']);assert.deepEqual(folds[0].crossing,[['b','b-copy']]);
assert.deepEqual(folds[1].train,['a','b','b-copy','c','d']);assert.deepEqual(folds[1].test,['e','f','late']);
assert.deepEqual(folds[2].train,['a','b','b-copy','d','e','f','late']);assert.deepEqual(folds[2].test,['g','h']);assert.deepEqual(folds[2].crossing,[['c','c-transform']]);assert(folds.every(f=>f.quarantine.includes('unknown')));
const futureOnly=r('new-after-window',28,'NEW','new content');
for(const field of ['train','test','quarantine'])assert.deepEqual(fold([...records,futureOnly],7)[field],folds[0][field]);
assert.throws(()=>fold([...records,records[0]],7),/duplicate ID/);
assert.throws(()=>fold([...records,r('bad',10,'Q','bad',['missing'])],7),/missing parent/);
assert.throws(()=>fold([...records,r('cycle1',10,'Q','one',['cycle2']),r('cycle2',10,'Q','two',['cycle1'])],7),/cycle/);
assert.throws(()=>fold([...records,r('early-child',4,'Q','early',['e'])],7),/causality violation/);
const ledger=[];function append(kind,payload){const seq=ledger.length,previous=seq?ledger.at(-1).digest:'0'.repeat(64),entry={seq,previous,kind,payload};ledger.push({...entry,digest:hash(entry)})}
function verified(log){let previous='0'.repeat(64);return log.every((e,i)=>{const ok=e.seq===i&&e.previous===previous&&e.digest===hash({seq:i,previous,kind:e.kind,payload:e.payload});previous=e.digest;return ok})}
append('H064_PARENT',{invariant:'temporal and ancestry boundaries'});for(const f of folds)append('FROZEN_FOLD',f);assert(verified(ledger));const tampered=structuredClone(ledger);tampered[1].payload.train=[];assert(!verified(tampered));
console.log(JSON.stringify({status:'PASS',sample_count:records.length,folds,ledger_entries:ledger.length,future_insertion_invariance:true},null,2));module.exports={fold,records,cutoffDays};