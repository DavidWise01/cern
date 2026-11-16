'use strict';
const assert=require('node:assert/strict');
const crypto=require('node:crypto');
const prior=.25, MIN_GAIN_PER_COST=.05;
const entropy=p=>(p<=0||p>=1)?0:-(p*Math.log2(p)+(1-p)*Math.log2(1-p));
function informationGain(p,sensitivity,specificity){
  const py=p*sensitivity+(1-p)*(1-specificity);
  const yes=py===0?0:p*sensitivity/py;
  const no=py===1?0:p*(1-sensitivity)/(1-py);
  return entropy(p)-py*entropy(yes)-(1-py)*entropy(no);
}
const candidates=[
  {id:'A',root:'new-A',cost:1,sensitivity:.80,specificity:.80,valid:true},
  {id:'B',root:'new-B',cost:3,sensitivity:.96,specificity:.96,valid:true},
  {id:'C',root:'new-C',cost:5,sensitivity:.99,specificity:.99,valid:true},
  {id:'D',root:'existing-X',cost:1,sensitivity:.99,specificity:.99,valid:true},
  {id:'E',root:'new-E',cost:2,sensitivity:.99,specificity:.99,valid:false},
];
function rank(options,usedRoots=[],budget=Infinity,p=prior){
  return options.map(c=>{
    const eligible=c.valid && !usedRoots.includes(c.root) && c.cost<=budget;
    const gain=eligible?informationGain(p,c.sensitivity,c.specificity):0;
    return {...c,eligible,gain,efficiency:gain/c.cost,reason:!c.valid?'unverified-source':usedRoots.includes(c.root)?'duplicate-root':c.cost>budget?'over-budget':null};
  }).sort((a,b)=>b.efficiency-a.efficiency||a.cost-b.cost||a.id.localeCompare(b.id));
}
function chooseNext(options,usedRoots,budget,p=prior,threshold=MIN_GAIN_PER_COST){
  const ranked=rank(options,usedRoots,budget,p);
  const best=ranked[0];
  return best&&best.eligible&&best.efficiency>=threshold?{action:'TEST',id:best.id,gain:best.gain,efficiency:best.efficiency}:{action:'STOP',reason:'no eligible test meets cost-adjusted information threshold'};
}
const base=rank(candidates,['existing-X'],5);
assert.equal(base[0].id,'A');
const maxRaw=base.filter(x=>x.eligible).sort((a,b)=>b.gain-a.gain)[0];
assert.equal(maxRaw.id,'C');
assert.equal(base.find(x=>x.id==='D').gain,0);
assert.equal(base.find(x=>x.id==='E').gain,0);
assert.equal(chooseNext(candidates,['existing-X'],1).id,'A');
assert.equal(chooseNext(candidates,['existing-X','new-A'],1).action,'STOP');
assert.equal(chooseNext(candidates,['existing-X'],.5).action,'STOP');
assert.equal(chooseNext(candidates,['existing-X'],5,prior,1).action,'STOP');
const record=[];
function append(type,payload){const previous=record.length?record.at(-1).hash:'0'.repeat(64);const item={sequence:record.length,previous,type,payload};record.push({...item,hash:crypto.createHash('sha256').update(JSON.stringify(item)).digest('hex')});}
function verify(){let previous='0'.repeat(64);for(const x of record){const item={sequence:x.sequence,previous:x.previous,type:x.type,payload:x.payload};if(x.previous!==previous||x.hash!==crypto.createHash('sha256').update(JSON.stringify(item)).digest('hex'))return false;previous=x.hash;}return true;}
append('PRIOR',{p:prior});append('CANDIDATE_RANKING',base.map(x=>({id:x.id,gain:x.gain,efficiency:x.efficiency,reason:x.reason})));append('TEST_SELECTED',chooseNext(candidates,['existing-X'],5));append('STOP_EVALUATED',chooseNext(candidates,['existing-X','new-A'],1));
assert(verify());const backup=record[1];record[1]={...backup,type:'FORGED'};assert(!verify());record[1]=backup;assert(verify());
const report={status:'PASS',prior,threshold:MIN_GAIN_PER_COST,rankings:base.map(x=>({id:x.id,eligible:x.eligible,gain:Number(x.gain.toFixed(6)),gainPerCost:Number(x.efficiency.toFixed(6)),reason:x.reason})),highestRawGain:maxRaw.id,highestGainPerCost:base[0].id,lowBudgetPick:chooseNext(candidates,['existing-X'],1),afterRootUsed:chooseNext(candidates,['existing-X','new-A'],1),journalEntries:record.length,notes:'Synthetic likelihoods and costs; conditional independence and source-root validity are assumptions. Sequential selection requires reassessment after actual observation.'};
console.log(JSON.stringify(report,null,2));
module.exports={informationGain,rank,chooseNext};
