'use strict';
const assert=require('node:assert/strict');
const crypto=require('node:crypto');
const links=['AD','AE','AF','DE','DF','EF'];
const weights=[25,15,8,5,4,3];
const hypotheses=[{id:'NONE',weight:40,shared:null},...links.map((id,i)=>({id,weight:weights[i],shared:id}))];
const entropy=hs=>{const total=hs.reduce((n,h)=>n+h.weight,0);return hs.reduce((s,h)=>{const p=h.weight/total;return s-(p?p*Math.log2(p):0)},0)};
function split(hs,link,result){return hs.filter(h=>(h.shared===link?'SHARED':'SEPARATE')===result)}
function gain(hs,link){const prior=entropy(hs),mass=hs.reduce((s,h)=>s+h.weight,0);const conditional=['SHARED','SEPARATE'].reduce((s,result)=>{const subset=split(hs,link,result),w=subset.reduce((n,h)=>n+h.weight,0);return s+w/mass*entropy(subset)},0);return prior-conditional}
const order=links.map(link=>({link,gain:gain(hypotheses,link)})).sort((a,b)=>b.gain-a.gain||a.link.localeCompare(b.link));
assert.deepEqual(order.map(x=>x.link),links);
assert(Math.abs(order[0].gain-0.8112781244591328)<1e-9);
const shared=split(hypotheses,'AD','SHARED'),separate=split(hypotheses,'AD','SEPARATE');
assert.deepEqual(shared.map(h=>h.id),['AD']);assert.equal(separate.length,6);
const cut=hs=>Math.min(...hs.map(h=>h.shared?1:2));
assert.equal(cut(hypotheses),1);assert.equal(cut(shared),1);assert.equal(cut(separate),1);
const allClear=links.reduce((hs,link)=>split(hs,link,'SEPARATE'),hypotheses);
assert.deepEqual(allClear.map(h=>h.id),['NONE']);assert.equal(cut(allClear),2);
const hash=x=>crypto.createHash('sha256').update(JSON.stringify(x)).digest('hex'),journal=[];
function append(type,data){const prior=journal.length?journal.at(-1).hash:'0'.repeat(64);const record={seq:journal.length,prior,type,data};journal.push({...record,hash:hash(record)});}
function valid(log){let prior='0'.repeat(64);return log.every((e,i)=>{const ok=e.seq===i&&e.prior===prior&&e.hash===hash({seq:i,prior:e.prior,type:e.type,data:e.data});prior=e.hash;return ok})}
append('H053_PRIOR',{sites:'ADEF',nominal:2,robust:1});
append('H054_QUERY',{link:'AD',gain:order[0].gain});
append('H054_COUNTERFACTUAL',{branch:'SEPARATE',remaining:separate.map(h=>h.id),observed:false});
assert(valid(journal));const changed=JSON.parse(JSON.stringify(journal));changed[0].data.robust=2;assert(!valid(changed));assert.equal(journal[0].data.robust,1);
console.log(JSON.stringify({status:'PASS',hypotheses:hypotheses.length,ranked:order.map(x=>({link:x.link,bits:+x.gain.toFixed(6)})),top:'AD',shared:{hypotheses:shared.length,cut:cut(shared)},separate:{hypotheses:separate.length,cut:cut(separate)},allClear:{hypotheses:allClear.length,cut:cut(allClear)},journal:journal.length,scope:'Synthetic single-hidden-link assumptions; no real-world dependency observed.'},null,2));
