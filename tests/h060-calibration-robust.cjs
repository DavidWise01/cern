'use strict';
const assert=require('node:assert/strict'),crypto=require('node:crypto');
const H=p=>p<=0||p>=1?0:-p*Math.log2(p)-(1-p)*Math.log2(1-p);
const posterior=(p,s,c,out)=>{const a=out==='SHARED'?s:1-s,b=out==='SHARED'?1-c:c;return p*a/(p*a+(1-p)*b)};
const gain=(p,s,c)=>{const py=p*s+(1-p)*(1-c);return H(p)-py*H(posterior(p,s,c,'SHARED'))-(1-py)*H(posterior(p,s,c,'SEPARATE'))};
const models=[[.65,.60],[.80,.75],[.90,.80],[.98,.95]],tau=.10;
const candidates=[{id:'B',root:'B',cost:3},{id:'C',root:'C',cost:5},{id:'D',root:'existing-X',cost:1}];
function assess(p,used=new Set(['existing-X'])){
 const rows=candidates.filter(q=>!used.has(q.root)).map(q=>{const scores=models.map(([s,c])=>gain(p,s,c)/q.cost);return {id:q.id,min:Math.min(...scores),max:Math.max(...scores)}});
 const robust=rows.filter(r=>r.min>=tau),plausible=rows.filter(r=>r.max>=tau);
 return {decision:robust.length?'CONTINUE':plausible.length?'REVIEW_CALIBRATION':'STOP',rows};
}
const ps=posterior(.25,.8,.8,'SHARED'),pn=posterior(.25,.8,.8,'SEPARATE');
assert(Math.abs(ps-4/7)<1e-12);assert(Math.abs(pn-1/13)<1e-12);
assert.equal(assess(ps).decision,'REVIEW_CALIBRATION');assert.equal(assess(pn).decision,'STOP');
assert.equal(assess(.25).decision,'REVIEW_CALIBRATION');
assert.equal(assess(.25,new Set(['existing-X','B','C'])).decision,'STOP');
const strong=models.map(([s,c])=>gain(.5,s,c));assert(Math.min(...strong)>.04);
assert(assess(ps).rows.every(r=>r.min<tau));
const journal=[],hash=x=>crypto.createHash('sha256').update(JSON.stringify(x)).digest('hex');
function append(type,data){const seq=journal.length,prev=seq?journal.at(-1).hash:'0'.repeat(64),entry={seq,prev,type,data};journal.push({...entry,hash:hash(entry)})}
function valid(log){let prev='0'.repeat(64);return log.every((e,i)=>{const ok=e.seq===i&&e.prev===prev&&e.hash===hash({seq:e.seq,prev:e.prev,type:e.type,data:e.data});prev=e.hash;return ok})}
append('H059_PARENT',{prior:.25});append('CALIBRATION',{models,tau});append('HYPOTHETICAL_SHARED',{posterior:ps,decision:assess(ps).decision});append('HYPOTHETICAL_SEPARATE',{posterior:pn,decision:assess(pn).decision});append('AUDIT',{no_actual_observation:true});assert(valid(journal));const bad=structuredClone(journal);bad[2].data.decision='CONTINUE';assert(!valid(bad));
console.log(JSON.stringify({status:'PASS',shared:assess(ps),separate:assess(pn),counterfactuals:true,journalEntries:journal.length},null,2));
