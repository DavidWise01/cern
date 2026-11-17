'use strict';
const assert=require('node:assert/strict'),c=require('node:crypto');
const hash=x=>c.createHash('sha256').update(JSON.stringify(x)).digest('hex');
const keys=Object.fromEntries(['authority','A','B','C','M'].map(x=>[x,c.generateKeyPairSync('ed25519')]));
const roots={A:'r1',B:'r2',C:'r3',M:'r1'};
const sign=(id,payload)=>({id,payload,signature:c.sign(null,Buffer.from(JSON.stringify(payload)),keys[id].privateKey).toString('base64')});
const verify=x=>!!keys[x?.id]&&c.verify(null,Buffer.from(JSON.stringify(x.payload)),keys[x.id].publicKey,Buffer.from(x.signature,'base64'));
const old={campaign:'H076',epoch:2,checkpoint:'004C',used:['a','b']};
function revoke(state,ticket){const p=ticket.payload;if(ticket.id!=='authority'||!verify(ticket)||p.campaign!==state.campaign||p.from!==hash(state)||p.epoch!==state.epoch+1)throw Error('DENIED');return {...state,epoch:p.epoch,checkpoint:p.checkpoint};}
const command=sign('authority',{campaign:'H076',from:hash(old),epoch:3,checkpoint:'005D'});
const current=revoke(old,command);
const anchor=(id,state)=>sign(id,{campaign:state.campaign,epoch:state.epoch,digest:hash(state)});
const fresh=['A','B','C'].map(id=>anchor(id,current));
function gate(state,packets,challengeEpoch){const valid=packets.filter(p=>roots[p.id]&&verify(p)&&p.payload.campaign===state.campaign);const epoch=Math.max(-1,...valid.map(p=>p.payload.epoch));if(epoch<challengeEpoch)return 'FREEZE_STALE_ANCHOR';const top=valid.filter(p=>p.payload.epoch===epoch);if(new Set(top.map(p=>p.payload.digest)).size>1)return 'FREEZE_CONFLICT';if(new Set(top.map(p=>roots[p.id])).size<3)return 'FREEZE_QUORUM';if(state.epoch<epoch)return 'FREEZE_ROLLBACK';if(state.epoch!==epoch||hash(state)!==top[0].payload.digest)return 'FREEZE_UNANCHORED_STATE';return 'ALLOW';}
const cases=[['current',current,fresh,3,'ALLOW'],['backup',old,fresh,3,'FREEZE_ROLLBACK'],['same epoch edit',{...current,used:[]},fresh,3,'FREEZE_UNANCHORED_STATE'],['mirror',current,[fresh[0],fresh[1],anchor('M',current)],3,'FREEZE_QUORUM'],['conflict',current,[...fresh,anchor('M',{...current,used:[]})],3,'FREEZE_CONFLICT'],['ahead',{...current,epoch:4},fresh,3,'FREEZE_UNANCHORED_STATE'],['stale anchors',old,['A','B','C'].map(id=>anchor(id,old)),3,'FREEZE_STALE_ANCHOR'],['no anchors',current,[],3,'FREEZE_STALE_ANCHOR'],['forged',current,[fresh[0],fresh[1],{...fresh[2],payload:{...fresh[2].payload,digest:'fake'}}],3,'FREEZE_QUORUM']];
for(const [name,s,p,e,want] of cases)assert.equal(gate(s,p,e),want,name);
assert.throws(()=>revoke(old,sign('A',command.payload)),/DENIED/);assert.throws(()=>revoke(current,command),/DENIED/);
console.log(JSON.stringify({status:'PASS',cases:cases.map(([name,s,p,e])=>[name,gate(s,p,e)]),revocationChecks:2,assumption:'trusted external epoch floor and independently maintained anchors; simulated backup objects, not actual database restoration'},null,2));
