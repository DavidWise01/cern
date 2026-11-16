'use strict';
// H052 exact four-site placement search, source roots are distinct by assumption.
const assert=require('node:assert/strict');
const S=[
{id:'A',cost:2,region:'N',provider:'P1',power:'G1',backbone:'K1'},
{id:'B',cost:2,region:'N',provider:'P2',power:'G2',backbone:'K2'},
{id:'C',cost:3,region:'S',provider:'P1',power:'G2',backbone:'K3'},
{id:'D',cost:3,region:'S',provider:'P3',power:'G3',backbone:'K4'},
{id:'E',cost:4,region:'E',provider:'P4',power:'G4',backbone:'K5'},
{id:'F',cost:5,region:'W',provider:'P5',power:'G5',backbone:'K6'}];
const layers=['region','provider','power','backbone'];
const choose=(a,k)=>k===0?[[]]:a.length<k?[]:[...choose(a.slice(1),k-1).map(x=>[a[0],...x]),...choose(a.slice(1),k)];
function domainCut(chosen){
 const deps=chosen.map(s=>layers.map(k=>k+':'+s[k]));
 const all=[...new Set(deps.flat())];
 let best=Infinity;
 for(let mask=0;mask<2**all.length;mask++){
   const failures=all.filter((_,i)=>mask&(2**i));
   if(failures.length>=best)continue;
   const survivors=deps.filter(d=>d.every(v=>!failures.includes(v))).length;
   if(survivors<3)best=failures.length;
 }
 return best;
}
const options=choose(S,4).map(s=>({ids:s.map(x=>x.id),cost:s.reduce((n,x)=>n+x.cost,0),cut:domainCut(s)}));
function best(budget){const choices=options.filter(o=>o.cost<=budget).sort((a,b)=>b.cut-a.cut||a.cost-b.cost||a.ids.join('').localeCompare(b.ids.join('')));return choices[0]??null;}
assert.equal(options.length,15);
assert.equal(best(9),null);
assert.deepEqual({ids:best(10).ids,cost:best(10).cost,cut:best(10).cut},{ids:['A','B','C','D'],cost:10,cut:1});
assert.equal(best(13).cut,1);
assert.deepEqual({ids:best(14).ids,cost:best(14).cost,cut:best(14).cut},{ids:['A','D','E','F'],cost:14,cut:2});
assert.equal(best(20).cut,2);
const winner=S.filter(s=>'ADEF'.includes(s.id));
assert.equal(domainCut(winner),2);
assert.equal(domainCut(winner.map(s=>({...s,power:'SHARED'}))),1);
// Mirror of A has the same underlying source and does not increase packing.
const mirror={...S[0],id:'A-mirror',origin:'A'};
assert.equal(new Set([S[0],mirror].map(s=>s.origin||s.id)).size,1);
const frontier=options.filter(a=>!options.some(b=>b.cost<=a.cost&&b.cut>=a.cut&&(b.cost<a.cost||b.cut>a.cut)));
assert.equal(frontier.length,3);
console.log(JSON.stringify({status:'PASS',options:15,budgets:{9:best(9),10:best(10),13:best(13),14:best(14),20:best(20)},pareto:frontier,shared_power_cut:1,scope:'Synthetic costs, known failure dependencies, distinct roots; no empirical pricing or probabilities.'},null,2));
module.exports={S,options,best,domainCut};