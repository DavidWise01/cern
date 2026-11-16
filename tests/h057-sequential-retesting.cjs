'use strict';
const assert=require('node:assert/strict'),crypto=require('node:crypto');
const prior=.25,calibrations=[[.65,.60],[.80,.75],[.90,.80],[.98,.95]];
const report=(id,root,outcome,other={})=>({id,root,outcome,fresh:true,authenticated:true,calibrated:true,...other});
function prepare(input){let roots=new Map(),invalid=[],duplicates=[],equivocal=[];for(const r of input){if(!r.root||!r.fresh||!r.authenticated||!r.calibrated||!['SHARED','SEPARATE'].includes(r.outcome)){invalid.push(r.id);continue}if(!roots.has(r.root))roots.set(r.root,[]);roots.get(r.root).push(r)}let selected=[];for(const [root,ps] of roots){if(new Set(ps.map(x=>x.outcome)).size>1){equivocal.push(root);continue}selected.push(ps[0]);duplicates.push(...ps.slice(1).map(x=>x.id))}return{selected,invalid,duplicates,equivocal}}
function posterior(reports,s,c){let odds=prior/(1-prior);for(const r of reports)odds*=r.outcome==='SHARED'?s/(1-c):(1-s)/c;return odds/(1+odds)}
function evaluate(raw){const p=prepare(raw),outcomes=new Set(p.selected.map(x=>x.outcome)),band=calibrations.map(([s,c])=>posterior(p.selected,s,c));return{decision:p.equivocal.length?'EQUIVOCATION_FREEZE':outcomes.size>1?'CONFLICT_FREEZE':p.selected.length>=3?'REVIEW_ELIGIBLE':'INSUFFICIENT_ROOTS',band:[Math.min(...band),Math.max(...band)],...p}}
const A=report('a','A','SHARED'),B=report('b','B','SHARED'),C=report('c','C','SHARED');
const base=evaluate([A,B]),duplicate=evaluate([A,B,report('copy','A','SHARED')]),independent=evaluate([A,B,C]);
assert.equal(base.decision,'INSUFFICIENT_ROOTS');assert.deepEqual(base.band,duplicate.band);assert.equal(duplicate.duplicates.length,1);assert.equal(independent.decision,'REVIEW_ELIGIBLE');assert(independent.band[0]>base.band[0]&&independent.band[1]>base.band[1]);
assert.equal(evaluate([A,B,report('c','C','SEPARATE')]).decision,'CONFLICT_FREEZE');
assert.equal(evaluate([A,B,C,report('a2','A','SEPARATE')]).decision,'EQUIVOCATION_FREEZE');
assert.equal(evaluate([A,B,report('c','C','SHARED',{fresh:false})]).decision,'INSUFFICIENT_ROOTS');
assert.equal(evaluate([A,B,report('c','','SHARED')]).decision,'INSUFFICIENT_ROOTS');
assert.equal(evaluate([A,B,report('c','C','SEPARATE'),report('d','D','SEPARATE')]).decision,'CONFLICT_FREEZE');
const entropy=p=>p<=0||p>=1?0:-p*Math.log2(p)-(1-p)*Math.log2(1-p);
const gains=calibrations.map(([s,c])=>{const py=prior*s+(1-prior)*(1-c),yes=posterior([A],s,c),no=posterior([report('n','N','SEPARATE')],s,c);return entropy(prior)-py*entropy(yes)-(1-py)*entropy(no)});
assert(gains.every(x=>x>0));
let journal=[],hash=o=>crypto.createHash('sha256').update(JSON.stringify(o)).digest('hex');function append(type,payload){const seq=journal.length,prev=seq?journal.at(-1).hash:'0'.repeat(64),r={seq,prev,type,payload:structuredClone(payload)};journal.push({...r,hash:hash(r)})}
function verify(log){let prev='0'.repeat(64);return log.every((e,i)=>{const ok=e.seq===i&&e.prev===prev&&e.hash===hash({seq:e.seq,prev:e.prev,type:e.type,payload:e.payload});prev=e.hash;return ok})}
append('PRIOR',{prior,calibrations});append('REPORT',A);append('REPORT',B);append('DUPLICATE',report('copy','A','SHARED'));append('REPORT',C);append('CERTIFICATE',{decision:independent.decision,band:independent.band});assert(verify(journal));
const tampered=structuredClone(journal);tampered[1].payload.outcome='SEPARATE';assert(!verify(tampered));
console.log(JSON.stringify({status:'PASS',governance_cases:8,calibration_models:4,baseline_band:base.band,three_independent_band:independent.band,duplicate_ignored:true,expected_information_gain:gains,append_only_records:journal.length},null,2));
module.exports={prepare,posterior,evaluate};
