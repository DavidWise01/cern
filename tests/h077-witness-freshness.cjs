'use strict';
const assert=require('node:assert/strict'), c=require('node:crypto');
const keys=Object.fromEntries(['A','B','C','M'].map(id=>[id,c.generateKeyPairSync('ed25519')]));
const roots=new Map([['A','r1'],['B','r2'],['C','r3'],['M','r1']]);
const hash=x=>c.createHash('sha256').update(JSON.stringify(x)).digest('hex');
const current={epoch:3,digest:hash('state-3')}, old={epoch:2,digest:hash('state-2')};
const challenge=now=>({campaign:'H077',nonce:c.randomBytes(32).toString('hex'),issued:now,deadline:now+10});
const sign=(id,q,state=current)=>{const payload={id,...q,...state};return {payload,signature:c.sign(null,Buffer.from(JSON.stringify(payload)),keys[id].privateKey).toString('base64')}};
const responses=(q,state)=>['A','B','C'].map(id=>sign(id,q,state));
const consumed=new Set();
function gate(q,packets,now,local,floor){
 if(now<q.issued||now>q.deadline)return 'EXPIRED';
 if(consumed.has(q.nonce))return 'REPLAY';
 const valid=packets.filter(({payload:p,signature})=>roots.has(p.id)&&['campaign','nonce','issued','deadline'].every(k=>p[k]===q[k])&&Number.isSafeInteger(p.epoch)&&p.epoch>=0&&typeof p.digest==='string'&&c.verify(null,Buffer.from(JSON.stringify(p)),keys[p.id].publicKey,Buffer.from(signature,'base64')));
 if(new Set(valid.map(p=>JSON.stringify([p.payload.epoch,p.payload.digest]))).size>1)return 'CONFLICT';
 if(new Set(valid.map(p=>roots.get(p.payload.id))).size<3)return 'QUORUM';
 const p=valid[0].payload;
 if(p.epoch<floor)return 'STALE';
 if(p.epoch!==local.epoch||p.digest!==local.digest)return 'MISMATCH';
 consumed.add(q.nonce);return 'ALLOW';
}
const q=challenge(100), fresh=responses(q), q2=challenge(101);
const results={};function check(name,actual,expected){assert.equal(actual,expected,name);results[name]=actual;}
check('fresh',gate(q,fresh,100,current,3),'ALLOW');
check('same_challenge_replay',gate(q,fresh,101,current,3),'REPLAY');
check('old_packets_new_challenge',gate(q2,fresh,101,current,3),'QUORUM');
check('expired',gate(q2,responses(q2),112,current,3),'EXPIRED');
check('before_issued',gate(q2,responses(q2),100,current,3),'EXPIRED');
check('mirror',gate(q2,['A','B','M'].map(id=>sign(id,q2)),102,current,3),'QUORUM');
const forged=structuredClone(sign('C',q2));forged.payload.digest='changed';
check('forged',gate(q2,[sign('A',q2),sign('B',q2),forged],102,current,3),'QUORUM');
check('conflict',gate(q2,[...responses(q2),sign('M',q2,old)],102,current,3),'CONFLICT');
check('fresh_but_stale',gate(q2,responses(q2,old),102,old,3),'STALE');
check('local_rollback',gate(q2,responses(q2),102,old,3),'MISMATCH');
check('same_epoch_edit',gate(q2,responses(q2),102,{...current,digest:hash('edited')},3),'MISMATCH');
check('new_challenge',gate(q2,responses(q2),102,current,3),'ALLOW');
console.log(JSON.stringify({status:'PASS',checks:Object.keys(results).length,results},null,2));
