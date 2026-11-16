'use strict';
const assert=require('node:assert/strict'),crypto=require('node:crypto');
const hash=x=>crypto.createHash('sha256').update(JSON.stringify(x)).digest('hex');
class DAG {
 constructor(){this.nodes=new Map()}
 add(id,parents=[]){assert(!this.nodes.has(id),'immutable node');assert.equal(new Set(parents).size,parents.length);this.nodes.set(id,Object.freeze({id,parents:Object.freeze([...parents])}))}
 roots(id,visiting=new Set()){if(visiting.has(id))throw Error('cycle');const node=this.nodes.get(id);if(!node)throw Error('unknown ancestor '+id);if(!node.parents.length)return new Set([id]);visiting.add(id);const roots=new Set();for(const p of node.parents)for(const root of this.roots(p,visiting))roots.add(root);visiting.delete(id);return roots}
}
const d=new DAG();for(const r of ['DATA_X','DATA_Y','DATA_Z','DATA_W'])d.add(r);
d.add('FOLD_X1',['DATA_X']);d.add('FOLD_X2',['DATA_X']);d.add('CAL1',['FOLD_X1']);d.add('CAL2',['FOLD_X2']);d.add('CAL3',['DATA_X']);d.add('CAL4',['DATA_Y']);d.add('CAL5',['DATA_Z']);d.add('CAL6',['DATA_W']);d.add('CAL_MERGED',['CAL1','CAL4']);
const roots=id=>[...d.roots(id)].sort();
function independent(models){let best=[];function visit(i,chosen,seen){if(i===models.length){if(chosen.length>best.length)best=[...chosen];return}visit(i+1,chosen,seen);const r=roots(models[i]);if(r.every(x=>!seen.has(x)))visit(i+1,[...chosen,models[i]],new Set([...seen,...r]));}visit(0,[],new Set());return best}
assert.deepEqual(roots('CAL1'),['DATA_X']);assert.deepEqual(roots('CAL2'),['DATA_X']);assert.deepEqual(roots('CAL_MERGED'),['DATA_X','DATA_Y']);
assert.equal(independent(['CAL1','CAL2','CAL3']).length,1);assert.equal(independent(['CAL1','CAL4','CAL5']).length,3);assert.equal(independent(['CAL_MERGED','CAL2','CAL5']).length,2);assert.equal(independent(['CAL_MERGED','CAL4','CAL5']).length,2);assert.equal(independent(['CAL1','CAL2','CAL3','CAL4','CAL5','CAL6']).length,4);
const bad=new DAG();bad.add('A',['B']);bad.add('B',['A']);assert.throws(()=>bad.roots('A'),/cycle/);
const unknown=new DAG();unknown.add('A',['MISSING']);assert.throws(()=>unknown.roots('A'),/unknown ancestor/);
assert.throws(()=>d.add('CAL1',['DATA_Z']),/immutable node/);
const entropy=p=>p<=0||p>=1?0:-p*Math.log2(p)-(1-p)*Math.log2(1-p);
const gain=(p,s,c)=>{const py=p*s+(1-p)*(1-c),yes=py?p*s/py:0,no=1-py?p*(1-s)/(1-py):0;return entropy(p)-py*entropy(yes)-(1-py)*entropy(no)};
const rates=[{id:'CAL1',s:.65,c:.60},{id:'CAL2',s:.80,c:.75},{id:'CAL3',s:.90,c:.80},{id:'CAL4',s:.98,c:.95}];
const values=rates.map(r=>({id:r.id,roots:roots(r.id),gain:gain(.25,r.s,r.c)}));assert(values.every(x=>x.gain>0));assert.equal(independent(rates.map(x=>x.id)).length,2);
const journal=[];function append(kind,data){const seq=journal.length,previous=seq?journal.at(-1).hash:'0'.repeat(64),e={seq,previous,kind,data};journal.push({...e,hash:hash(e)})}
function valid(log){let prev='0'.repeat(64);return log.every((e,i)=>{const ok=e.seq===i&&e.previous===prev&&e.hash===hash({seq:e.seq,previous:e.previous,kind:e.kind,data:e.data});prev=e.hash;return ok})}
append('PARENT_H060',{id:'H060',models:4});append('ANCESTRY_GRAPH',{nodes:d.nodes.size});append('COUNTEREXAMPLE',{models:['CAL1','CAL2','CAL3'],uniqueRoots:1});append('CORRECTION',{models:['CAL1','CAL4','CAL5'],uniqueRoots:3});append('CONSERVATION',{modelBasedGains:values.map(x=>x.gain),independentOrigins:2});assert(valid(journal));const tampered=structuredClone(journal);tampered[2].data.uniqueRoots=3;assert(!valid(tampered));
console.log(JSON.stringify({status:'PASS',graph_nodes:d.nodes.size,shared_three_models_count:1,independent_three_models_count:3,independent_four_numerical_models:2,append_only_records:journal.length,gains:values.map(x=>({id:x.id,bits:+x.gain.toFixed(6),roots:x.roots}))},null,2));
module.exports={DAG,independent};