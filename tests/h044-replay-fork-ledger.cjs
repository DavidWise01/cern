'use strict';
// H044: append-only replay / fork detection; Node.js standard library.
const assert=require('node:assert/strict');
const crypto=require('node:crypto');
const sha=x=>crypto.createHash('sha256').update(JSON.stringify(x)).digest('hex');
const genesis='0'.repeat(64);
class Ledger {
 constructor(snapshot){this.entries=snapshot?JSON.parse(snapshot):[];this.rebuild();}
 rebuild(){
  this.events=new Set();this.byHash=new Map();this.children=new Map();
  for(const e of this.entries){const h=sha(e);assert(!this.events.has(e.event));this.events.add(e.event);this.byHash.set(h,e);
   const key=e.parent+'|'+e.epoch;const list=this.children.get(key)||[];list.push(h);this.children.set(key,list);}
 }
 snapshot(){return JSON.stringify(this.entries);}
 append(e){
  // The caller has authenticated the event; this demo does not authenticate signers.
  if(this.events.has(e.event))return {decision:'REPLAY',stored:false};
  if(!Number.isSafeInteger(e.epoch)||e.epoch<0||!e.event||!e.state||!e.parent)return {decision:'INVALID',stored:false};
  if(e.parent!==genesis&&!this.byHash.has(e.parent))return {decision:'ORPHAN',stored:false};
  if(e.parent===genesis&&e.epoch!==0)return {decision:'INVALID_GENESIS',stored:false};
  if(e.parent!==genesis&&e.epoch!==this.byHash.get(e.parent).epoch+1)return {decision:'INVALID_EPOCH',stored:false};
  const hash=sha(e);this.entries.push({...e});this.rebuild();
  const siblings=this.children.get(e.parent+'|'+e.epoch)||[];
  const conflict=siblings.filter(h=>h!==hash&&this.byHash.get(h).state!==e.state);
  if(conflict.length)return {decision:'FORK',stored:true,hash,conflicts:conflict};
  if(siblings.length>1)return {decision:'DUPLICATE_STATE_BRANCH',stored:true,hash};
  return {decision:'APPENDED',stored:true,hash};
 }
 forkRoots(){return [...this.children].filter(([,hashes])=>new Set(hashes.map(h=>this.byHash.get(h).state)).size>1).map(([key,hashes])=>({key,hashes}));}
}
let L=new Ledger();const g=L.append({event:'g0',parent:genesis,epoch:0,state:'init'});
assert.equal(g.decision,'APPENDED');
const a=L.append({event:'a1',parent:g.hash,epoch:1,state:'rgb-A'});assert.equal(a.decision,'APPENDED');
assert.equal(L.append({event:'a1',parent:g.hash,epoch:1,state:'rgb-A'}).decision,'REPLAY');
L=new Ledger(L.snapshot()); // persisted entries loaded after restart
assert.equal(L.append({event:'a1',parent:g.hash,epoch:1,state:'rgb-A'}).decision,'REPLAY');
const b=L.append({event:'b1',parent:g.hash,epoch:1,state:'rgb-B'});
assert.equal(b.decision,'FORK');assert.equal(L.entries.length,3);assert.equal(L.forkRoots().length,1);
const a2=L.append({event:'a2',parent:a.hash,epoch:2,state:'rgb-A2'});assert.equal(a2.decision,'APPENDED');
const b2=L.append({event:'b2',parent:b.hash,epoch:2,state:'rgb-B2'});assert.equal(b2.decision,'APPENDED');
assert.equal(L.append({event:'z',parent:'1'.repeat(64),epoch:5,state:'orphan'}).decision,'ORPHAN');
assert.equal(L.append({event:'bad',parent:a.hash,epoch:7,state:'skip'}).decision,'INVALID_EPOCH');
const snapshot=L.snapshot(),restored=new Ledger(snapshot);assert.deepEqual(restored.entries,L.entries);assert.equal(restored.forkRoots().length,1);
const divergent=restored.forkRoots()[0];assert.equal(divergent.key,g.hash+'|1');
assert.notEqual(a.hash,b.hash);assert.equal(restored.byHash.get(a.hash).parent,restored.byHash.get(b.hash).parent);
assert.equal(restored.entries.length,5);
console.log(JSON.stringify({status:'PASS',retained_events:5,fork_count:1,replay_before_restart:'REJECTED',replay_after_restart:'REJECTED',branch_A_retained:true,branch_B_retained:true,orphan:'REJECTED',epoch_jump:'REJECTED',first_conflicting_parent:g.hash,limitations:'In-memory simulation of durable state through serialization/reload. Real append-only storage, authenticated roots, atomic transactions, and tamper-resistant hashes must be enforced by deployment.'},null,2));
