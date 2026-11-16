'use strict';
const assert=require('node:assert/strict');
const crypto=require('node:crypto');
const sha=x=>crypto.createHash('sha256').update(JSON.stringify(x)).digest('hex');
const models=[
 {id:'m1',validation:0.70,holdout:0.69},
 {id:'m2',validation:0.73,holdout:0.68},
 {id:'m3',validation:0.76,holdout:0.64},
 {id:'m4',validation:0.82,holdout:0.61},
 {id:'m5',validation:0.79,holdout:0.65},
 {id:'m6',validation:0.78,holdout:0.67}
];
const seal=sha(models.map(({id,holdout})=>[id,holdout]));
class Protocol{
 constructor(){this.phase='DEVELOP';this.selected=null;this.used=false;this.log=[];this.holdoutSeal=seal;this.append('INIT',{holdoutSeal:seal});}
 append(type,data){const index=this.log.length,previous=index?this.log.at(-1).digest:'0'.repeat(64),body={index,previous,type,data:structuredClone(data)};this.log.push({...body,digest:sha(body)});}
 search(){if(this.phase!=='DEVELOP')throw Error('selection locked');const ranking=models.map(x=>({id:x.id,validation:x.validation})).sort((a,b)=>b.validation-a.validation||a.id.localeCompare(b.id));this.append('VALIDATION_SEARCH',{ranking});return ranking;}
 peek(){this.append('REJECTED_PEEK',{phase:this.phase});throw Error('sealed holdout: no peek');}
 freeze(id){if(this.phase!=='DEVELOP')throw Error('freeze only before evaluation');if(!models.some(m=>m.id===id))throw Error('unknown model');this.selected=id;this.phase='FROZEN';this.append('FREEZE',{id});}
 change(id){this.append('REJECTED_RETUNE',{id,phase:this.phase});throw Error('model selection locked');}
 evaluate(){if(this.phase!=='FROZEN'||this.used){this.append('REJECTED_REUSE',{phase:this.phase});throw Error('holdout access unavailable');}if(sha(models.map(({id,holdout})=>[id,holdout]))!==this.holdoutSeal)throw Error('holdout seal mismatch');const model=models.find(m=>m.id===this.selected);this.used=true;this.phase='REPORTED';const result={id:model.id,score:model.holdout,validation:model.validation,selectionGap:+(model.validation-model.holdout).toFixed(6),holdoutRoot:'FINAL-H',holdoutSeal:this.holdoutSeal};this.append('FINAL_ONCE',result);return result;}
 static verify(log){let previous='0'.repeat(64);return log.every((e,index)=>{const ok=e.index===index&&e.previous===previous&&e.digest===sha({index,previous,type:e.type,data:e.data});previous=e.digest;return ok;});}
}
const p=new Protocol();
assert.throws(()=>p.peek(),/sealed holdout/);
const rank=p.search();assert.equal(rank[0].id,'m4');
assert.equal(rank[0].validation,.82);assert.equal(models.find(x=>x.id==='m4').holdout,.61);
p.freeze('m4');assert.throws(()=>p.search(),/selection locked/);
assert.throws(()=>p.change('m1'),/locked/);
const final=p.evaluate();assert.deepEqual({score:final.score,gap:final.selectionGap},{score:.61,gap:.21});
assert.throws(()=>p.evaluate(),/unavailable/);assert.throws(()=>p.freeze('m1'),/before evaluation/);
assert(Protocol.verify(p.log));
const altered=structuredClone(p.log);altered.find(x=>x.type==='FINAL_ONCE').data.score=.99;assert(!Protocol.verify(altered));
const fresh=new Protocol();fresh.freeze('m1');assert.equal(fresh.evaluate().score,.69);
assert.equal(p.log.filter(x=>x.type==='FINAL_ONCE').length,1);
assert.equal(fresh.log.filter(x=>x.type==='FINAL_ONCE').length,1);
const output={status:'PASS',candidates:models.length,validationWinner:'m4',selectedValidation:.82,selectedHoldout:.61,gap:.21,comparisonNotSelectableAfterFeedback:{model:'m1',holdout:.69},heldoutAccesses:1,blockedPeek:true,blockedReuse:true,blockedRetune:true,blockedSelectionAfterFreeze:true,auditEntries:p.log.length,holdoutSeal:seal,caveat:'Synthetic scores demonstrate protocol behavior, not a statistical estimate of generalization.'};
console.log(JSON.stringify(output,null,2));
module.exports={Protocol,models,seal};