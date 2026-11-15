'use strict';
const assert=require('node:assert/strict');
// H046: immutable source ancestry DAG; distinguish direct parents from ultimate roots.
class Ancestry {
 constructor(){this.nodes=new Map();}
 add(id,parents=[]){if(this.nodes.has(id))throw Error('immutable-node');if(!Array.isArray(parents)||new Set(parents).size!==parents.length)throw Error('duplicate-parent');this.nodes.set(id,Object.freeze({id,parents:Object.freeze([...parents])}));}
 roots(id,visiting=new Set(),memo=new Map()){
  if(memo.has(id))return new Set(memo.get(id));
  if(visiting.has(id))throw Error('cycle');
  const n=this.nodes.get(id);if(!n)throw Error('unknown-ancestor:'+id);
  visiting.add(id);
  const out=new Set();
  if(n.parents.length===0)out.add(id);
  else for(const p of n.parents)for(const r of this.roots(p,visiting,memo))out.add(r);
  visiting.delete(id);memo.set(id,[...out]);return out;
 }
 disjointWitnesses(ids){
  const seen=new Set(),records=[];
  for(const id of ids){const roots=[...this.roots(id)].sort();const overlaps=roots.filter(x=>seen.has(x));records.push({id,roots,overlaps,independent:overlaps.length===0});for(const x of roots)seen.add(x);}
  return {allIndependent:records.every(r=>r.independent),records,distinctRoots:seen.size};
 }
 validateAcyclic(){for(const id of this.nodes.keys())this.roots(id);return true;}
}
const a=new Ancestry();
for(const r of ['sourceX','sourceY','sourceZ','sourceQ'])a.add(r);
a.add('mirrorGit',['sourceX']);a.add('mirrorWeb',['sourceX']);
a.add('branchA',['mirrorGit']);a.add('branchB',['mirrorWeb']);
a.add('lineageY',['sourceY']);a.add('lineageZ',['sourceZ']);
a.add('mergeXY',['branchA','lineageY']);
a.add('w1',['branchA']);a.add('w2',['branchB']);a.add('w3',['lineageZ']);
a.add('wy',['lineageY']);a.add('wmerge',['mergeXY']);
assert.deepEqual([...a.roots('w1')],['sourceX']);
assert.deepEqual([...a.roots('w2')],['sourceX']);
assert.deepEqual([...a.roots('wmerge')].sort(),['sourceX','sourceY']);
const shared=a.disjointWitnesses(['w1','w2','w3']);
assert.equal(shared.allIndependent,false);assert.deepEqual(shared.records[1].overlaps,['sourceX']);
const independent=a.disjointWitnesses(['w1','wy','w3']);
assert.equal(independent.allIndependent,true);assert.equal(independent.distinctRoots,3);
const multi=a.disjointWitnesses(['wmerge','w2','w3']);
assert.equal(multi.allIndependent,false);
assert.throws(()=>a.add('w1',['sourceQ']),/immutable-node/);
assert.throws(()=>a.roots('missing'),/unknown-ancestor/);
const cyclic=new Ancestry();cyclic.add('c1',['c2']);cyclic.add('c2',['c1']);assert.throws(()=>cyclic.validateAcyclic(),/cycle/);
const unknown=new Ancestry();unknown.add('w',['missing']);assert.throws(()=>unknown.validateAcyclic(),/unknown-ancestor/);
assert(a.validateAcyclic());
console.log(JSON.stringify({status:'PASS',cases:9,shared_grandparent:'detected',independent_roots:'3',multi_parent_overlap:'detected',cycle:'rejected',unknown:'rejected',mutation:'rejected',scope:'Declared graph is internally checked; external authenticity of root claims is not established.'},null,2));
