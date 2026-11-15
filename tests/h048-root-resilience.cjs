'use strict';
// H048 — Root resilience, ancestry overlap, and immutable recovery generations.
const assert = require('node:assert/strict');
const crypto = require('node:crypto');
const digest = x => crypto.createHash('sha256').update(JSON.stringify(x)).digest('hex');
const allSubsets = a => {
  const out=[];for(let mask=0;mask<(1<<a.length);mask++)out.push(a.filter((_,i)=>mask&(1<<i)));
  return out;
};
// A witness depends on ALL its ultimate ancestry roots; a failed root invalidates the witness.
function maximumDisjoint(witnesses,failed=[]){
 const broken=new Set(failed);
 const valid=witnesses.filter(w=>w.roots.every(r=>!broken.has(r)));
 let best=[];
 function backtrack(i,selected,used){
  if(i===valid.length){if(selected.length>best.length)best=[...selected];return;}
  backtrack(i+1,selected,used);
  const w=valid[i];
  if(w.roots.every(r=>!used.has(r)))backtrack(i+1,[...selected,w.id],new Set([...used,...w.roots]));
 }
 backtrack(0,[],new Set());
 return best;
}
function minimumCut(witnesses,threshold=3){
 const roots=[...new Set(witnesses.flatMap(w=>w.roots))].sort();
 const initially=maximumDisjoint(witnesses);
 if(initially.length<threshold)return {eligible:false,minCut:0,cuts:[[]],roots};
 const failures=allSubsets(roots).filter(f=>maximumDisjoint(witnesses,f).length<threshold);
 const n=Math.min(...failures.map(f=>f.length));
 return {eligible:true,minCut:n,cuts:failures.filter(f=>f.length===n),roots};
}
const cases=[];
for(let n=3;n<=7;n++){
 const base=Array.from({length:n},(_,i)=>({id:'W'+(i+1),roots:['R'+(i+1)]}));
 const result=minimumCut(base);
 assert.equal(result.minCut,n-2);
 assert.equal(result.cuts.length,(n*(n-1)/2)); // Any n-2 failures cause <3 remaining.
 cases.push({n,minimumCut:result.minCut,minimumCuts:result.cuts.length});
}
const base4=Array.from({length:4},(_,i)=>({id:'W'+(i+1),roots:['R'+(i+1)]}));
const mirrored=[...base4,{id:'mirror-R1',roots:['R1']},{id:'mirror-R2',roots:['R2']}];
assert.equal(minimumCut(mirrored).minCut,2);
assert.equal(maximumDisjoint(mirrored).length,4);
const merged=[...base4,{id:'merged-R1R2',roots:['R1','R2']}];
assert.equal(minimumCut(merged).minCut,2);
assert.equal(maximumDisjoint(merged).length,4);
// A merged witness cannot be counted together with either overlapping parent.
assert.equal(maximumDisjoint([{id:'a',roots:['R1']},{id:'b',roots:['R2']},{id:'m',roots:['R1','R2']}]).length,2);
const uncertain=[{id:'a',roots:['R1']},{id:'b',roots:['R2']},{id:'c',roots:[]}];
// Unknown-source packets cannot graduate. Reject invalid ancestry before set operations.
function validate(ws){for(const w of ws)if(!w.roots.length||new Set(w.roots).size!==w.roots.length)throw Error('unverified-ancestry');}
assert.throws(()=>validate(uncertain),/unverified-ancestry/);
let journal=[];
function append(type,payload){
 const previous=journal.length?journal.at(-1).hash:'0'.repeat(64);
 const record={seq:journal.length,previous,type,payload};
 journal.push(Object.freeze({...record,hash:digest(record)}));
}
function verify(){
 let previous='0'.repeat(64);
 for(let i=0;i<journal.length;i++){
  const x=journal[i],data={seq:x.seq,previous:x.previous,type:x.type,payload:x.payload};
  if(x.seq!==i||x.previous!==previous||digest(data)!==x.hash)return false;
  previous=x.hash;
 }
 return true;
}
append('ROOT_REGISTERED',{root:'R1',generation:0});
append('ROOT_COMPROMISED',{root:'R1',generation:0});
append('NEW_GENERATION',{root:'R1-gen1',parentRoot:'R1',generation:1});
append('REPAIR_ASSESSMENT',{root:'R1-gen1',status:'PROVISIONAL',priorRootCompromised:true});
assert(verify());assert.equal(journal.length,4);
assert.equal(journal[1].payload.root,'R1');
assert.equal(journal[3].payload.status,'PROVISIONAL');
const before=journal.length;
append('REPAIR_REJECTED',{root:'R1-gen1',reason:'shared-origin; not new independent root'});
assert.equal(journal.length,before+1);
assert(verify());assert.equal(journal[1].type,'ROOT_COMPROMISED');
const altered=journal[1];journal[1]={...altered,payload:{root:'R1',generation:0,rewritten:true}};
assert(!verify());journal[1]=altered;assert(verify());
console.log(JSON.stringify({status:'PASS',independent_root_cases:cases,mirror_minCut:2,merged_minCut:2,merged_overlap_rejected:true,unknown_ancestry_rejected:true,append_only_events:journal.length,old_compromise_preserved:true,repair_graduation:false,limitation:'synthetic trust roots and in-memory hash chain; no durable transaction or external provenance authentication'},null,2));
