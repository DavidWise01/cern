'use strict';
const assert=require('node:assert/strict');
function evaluate(g,failed=[]){const dead=new Set(g.roots.filter(r=>r.domains.some(d=>failed.includes(d))).map(r=>r.id));const ws=g.witnesses.filter(w=>w.roots.every(r=>!dead.has(r)));let best=[];function walk(i,chosen,used){if(chosen.length+ws.length-i<best.length)return;if(i===ws.length){if(chosen.length>best.length)best=chosen;return;}let w=ws[i];if(w.roots.every(r=>!used.has(r)))walk(i+1,[...chosen,w.id],new Set([...used,...w.roots]));walk(i+1,chosen,used)}walk(0,[],new Set());return {packing:best.length,deadRoots:[...dead].sort(),selected:best};}
function solve(g){const ids=g.roots.map(r=>r.id),wi=g.witnesses.map(w=>w.id);assert.equal(new Set(ids).size,ids.length);assert.equal(new Set(wi).size,wi.length);assert(g.roots.every(r=>r.domains.length&&r.domains.length===new Set(r.domains).size));assert(g.witnesses.every(w=>w.roots.length&&w.roots.every(r=>ids.includes(r))));const ds=[...new Set(g.roots.flatMap(r=>r.domains))].sort();assert(ds.length<=18);const base=evaluate(g).packing;if(base<3)return {base,minCut:0,cuts:[[]]};let min=Infinity,cuts=[];for(let mask=0;mask<(1<<ds.length);mask++){const lost=ds.filter((_,i)=>mask&(1<<i));if(lost.length>min)continue;if(evaluate(g,lost).packing<3){if(lost.length<min){min=lost.length;cuts=[]}cuts.push(lost)}}return {base,minCut:min,cuts};}
const mk=(domains,sets)=>({roots:domains.map((d,i)=>({id:'R'+(i+1),domains:d})),witnesses:sets.map((rs,i)=>({id:'W'+(i+1),roots:rs.map(n=>'R'+n)}))});
const cases={
 sharedProvider:[mk([['P'],['P'],['P']],[[1],[2],[3]]),1],
 distinctProviders:[mk([['P1'],['P2'],['P3']],[[1],[2],[3]]),1],
 fourIsolated:[mk([['P1'],['P2'],['P3'],['P4']],[[1],[2],[3],[4]]),2],
 fourTwoByTwo:[mk([['PA'],['PA'],['PB'],['PB']],[[1],[2],[3],[4]]),1],
 fiveThreeShared:[mk([['P'],['P'],['P'],['Q'],['R']],[[1],[2],[3],[4],[5]]),1],
 fiveTwoShared:[mk([['P'],['P'],['Q'],['R'],['S']],[[1],[2],[3],[4],[5]]),2],
 cascadingDependencies:[mk([['P','POWER'],['Q','POWER'],['R','POWER']],[[1],[2],[3]]),1],
 mirrored:[mk([['P'],['Q'],['R']],[[1],[1],[2],[3]]),1],
 merged:[mk([['P'],['Q'],['R'],['S']],[[1],[2],[3],[4],[1,2]]),2]
};
const report={};for(const [name,[g,expected]] of Object.entries(cases)){const r=solve(g);assert.equal(r.minCut,expected,name);report[name]={roots:g.roots.length,domains:new Set(g.roots.flatMap(r=>r.domains)).size,minimumDomainCut:r.minCut};}
assert.deepEqual(evaluate(cases.sharedProvider[0],['P']).deadRoots,['R1','R2','R3']);
assert.equal(evaluate(cases.fiveTwoShared[0],['P']).packing,3);
assert.equal(solve(mk([['P'],['Q']],[[1],[2]])).minCut,0);
console.log(JSON.stringify({status:'PASS',scenarios:report,scope:'Deterministic declared failure domains; no empirical validation, probability independence, or cryptographic provenance.'},null,2));
module.exports={evaluate,solve};
