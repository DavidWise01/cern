'use strict';
const assert=require('node:assert/strict');
function normalize(ws){const ids=new Set();return ws.map(w=>{assert(typeof w.id==='string'&&w.id.length&&!ids.has(w.id),'unique witness ID required');ids.add(w.id);assert(Array.isArray(w.roots)&&w.roots.length,'known nonempty roots required');assert(w.roots.every(r=>typeof r==='string'&&r.length),'invalid root');assert(new Set(w.roots).size===w.roots.length,'duplicate root');return{id:w.id,roots:[...w.roots].sort()};});}
function pack(ws,failed=[]){const f=new Set(failed),available=ws.filter(w=>w.roots.every(r=>!f.has(r)));let best=[];function go(i,chosen,used){if(chosen.length+available.length-i<best.length)return;if(i===available.length){if(chosen.length>best.length)best=[...chosen];return;}const w=available[i];if(w.roots.every(r=>!used.has(r)))go(i+1,[...chosen,w.id],new Set([...used,...w.roots]));go(i+1,chosen,used);}go(0,[],new Set());return best;}
function solve(raw,threshold=3){assert(Number.isSafeInteger(threshold)&&threshold>0,'positive threshold');const ws=normalize(raw),roots=[...new Set(ws.flatMap(w=>w.roots))].sort();assert(roots.length<=20,'exhaustive bound 20 roots');const baseline=pack(ws);const rootImpacts=roots.map(r=>({root:r,affected:ws.filter(w=>w.roots.includes(r)).map(w=>w.id)}));if(baseline.length<threshold)return{initiallyQualified:false,baseline,minimumCut:0,minimalCuts:[[]],rootImpacts};let minimum=Infinity,cuts=[];for(let mask=0;mask<2**roots.length;mask++){const failed=roots.filter((_,i)=>mask&(2**i));if(failed.length>minimum)continue;if(pack(ws,failed).length<threshold){if(failed.length<minimum){minimum=failed.length;cuts=[];}cuts.push(failed);}}return{initiallyQualified:true,baseline,minimumCut:minimum,minimalCuts:cuts,rootImpacts};}
const w=(id,...roots)=>({id,roots});
const cases={
 independent3:[w('A','X'),w('B','Y'),w('C','Z')],
 independent5:[w('A','X'),w('B','Y'),w('C','Z'),w('D','Q'),w('E','V')],
 mirrors:[w('A','X'),w('A-copy','X'),w('B','Y'),w('C','Z')],
 mergedButIndependent:[w('XY','X','Y'),w('Z','Z'),w('Q','Q')],
 overlappingNoQuorum:[w('XY','X','Y'),w('YZ','Y','Z'),w('XZ','X','Z'),w('Q','Q')],
 mergedWithAlternates:[w('XY','X','Y'),w('Z','Z'),w('Q','Q'),w('X','X'),w('Y','Y')],
 chain:[w('A','X'),w('B','Y'),w('C','Z'),w('D','Q'),w('XY','X','Y'),w('YZ','Y','Z')]
};
const results=Object.fromEntries(Object.entries(cases).map(([name,ws])=>[name,solve(ws)]));
assert.equal(results.independent3.minimumCut,1);assert.equal(results.independent5.minimumCut,3);assert.equal(results.independent5.minimalCuts.length,10);
assert.equal(results.mirrors.minimumCut,1);assert.equal(results.mergedButIndependent.minimumCut,1);
assert.equal(results.overlappingNoQuorum.initiallyQualified,false);assert.equal(results.overlappingNoQuorum.minimumCut,0);
assert.equal(results.mergedWithAlternates.minimumCut,2);assert.equal(results.chain.minimumCut,2);
assert.deepEqual(results.mirrors.rootImpacts.find(x=>x.root==='X').affected,['A','A-copy']);
assert.throws(()=>solve([w('bad')]),/nonempty roots/);assert.throws(()=>solve([w('dup','X'),w('dup','Y')]),/unique witness/);assert.throws(()=>solve([w('bad','X','X')]),/duplicate root/);
console.log(JSON.stringify({status:'PASS',threshold:3,scenarios:Object.fromEntries(Object.entries(results).map(([name,r])=>[name,{eligible:r.initiallyQualified,baselineIndependentWitnesses:r.baseline.length,minimumCut:r.minimumCut,minimalCutCount:r.minimalCuts.length,minimalCuts:r.minimalCuts}])),limit:'Exact exhaustive solver up to 20 declared roots; exponential complexity; no authentication of claimed ancestry.'},null,2));
module.exports={solve,pack,normalize};
