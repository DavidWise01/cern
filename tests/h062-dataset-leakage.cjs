'use strict';
const assert=require('node:assert/strict');
const crypto=require('node:crypto');
const digest=x=>crypto.createHash('sha256').update(JSON.stringify(x)).digest('hex');
// Synthetic study; hashes are exact-content identifiers, not semantic similarity.
const samples=[
 {id:'x-a',dataset:'X',source:'sA',content:'alpha',parents:[]},
 {id:'x-b',dataset:'X',source:'sB',content:'beta',parents:[]},
 {id:'x-c',dataset:'X',source:'sC',content:'gamma',parents:[]},
 {id:'y-b',dataset:'Y',source:'sB',content:'beta',parents:[]},
 {id:'y-crop',dataset:'Y',source:'crop-C',content:'gamma cropped',parents:['x-c']},
 {id:'y-new',dataset:'Y',source:'sY',content:'delta',parents:[]},
 {id:'z-new',dataset:'Z',source:'sZ',content:'epsilon',parents:[]},
 {id:'z-new2',dataset:'Z',source:'sZ2',content:'zeta',parents:[]},
 {id:'q-unknown',dataset:'Q',source:null,content:null,parents:[]},
];
function roots(index,id,trail=new Set()){
 if(trail.has(id))throw Error('cycle detected');
 const s=index.get(id);if(!s)throw Error('unknown parent '+id);
 if(!s.parents.length)return new Set(s.source?[s.source]:[]);
 const t=new Set(trail);t.add(id);const out=new Set();
 for(const p of s.parents)for(const r of roots(index,p,t))out.add(r);
 return out;
}
function audit(all,a,b){
 const index=new Map(all.map(s=>[s.id,s]));
 assert.equal(index.size,all.length,'unique IDs');
 const A=all.filter(s=>s.dataset===a),B=all.filter(s=>s.dataset===b);
 const sameIds=A.flatMap(x=>B.filter(y=>x.id===y.id).map(y=>[x.id,y.id]));
 const exact=A.flatMap(x=>B.filter(y=>x.content!==null&&y.content!==null&&digest(x.content)===digest(y.content)).map(y=>[x.id,y.id]));
 const lineage=A.flatMap(x=>B.filter(y=>{const rx=roots(index,x.id),ry=roots(index,y.id);return [...rx].some(v=>ry.has(v));}).map(y=>[x.id,y.id]));
 const unknown=[...A,...B].filter(x=>!x.source&&!x.parents.length).map(x=>x.id);
 return {sameIds,exact,lineage,unknown,independent:!sameIds.length&&!exact.length&&!lineage.length&&!unknown.length};
}
const xy=audit(samples,'X','Y'),xz=audit(samples,'X','Z'),xq=audit(samples,'X','Q');
assert.equal(xy.exact.length,1);assert.deepEqual(xy.exact[0],['x-b','y-b']);
assert.equal(xy.lineage.length,2);assert.deepEqual(xy.lineage,[['x-b','y-b'],['x-c','y-crop']]);
assert.equal(xy.independent,false);assert.equal(xz.independent,true);
assert.equal(xq.independent,false);assert.deepEqual(xq.unknown,['q-unknown']);
// A globally duplicated sample ID is rejected before cross-dataset comparison.
assert.throws(()=>audit([...samples,{id:'x-a',dataset:'Y',source:'sA',content:'alpha',parents:[]}],'X','Y'),/unique IDs/);
const broken=[...samples,{id:'bad',dataset:'R',source:'bad',content:'bad',parents:['missing']}];
assert.throws(()=>audit(broken,'R','X'),/unknown parent/);
const cyclic=[...samples,{id:'cycle-a',dataset:'R',source:'a',content:'a',parents:['cycle-b']},{id:'cycle-b',dataset:'R',source:'b',content:'b',parents:['cycle-a']}];
assert.throws(()=>audit(cyclic,'R','X'),/cycle detected/);
// Exact-content duplicates with different sources are still duplicates.
const exactDifferentSources=[...samples,{id:'z-duplicate',dataset:'Z',source:'independently-claimed',content:'alpha',parents:[]}];
assert.equal(audit(exactDifferentSources,'X','Z').exact.length,1);
const journal=[];function append(kind,payload){const seq=journal.length,prev=seq?journal.at(-1).hash:'0'.repeat(64),entry={seq,prev,kind,payload};journal.push({...entry,hash:digest(entry)});}
function valid(log){let prev='0'.repeat(64);return log.every((r,i)=>{const ok=r.seq===i&&r.prev===prev&&r.hash===digest({seq:i,prev,kind:r.kind,payload:r.payload});prev=r.hash;return ok;});}
append('H061_PRIOR',{declared:'datasets X and Y separate'});append('EXACT_DUPLICATE',{pair:xy.exact[0]});append('TRANSFORM_ANCESTRY',{pair:['x-c','y-crop']});append('RECLASSIFY',{X_Y:'NOT_INDEPENDENT',X_Z:'INDEPENDENT_WITHIN_DECLARED_MODEL',X_Q:'UNKNOWN'});assert(valid(journal));const tampered=structuredClone(journal);tampered[0].payload.declared='datasets same';assert(!valid(tampered));
console.log(JSON.stringify({status:'PASS',sampleRecords:samples.length,xy,xz,xq,duplicateIdRejected:true,cycleRejected:true,unknownParentRejected:true,exactContentEvenWithDifferentRoots:true,appendOnlyRecords:journal.length,scope:'Synthetic source mapping and exact content only; no fuzzy similarity proof'},null,2));
