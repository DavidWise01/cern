'use strict';
const assert=require('node:assert/strict');
const choose=(a,k)=>{if(!k)return[[]];if(a.length<k)return[];return [...choose(a.slice(1),k-1).map(x=>[a[0],...x]),...choose(a.slice(1),k)]};
const model=(groups)=>({roots:groups.map((x,i)=>({id:'R'+i,domains:['source:'+i,...Object.entries(x).map(([k,v])=>k+':'+v)]})),witnesses:groups.map((_,i)=>({id:'W'+i,roots:['R'+i]}))});
function packing(g,fail=[]){const dead=new Set(g.roots.filter(r=>r.domains.some(d=>fail.includes(d))).map(r=>r.id));const live=g.witnesses.filter(w=>w.roots.every(r=>!dead.has(r)));let best=0;function rec(i,n,used){if(i===live.length){best=Math.max(best,n);return}rec(i+1,n,used);const w=live[i];if(w.roots.every(r=>!used.has(r)))rec(i+1,n+1,new Set([...used,...w.roots]))}rec(0,0,new Set());return best}
function cut(g,scope='all'){const domains=[...new Set(g.roots.flatMap(r=>r.domains))].filter(x=>scope==='all'||x.startsWith(scope+':'));if(packing(g)<3)return 0;for(let n=1;n<=domains.length;n++)if(choose(domains,n).some(f=>packing(g,f)<3))return n;return null}
const make=(r,p,pw,b)=>r.map((region,i)=>({region,provider:p[i],power:pw[i],backbone:b[i]}));
const cases=[
['oneRegion',make(['N','N','N','N'],['A','B','C','D'],['1','2','3','4'],['1','2','3','4']),1,1],
['twoRegions',make(['N','N','S','S'],['A','B','C','D'],['1','2','3','4'],['1','2','3','4']),1,1],
['fourRegions',make(['N','S','E','W'],['A','B','C','D'],['1','2','3','4'],['1','2','3','4']),2,2],
['commonPower',make(['N','S','E','W'],['A','B','C','D'],['G','G','G','G'],['1','2','3','4']),1,2],
['bridgeBackbone',make(['N','S','E','W'],['A','B','C','D'],['1','2','3','4'],['B','B','3','4']),1,2],
['twoProviders',make(['N','S','E','W'],['A','A','B','B'],['1','2','3','4'],['1','2','3','4']),1,2],
['fiveTwoTwoOne',make(['N','N','S','S','E'],['A','B','C','D','E'],['1','2','3','4','5'],['1','2','3','4','5']),2,2],
['fiveIndependent',make(['N','S','E','W','C'],['A','B','C','D','E'],['1','2','3','4','5'],['1','2','3','4','5']),3,3]
];
const report={};for(const [name,data,a,r] of cases){const g=model(data);assert.equal(cut(g),a,name);assert.equal(cut(g,'region'),r,name+' region');report[name]={all:a,region:r};}
const optimization=[];for(let count=1;count<=4;count++){let best=0;for(let code=0;code<count**4;code++){let n=code;const regions=Array.from({length:4},()=>{const d=n%count;n=Math.floor(n/count);return 'D'+d});const g=model(make(regions,['A','B','C','D'],['1','2','3','4'],['1','2','3','4']));best=Math.max(best,cut(g,'region'));}optimization.push(best);}
assert.deepEqual(optimization,[1,1,1,2]);
const orig=model(cases[2][1]);orig.witnesses.push({id:'mirror',roots:['R0']});assert.equal(cut(orig),2);orig.witnesses.push({id:'merged',roots:['R0','R1']});assert.equal(cut(orig),2);
console.log(JSON.stringify({status:'PASS',scenarios:report,optimal_region_cuts:optimization,mirror_and_merge_do_not_help:true},null,2));
