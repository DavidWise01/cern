'use strict';
const assert=require('node:assert/strict'),crypto=require('node:crypto');
const hash=o=>crypto.createHash('sha256').update(JSON.stringify(o)).digest('hex');
const parent='G0',A={parent,epoch:1,state:'RGB-A',observation:{octant:[1,1,1],primary:'R',secondary:'RG'}},B={parent,epoch:1,state:'RGB-B',observation:{octant:[1,-1,1],primary:'G',secondary:'GB'}};
const branches=[A,B].map(x=>({...x,hash:hash(x),observationHash:hash(x.observation)}));
const witness=(id,root,ancestor,branch,extra={})=>({id,root,ancestor,parent,epoch:1,branchHash:branches[branch].hash,observationHash:branches[branch].observationHash,authenticated:true,fresh:true,...extra});
function decide(packets){
const seen=new Set(),valid=[],rejected=[];
for(const w of packets){let reason=null;if(!w.authenticated)reason='unauthenticated';else if(!w.fresh)reason='stale';else if(w.parent!==parent||w.epoch!==1)reason='wrong-checkpoint';else if(seen.has(w.root+'|'+w.id))reason='replay';else if(!w.root||!w.ancestor)reason='unverified-lineage';else {const b=branches.find(x=>x.hash===w.branchHash);if(!b||b.observationHash!==w.observationHash)reason='observation-mismatch';}
if(reason)rejected.push({id:w.id,reason});else {seen.add(w.root+'|'+w.id);valid.push(w)}}
const votes=new Map();for(const w of valid){if(!votes.has(w.root))votes.set(w.root,new Set());votes.get(w.root).add(w.branchHash)}
const equivocal=[...votes].filter(([,v])=>v.size>1).map(([r])=>r),eligible=valid.filter(w=>!equivocal.includes(w.root));
const support=branches.map(b=>{const members=eligible.filter(w=>w.branchHash===b.hash);const roots=[...new Set(members.map(x=>x.root))],ancestors=[...new Set(members.map(x=>x.ancestor))];return {branch:b.state,roots,ancestors,count:ancestors.length,qualified:ancestors.length>=3}});
const qualified=support.filter(s=>s.qualified);
return {outcome:qualified.length===1?'SUPPORTED_NOT_MERGED':qualified.length>1?'CONTESTED':'UNRESOLVED',support,equivocal,rejected,branchesPreserved:branches.map(x=>x.hash)};
}
const scenarios=[
['independent_A',[witness('a','A','srcA',0),witness('b','B','srcB',0),witness('c','C','srcC',0)],'SUPPORTED_NOT_MERGED'],
['shared_ancestor',[witness('a','A','srcX',0),witness('b','B','srcX',0),witness('c','C','srcC',0)],'UNRESOLVED'],
['both_qualified',[witness('a','A','srcA',0),witness('b','B','srcB',0),witness('c','C','srcC',0),witness('d','D','srcD',1),witness('e','E','srcE',1),witness('f','F','srcF',1)],'CONTESTED'],
['observation_mismatch',[witness('a','A','srcA',0),witness('b','B','srcB',0),witness('c','C','srcC',0,{observationHash:'wrong'})],'UNRESOLVED','observation-mismatch'],
['stale',[witness('a','A','srcA',0),witness('b','B','srcB',0),witness('c','C','srcC',0,{fresh:false})],'UNRESOLVED','stale'],
['forgery',[witness('a','A','srcA',0),witness('b','B','srcB',0),witness('c','C','srcC',0,{authenticated:false})],'UNRESOLVED','unauthenticated'],
['replay',[witness('a','A','srcA',0),witness('b','B','srcB',0),witness('b','B','srcB',0),witness('c','C','srcC',0)],'SUPPORTED_NOT_MERGED','replay'],
['equivocation',[witness('a','A','srcA',0),witness('aa','A','srcA',1),witness('b','B','srcB',0),witness('c','C','srcC',0)],'UNRESOLVED',null,'A'],
['wrong_parent',[witness('a','A','srcA',0),witness('b','B','srcB',0),witness('c','C','srcC',0,{parent:'OTHER'})],'UNRESOLVED','wrong-checkpoint'],
['unknown_ancestry',[witness('a','A','srcA',0),witness('b','B','srcB',0),witness('c','C',null,0)],'UNRESOLVED','unverified-lineage'],
['two_roots',[witness('a','A','srcA',0),witness('b','B','srcB',0)],'UNRESOLVED'],
['no_votes',[],'UNRESOLVED']
];
for(const [name,packets,expected,reason,equivocal] of scenarios){const r=decide(packets);assert.equal(r.outcome,expected,name);if(reason)assert(r.rejected.some(x=>x.reason===reason),name);if(equivocal)assert(r.equivocal.includes(equivocal),name);assert.deepEqual(r.branchesPreserved,branches.map(x=>x.hash));}
console.log(JSON.stringify({status:'PASS',scenarios:scenarios.length,supported:2,contested:1,unresolved:9,warning:'Synthetic authentication and lineage metadata. No trusted identity proof or automatic merge.'},null,2));
