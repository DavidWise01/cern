'use strict';
const assert=require('node:assert/strict');
// H055 - noisy binary dependency observations; declared source roots and calibration.
const prior=0.25, sensitivity=0.9, specificity=0.8;
const L=(outcome)=>outcome==='SHARED'?{h:sensitivity,n:1-specificity}:{h:1-sensitivity,n:specificity};
function assess(reports){
 const seen=new Map(),duplicates=[],invalid=[];
 for(const r of reports){
  if(!r.authenticated||!r.calibrated||!r.fresh||!['SHARED','SEPARATE'].includes(r.outcome)||!r.root){
   invalid.push(r.id);continue;
  }
  if(seen.has(r.root)){duplicates.push(r.id);continue;}
  seen.set(r.root,r);
 }
 let h=prior,n=1-prior;
 for(const r of seen.values()){const f=L(r.outcome);h*=f.h;n*=f.n;}
 const posterior=h/(h+n),yes=[...seen.values()].filter(r=>r.outcome==='SHARED'),no=[...seen.values()].filter(r=>r.outcome==='SEPARATE');
 const corroborated=seen.size>=3&&(yes.length===0||no.length===0);
 // Confidence score alone cannot promote; conflicting roots trigger review.
 const decision=corroborated?'REVIEW_ELIGIBLE':yes.length&&no.length?'CONFLICT_FREEZE':'INSUFFICIENT_ROOTS';
 return {posterior,decision,independent:seen.size,yes:yes.length,no:no.length,duplicates,invalid};
}
const r=(id,root,outcome,options={})=>({id,root,outcome,authenticated:true,calibrated:true,fresh:true,...options});
const scenarios={
 three_shared:[r('a','A','SHARED'),r('b','B','SHARED'),r('c','C','SHARED')],
 three_separate:[r('a','A','SEPARATE'),r('b','B','SEPARATE'),r('c','C','SEPARATE')],
 conflict:[r('a','A','SHARED'),r('b','B','SEPARATE'),r('c','C','SHARED')],
 mirrored:[r('a','A','SHARED'),r('copy1','A','SHARED'),r('copy2','A','SHARED')],
 two_roots:[r('a','A','SHARED'),r('b','B','SHARED')],
 stale:[r('a','A','SHARED'),r('b','B','SHARED'),r('c','C','SHARED',{fresh:false})],
 uncalibrated:[r('a','A','SHARED'),r('b','B','SHARED'),r('c','C','SHARED',{calibrated:false})],
 unauthenticated:[r('a','A','SHARED'),r('b','B','SHARED'),r('c','C','SHARED',{authenticated:false})],
 four_conflict:[r('a','A','SHARED'),r('b','B','SHARED'),r('c','C','SHARED'),r('d','D','SEPARATE')]
};
const expected=['REVIEW_ELIGIBLE','REVIEW_ELIGIBLE','CONFLICT_FREEZE','INSUFFICIENT_ROOTS','INSUFFICIENT_ROOTS','INSUFFICIENT_ROOTS','INSUFFICIENT_ROOTS','INSUFFICIENT_ROOTS','CONFLICT_FREEZE'];
const report={};for(const [i,[name,data]] of Object.entries(scenarios).entries()){const z=assess(data);assert.equal(z.decision,expected[i],name);report[name]={posterior:+z.posterior.toFixed(6),decision:z.decision,distinct_roots:z.independent};}
assert.equal(assess(scenarios.mirrored).duplicates.length,2);
assert(assess(scenarios.three_shared).posterior>.9);
assert(assess(scenarios.three_separate).posterior<.01);
assert.equal(assess(scenarios.four_conflict).decision,'CONFLICT_FREEZE');
console.log(JSON.stringify({status:'PASS',scenarios:Object.keys(scenarios).length,prior,sensitivity,specificity,report,warning:'Posterior relies on assumed conditional independence and synthetic calibration; REVIEW_ELIGIBLE never means authenticated truth or auto-promotion.'},null,2));
module.exports={assess};
