'use strict';
const assert=require('node:assert/strict'),crypto=require('node:crypto');
const cutoff='2026-01-10T00:00:00Z';
const day=d=>`2026-01-${String(d).padStart(2,'0')}T00:00:00Z`;
const rows=[
{id:'train-clean',source:'SRC-1',eventAt:day(2),availableAt:day(3),content:'clean training',parents:[]},
{id:'train-parent',source:'SRC-2',eventAt:day(4),availableAt:day(5),content:'parent text',parents:[]},
{id:'future-transform',source:'DERIVED',eventAt:day(4),availableAt:day(12),content:'altered parent text',parents:['train-parent']},
{id:'train-copy-source',source:'SRC-3',eventAt:day(5),availableAt:day(6),content:'verbatim sample',parents:[]},
{id:'future-copy',source:'SRC-DIFFERENT',eventAt:day(5),availableAt:day(14),content:'verbatim sample',parents:[]},
{id:'test-new',source:'SRC-4',eventAt:day(11),availableAt:day(12),content:'new evaluation case',parents:[]},
{id:'test-new2',source:'SRC-5',eventAt:day(12),availableAt:day(13),content:'second evaluation case',parents:[]},
{id:'late-report',source:'SRC-6',eventAt:day(1),availableAt:day(15),content:'old event, late report',parents:[]},
{id:'unknown-time',source:'SRC-7',eventAt:day(6),availableAt:null,content:'unknown publication',parents:[]}
];
function partition(data,cutoff){
 const byId=new Map();for(const r of data){if(byId.has(r.id))throw Error('duplicate sample ID');byId.set(r.id,r)}
 const visiting=new Set(),visited=new Set();function validate(id){if(visiting.has(id))throw Error('ancestry cycle');if(visited.has(id))return;const r=byId.get(id);if(!r)throw Error('missing ancestry parent');visiting.add(id);for(const p of r.parents){validate(p);const par=byId.get(p);if(r.availableAt&&par.availableAt&&r.availableAt<par.availableAt)throw Error('child available before parent');}visiting.delete(id);visited.add(id)}
 for(const r of data)validate(r.id);
 const rootsCache=new Map();function roots(id){if(rootsCache.has(id))return rootsCache.get(id);const r=byId.get(id),out=new Set(r.parents.length?[]:[r.source]);for(const p of r.parents)for(const x of roots(p))out.add(x);rootsCache.set(id,out);return out}
 const uf=new Map(data.map(x=>[x.id,x.id]));function find(x){while(uf.get(x)!==x)x=uf.get(x);return x}function join(a,b){a=find(a);b=find(b);if(a!==b)uf.set(b,a)}
 for(let i=0;i<data.length;i++)for(let j=i+1;j<data.length;j++){const a=data[i],b=data[j];if((a.content!==null&&b.content!==null&&a.content===b.content)||[...roots(a.id)].some(x=>roots(b.id).has(x)))join(a.id,b.id)}
 const groups=new Map();for(const r of data){const k=find(r.id);if(!groups.has(k))groups.set(k,[]);groups.get(k).push(r)}
 const outcome={train:[],test:[],quarantine:[],reasons:{},groups:[]};
 for(const group of groups.values()){
   const unknown=group.some(r=>!r.availableAt||!r.source||!r.eventAt);
   const before=group.some(r=>r.availableAt&&r.availableAt<=cutoff);
   const after=group.some(r=>r.availableAt&&r.availableAt>cutoff);
   const kind=unknown?'quarantine':before&&after?'quarantine':before?'train':'test';
   const reason=unknown?'unknown metadata':before&&after?'cross-cutoff ancestry':'none';
   outcome.groups.push({ids:group.map(r=>r.id).sort(),kind,reason});
   for(const r of group){outcome[kind].push(r.id);if(reason!=='none')outcome.reasons[r.id]=reason;}
 }
 for(const kind of ['train','test','quarantine'])outcome[kind].sort();
 return outcome;
}
const result=partition(rows,cutoff);
assert.deepEqual(result.train,['train-clean']);
assert.deepEqual(result.test,['late-report','test-new','test-new2']);
assert.deepEqual(result.quarantine,['future-copy','future-transform','train-copy-source','train-parent','unknown-time']);
assert.equal(result.reasons['future-transform'],'cross-cutoff ancestry');
assert.equal(result.reasons['future-copy'],'cross-cutoff ancestry');
assert.equal(result.reasons['unknown-time'],'unknown metadata');
assert(result.test.includes('late-report'));
assert.throws(()=>partition([...rows,{...rows[0]}],cutoff),/duplicate sample ID/);
assert.throws(()=>partition([...rows,{id:'bad',source:'x',eventAt:day(2),availableAt:day(3),content:'z',parents:['missing']}],cutoff),/missing ancestry parent/);
assert.throws(()=>partition([...rows,{id:'cyc-a',source:'x',eventAt:day(2),availableAt:day(3),content:'z',parents:['cyc-b']},{id:'cyc-b',source:'x',eventAt:day(2),availableAt:day(3),content:'zz',parents:['cyc-a']}],cutoff),/ancestry cycle/);
assert.throws(()=>partition([...rows,{id:'impossible',source:'x',eventAt:day(2),availableAt:day(3),content:'xx',parents:['test-new']}],cutoff),/child available before parent/);
const h=o=>crypto.createHash('sha256').update(JSON.stringify(o)).digest('hex'),ledger=[];
function append(type,data){const seq=ledger.length,previous=seq?ledger.at(-1).hash:'0'.repeat(64),entry={seq,previous,type,data};ledger.push({...entry,hash:h(entry)})}
function verify(log){let previous='0'.repeat(64);return log.every((e,i)=>{const ok=e.seq===i&&e.previous===previous&&e.hash===h({seq:i,previous,type:e.type,data:e.data});previous=e.hash;return ok})}
append('H063_PARENT',{rule:'known source groups never split'});append('CUTOFF',{cutoff});append('GROUP_CLASSIFICATION',result.groups);append('CERTIFICATE',{train:result.train,test:result.test,quarantine:result.quarantine});assert(verify(ledger));const altered=structuredClone(ledger);altered[3].data.quarantine=[];assert(!verify(altered));
console.log(JSON.stringify({status:'PASS',cutoff,records:rows.length,train:result.train,test:result.test,quarantine:result.quarantine,group_count:result.groups.length,ledger_entries:ledger.length,rule:'Zero known cross-cutoff ancestry, old events arriving late belong to post-cutoff information.'},null,2));
module.exports={partition,rows,cutoff};
