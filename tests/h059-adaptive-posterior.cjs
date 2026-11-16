'use strict';
const assert=require('node:assert/strict'),crypto=require('node:crypto');
const prior=.25,minimumEfficiency=.10;
const tests=[{id:'A',root:'root-A',cost:1,s:.8,c:.8},{id:'B',root:'root-B',cost:3,s:.96,c:.96},{id:'C',root:'root-C',cost:5,s:.99,c:.99},{id:'D',root:'existing-X',cost:1,s:.99,c:.99},{id:'E',root:'unverified-E',cost:2,s:.99,c:.99,verified:false}];
const H=p=>p<=0||p>=1?0:-p*Math.log2(p)-(1-p)*Math.log2(1-p);
function update(p,t,result){const a=result==='SHARED'?t.s:1-t.s,b=result==='SHARED'?1-t.c:t.c;return p*a/(p*a+(1-p)*b)}
function gain(p,t){const q=p*t.s+(1-p)*(1-t.c);return H(p)-q*H(update(p,t,'SHARED'))-(1-q)*H(update(p,t,'SEPARATE'))}
function rank(p,used){const rows=tests.filter(t=>t.verified!==false&&!used.has(t.root)).map(t=>({id:t.id,root:t.root,efficiency:gain(p,t)/t.cost})).sort((a,b)=>b.efficiency-a.efficiency||a.id.localeCompare(b.id));return{rows,next:rows[0]?.efficiency>=minimumEfficiency?rows[0].id:null}}
const baseline=rank(prior,new Set(['existing-X']));assert.equal(baseline.next,'A');
const A=tests[0],yes=update(prior,A,'SHARED'),no=update(prior,A,'SEPARATE');
assert(Math.abs(yes-4/7)<1e-12);assert(Math.abs(no-1/13)<1e-12);
const shared=rank(yes,new Set(['existing-X','root-A'])),separate=rank(no,new Set(['existing-X','root-A']));
assert.equal(shared.next,'B');assert.equal(separate.next,null);
assert.deepEqual(shared.rows.map(r=>r.id),['B','C']);assert.deepEqual(separate.rows.map(r=>r.id),['B','C']);
assert.equal(rank(shared.next==='B'?yes:no,new Set(['existing-X','root-A','root-B','root-C'])).next,null);
assert(update(prior,A,'SHARED')===yes);assert(update(yes,A,'SHARED')>yes); // Demonstrates double counting would be wrong.
const ledger=[],hash=x=>crypto.createHash('sha256').update(JSON.stringify(x)).digest('hex');
function append(type,data){const seq=ledger.length,prev=seq?ledger[seq-1].hash:'0'.repeat(64),x={seq,prev,type,data};ledger.push({...x,hash:hash(x)})}
function valid(items){let prev='0'.repeat(64);return items.every((x,i)=>{const ok=x.seq===i&&x.prev===prev&&x.hash===hash({seq:x.seq,prev:x.prev,type:x.type,data:x.data});prev=x.hash;return ok})}
append('BASELINE',{prior,selection:baseline.next});append('COUNTERFACTUAL',{outcome:'SHARED',posterior:yes,next:shared.next,observed:false});append('COUNTERFACTUAL',{outcome:'SEPARATE',posterior:no,next:separate.next,observed:false});append('DUPLICATE_GUARD',{root:'root-A',unchanged_posterior:yes});
assert(valid(ledger));const bad=structuredClone(ledger);bad[1].data.posterior=prior;assert(!valid(bad));
const round=x=>+x.toFixed(6);
console.log(JSON.stringify({status:'PASS',prior,first:'A',shared:{posterior:round(yes),next:shared.next,ranked:shared.rows.map(x=>({id:x.id,bits_per_cost:round(x.efficiency)}))},separate:{posterior:round(no),next:separate.next,ranked:separate.rows.map(x=>({id:x.id,bits_per_cost:round(x.efficiency)}))},duplicate_ignored:true,ledger:ledger.length,assumptions:'Synthetic accuracy, source identities and independent measurements; outcomes hypothetical.'},null,2));
