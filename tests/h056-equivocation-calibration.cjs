'use strict';
const assert=require('node:assert/strict'),crypto=require('node:crypto');
const H=x=>crypto.createHash('sha256').update(JSON.stringify(x)).digest('hex');
const CHECKPOINT='H056:A-D:2026-11-16';
const packet=(id,root,outcome,opts={})=>({id,root,checkpoint:CHECKPOINT,outcome,authenticated:true,fresh:true,calibrated:true,...opts});
function evaluate(reports){
const groups=new Map(),invalid=[];
for(const r of reports){if(!r.root||!r.id||r.checkpoint!==CHECKPOINT||!['SHARED','SEPARATE'].includes(r.outcome)||!r.authenticated||!r.fresh||!r.calibrated){invalid.push(r.id);continue}if(!groups.has(r.root))groups.set(r.root,[]);groups.get(r.root).push(r)}
const equivocal=[],duplicates=[],accepted=[];
for(const [root,items] of groups){if(new Set(items.map(x=>x.outcome)).size>1){equivocal.push(root);continue}duplicates.push(...items.slice(1).map(x=>x.id));accepted.push(items[0])}
const yes=accepted.filter(x=>x.outcome==='SHARED').length,no=accepted.length-yes;
return {decision:equivocal.length?'EQUIVOCATION_FREEZE':yes&&no?'CONFLICT_FREEZE':accepted.length>=3?'REVIEW_ELIGIBLE':'INSUFFICIENT_ROOTS',accepted,equivocal,duplicates,invalid}
}
function posterior(reports,prior,sensitivity,specificity){let h=prior,n=1-prior;for(const r of reports){if(r.outcome==='SHARED'){h*=sensitivity;n*=1-specificity}else{h*=1-sensitivity;n*=specificity}}return h/(h+n)}
const scenarios=[
['three-agree',[packet('a','A','SHARED'),packet('b','B','SHARED'),packet('c','C','SHARED')],'REVIEW_ELIGIBLE'],
['two-only',[packet('a','A','SHARED'),packet('b','B','SHARED')],'INSUFFICIENT_ROOTS'],
['copies',[packet('a','A','SHARED'),packet('aa','A','SHARED'),packet('b','B','SHARED'),packet('c','C','SHARED')],'REVIEW_ELIGIBLE'],
['cross-root-conflict',[packet('a','A','SHARED'),packet('b','B','SEPARATE'),packet('c','C','SHARED')],'CONFLICT_FREEZE'],
['same-root-equivocation',[packet('a','A','SHARED'),packet('aa','A','SEPARATE'),packet('b','B','SHARED'),packet('c','C','SHARED'),packet('d','D','SHARED')],'EQUIVOCATION_FREEZE'],
['revision-with-supersedes',[packet('a','A','SHARED'),packet('aa','A','SEPARATE',{supersedes:'a'}),packet('b','B','SHARED'),packet('c','C','SHARED')],'EQUIVOCATION_FREEZE'],
['stale',[packet('a','A','SHARED'),packet('b','B','SHARED'),packet('c','C','SHARED',{fresh:false})],'INSUFFICIENT_ROOTS'],
['unauthenticated',[packet('a','A','SHARED'),packet('b','B','SHARED'),packet('c','C','SHARED',{authenticated:false})],'INSUFFICIENT_ROOTS'],
['wrong-checkpoint',[packet('a','A','SHARED'),packet('b','B','SHARED'),packet('c','C','SHARED',{checkpoint:'other'})],'INSUFFICIENT_ROOTS'],
['three-separate',[packet('a','A','SEPARATE'),packet('b','B','SEPARATE'),packet('c','C','SEPARATE')],'REVIEW_ELIGIBLE']
];
for(const [name,reports,expected] of scenarios)assert.equal(evaluate(reports).decision,expected,name);
assert.deepEqual(evaluate(scenarios[4][1]).equivocal,['A']);
const grid=[];for(const s of [.65,.8,.9,.98])for(const c of [.6,.75,.8,.95])grid.push({s,c,p:posterior(scenarios[0][1],.25,s,c)});
const lo=Math.min(...grid.map(x=>x.p)),hi=Math.max(...grid.map(x=>x.p));
assert.equal(grid.length,16);assert(lo<.9&&hi>.99);
const mixed=[packet('a','A','SHARED'),packet('b','B','SHARED'),packet('c','C','SEPARATE')];
const mixedP=grid.map(x=>posterior(mixed,.25,x.s,x.c));assert.equal(evaluate(mixed).decision,'CONFLICT_FREEZE');
const journal=[];function append(report){const seq=journal.length,prev=seq?journal[seq-1].hash:'0'.repeat(64),content={seq,prev,report:structuredClone(report)};journal.push({...content,hash:H(content)})}
function verify(log){let prev='0'.repeat(64);return log.every((r,i)=>{const ok=r.seq===i&&r.prev===prev&&r.hash===H({seq:r.seq,prev:r.prev,report:r.report});prev=r.hash;return ok})}
scenarios[4][1].forEach(append);append({type:'EQUIVOCATION_FLAG',root:'A'});append({type:'RECALIBRATION',old:[.9,.8],candidate:[.65,.6]});
assert(verify(journal));const bad=structuredClone(journal);bad[0].report.outcome='SEPARATE';assert(!verify(bad));assert.equal(journal[0].report.outcome,'SHARED');
console.log(JSON.stringify({status:'PASS',scenarios:scenarios.length,calibration_combinations:grid.length,three_shared_posterior_range:[lo,hi],mixed_posterior_range:[Math.min(...mixedP),Math.max(...mixedP)],equivocating_root:'A',journal_entries:journal.length,scope:'Synthetic roots and calibration, not authenticated real-world sources'},null,2));
module.exports={evaluate,posterior};
