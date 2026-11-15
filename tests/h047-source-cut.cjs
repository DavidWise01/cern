'use strict';
const assert=require('node:assert/strict'),crypto=require('node:crypto');
const H=x=>crypto.createHash('sha256').update(JSON.stringify(x)).digest('hex');
class DAG{
 constructor(){this.nodes=new Map()}
 add(id,parents=[]){if(this.nodes.has(id))throw Error('immutable');if(new Set(parents).size!==parents.length)throw Error('duplicate-parent');this.nodes.set(id,{id,parents:[...parents]})}
 roots(id,stack=new Set()){if(stack.has(id))throw Error('cycle');const n=this.nodes.get(id);if(!n)throw Error('unknown:'+id);if(!n.parents.length)return new Set([id]);stack.add(id);const s=new Set();for(const p of n.parents)for(const r of this.roots(p,stack))s.add(r);stack.delete(id);return s}
}
const G=new DAG();for(const r of ['X','Y','Z','Q'])G.add(r);
G.add('github',['X']);G.add('0root',['X']);G.add('wX1',['github']);G.add('wX2',['0root']);G.add('wY',['Y']);G.add('wZ',['Z']);G.add('wQ',['Q']);G.add('merged',['github','Y']);G.add('wXY',['merged']);
const all=['wX1','wX2','wY','wZ','wQ','wXY'],roots=['X','Y','Z','Q'];
const rootSets=ws=>ws.map(id=>({id,roots:[...G.roots(id)].sort()}));
function qualified(ws,failed){const f=new Set(failed),available=rootSets(ws).filter(w=>w.roots.every(r=>!f.has(r)));let best=[];function visit(i,picked,used){if(i===available.length){if(picked.length>best.length)best=[...picked];return}visit(i+1,picked,used);const w=available[i];if(w.roots.every(r=>!used.has(r)))visit(i+1,[...picked,w.id],new Set([...used,...w.roots]))}visit(0,[],new Set());return {promotable:best.length>=3,independentWitnesses:best.length,witnesses:best,invalidated:rootSets(ws).filter(w=>w.roots.some(r=>f.has(r))).map(w=>w.id)}}
function cut(ws){const initially=qualified(ws,[]).promotable;if(!initially)return {initiallyQualified:false,minCut:0,cuts:[[]]};const failing=[];for(let mask=0;mask<1<<roots.length;mask++){const lost=roots.filter((_,i)=>mask&(1<<i));if(!qualified(ws,lost).promotable)failing.push(lost)}const minimum=Math.min(...failing.map(x=>x.length));return {initiallyQualified:true,minCut:minimum,cuts:failing.filter(x=>x.length===minimum)}}
class Audit{constructor(){this.entries=[]}append(event){const prev=this.entries.length?this.entries.at(-1).hash:'0'.repeat(64);const record={seq:this.entries.length,prev,event};const entry=Object.freeze({...record,hash:H(record)});this.entries.push(entry);return entry}verify(){let prev='0'.repeat(64);for(let i=0;i<this.entries.length;i++){const {seq,prev:p,event,hash}=this.entries[i];if(seq!==i||p!==prev||hash!==H({seq,prev:p,event}))return false;prev=hash}return true}}
const three=['wX1','wY','wZ'],four=['wX1','wY','wZ','wQ'],mirrors=['wX1','wX2','wY'],mixed=['wXY','wZ','wQ'];
assert.equal(qualified(three,[]).independentWitnesses,3);assert.equal(qualified(mirrors,[]).independentWitnesses,2);assert.equal(qualified(mixed,[]).independentWitnesses,3);
assert.deepEqual(qualified(all,['X']).invalidated,['wX1','wX2','wXY']);assert.deepEqual(qualified(all,['Y']).invalidated,['wY','wXY']);
const c3=cut(three),c4=cut(four),cm=cut(mirrors),cx=cut(mixed);
assert.equal(c3.minCut,1);assert.equal(c3.cuts.length,3);assert.equal(c4.minCut,2);assert.equal(c4.cuts.length,6);assert.equal(cm.initiallyQualified,false);
assert.equal(cx.minCut,1);assert.deepEqual(cx.cuts.map(x=>x.join('')),['X','Y','Z','Q']);
const audit=new Audit();audit.append({type:'CHECKPOINT',witnesses:three,status:'ELIGIBLE'});audit.append({type:'ROOT_COMPROMISE',root:'X',affected:qualified(all,['X']).invalidated});audit.append({type:'PROMOTION_FROZEN',checkpoint:'three'});
assert(audit.verify());const original=audit.entries[1];audit.entries[1]={...original,event:{type:'ROOT_COMPROMISE',root:'Y'}};assert(!audit.verify());audit.entries[1]=original;assert(audit.verify());
assert.throws(()=>G.add('X'),/immutable/);const cyclic=new DAG();cyclic.add('a',['b']);cyclic.add('b',['a']);assert.throws(()=>cyclic.roots('a'),/cycle/);assert.throws(()=>G.roots('MISSING'),/unknown/);
console.log(JSON.stringify({status:'PASS',three_root_cut:c3,four_root_cut:c4,multi_parent_cut:cx,root_X_blast_radius:qualified(all,['X']).invalidated,root_Y_blast_radius:qualified(all,['Y']).invalidated,audit_records:3,hash_tampering_detected:true,scope:'Declared ancestry and in-memory hash chain; no trusted external source authentication or durable storage.'},null,2));
