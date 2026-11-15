'use strict';
// H043: Synthetic signed packets, replay protection, and source-root lineage.
const assert = require('node:assert/strict');
const crypto = require('node:crypto');
const CURRENT = 1000, WINDOW = 20, CHECKPOINT = 'H043|cube-rotation:7|layer:visible|rank:40319';
const secrets = new Map([['A','test-key-A'],['B','test-key-B'],['C','test-key-C'],['D','test-key-D'],['M','test-key-M']]);
const canonical = p => JSON.stringify([p.id,p.root,p.ancestor,p.checkpoint,p.epoch,p.nonce]);
const sign = p => crypto.createHmac('sha256', secrets.get(p.root)).update(canonical(p)).digest('hex');
function make(root,id,ancestor=root,epoch=CURRENT,nonce=id,checkpoint=CHECKPOINT){
 const p={id,root,ancestor,checkpoint,epoch,nonce};
 return {...p,signature:sign(p)};
}
function verifyPacket(p,seen){
 if(!secrets.has(p.root))return 'unknown-root';
 const expected=Buffer.from(sign(p),'hex'), actual=/^[a-f0-9]{64}$/.test(p.signature)?Buffer.from(p.signature,'hex'):Buffer.alloc(0);
 if(actual.length!==expected.length||!crypto.timingSafeEqual(actual,expected))return 'bad-signature';
 if(!Number.isSafeInteger(p.epoch)||p.epoch>CURRENT||CURRENT-p.epoch>WINDOW)return 'stale-or-future';
 if(p.checkpoint!==CHECKPOINT)return 'checkpoint-mismatch';
 const replayKey=p.root+'|'+p.nonce;
 if(seen.has(replayKey))return 'replay';
 seen.add(replayKey);return 'valid';
}
function check(packets){
 const seen=new Set(),valid=[],rejections=[];
 for(const p of packets){const result=verifyPacket(p,seen);if(result==='valid')valid.push(p);else rejections.push({id:p.id,reason:result});}
 const distinctRoots=new Set(valid.map(p=>p.root)).size;
 const distinctAncestry=new Set(valid.map(p=>p.ancestor)).size;
 const accepted=valid.length>=3&&distinctRoots>=3&&distinctAncestry>=3;
 return {accepted,valid:valid.length,distinctRoots,distinctAncestry,rejections};
}
const cases=[
 ['independent', [make('A','w1'),make('B','w2'),make('C','w3')], true],
 ['forged-payload', (()=>{const p=make('B','forged');return [make('A','w1'),{...p,checkpoint:'altered'},make('C','w3')];})(),false],
 ['invalid-signature', [make('A','w1'),{...make('B','w2'),signature:'0'.repeat(64)},make('C','w3')],false],
 ['replayed-packet', [make('A','w1'),make('B','w2'),make('B','w2'),make('C','w3')],true],
 ['stale', [make('A','w1'),make('B','w2','B',CURRENT-WINDOW-1),make('C','w3')],false],
 ['future', [make('A','w1'),make('B','w2','B',CURRENT+1),make('C','w3')],false],
 ['shared-ancestor', [make('A','w1','M'),make('B','w2','M'),make('C','w3','C')],false],
 ['single-root-mirrors', [make('A','w1'),make('A','w2'),make('A','w3')],false],
 ['unknown-root', [make('A','w1'),{id:'w2',root:'Z',ancestor:'Z',checkpoint:CHECKPOINT,epoch:CURRENT,nonce:'w2',signature:'0'.repeat(64)},make('C','w3')],false],
 ['mismatched-checkpoint', [make('A','w1'),make('B','w2'),make('C','w3', 'C',CURRENT,'w3','other')],false],
 ['colluding-distinct-roots-undetectable', [make('A','w1'),make('B','w2'),make('C','w3')],true],
 ['four-with-forgery', [make('A','w1'),make('B','w2'),make('C','w3'),{...make('D','w4'),signature:'0'.repeat(64)}],true],
 ['three-valid-but-shared-ancestry', [make('A','w1','M'),make('B','w2','B'),make('C','w3','M')],false],
];
for(const [name,packets,expected] of cases){
 const r=check(packets);assert.equal(r.accepted,expected,name);
 if(name==='replayed-packet')assert(r.rejections.some(x=>x.reason==='replay'));
 if(name==='shared-ancestor')assert.equal(r.distinctAncestry,2);
}
const report={status:'PASS',synthetic_cases:cases.length,accepted:cases.filter(([, ,e])=>e).length,rejected:cases.filter(([, ,e])=>!e).length,limitation:'Signed distinct keys cannot reveal covert collusion or falsely declared ancestry. This test assumes a trusted key registry and trusted root-ancestry mapping; signing the declared ancestor alone does NOT authenticate it.'};
console.log(JSON.stringify(report,null,2));
