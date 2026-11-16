'use strict';
// H053: unknown pairwise dependency, Pareto stability and append-only recertification.
const assert=require('node:assert/strict'),crypto=require('node:crypto');
const sites=[
{id:'A',cost:2,region:'N',provider:'P1',power:'G1',backbone:'K1'},
{id:'B',cost:2,region:'N',provider:'P2',power:'G2',backbone:'K2'},
{id:'C',cost:3,region:'S',provider:'P1',power:'G2',backbone:'K3'},
{id:'D',cost:3,region:'S',provider:'P3',power:'G3',backbone:'K4'},
{id:'E',cost:4,region:'E',provider:'P4',power:'G4',backbone:'K5'},
{id:'F',cost:5,region:'W',provider:'P5',power:'G5',backbone:'K6'}];
const choose=(a,k)=>k===0?[[]]:a.length<k?[]:[...choose(a.slice(1),k-1).map(x=>[a[0],...x]),...choose(a.slice(1),k)];
const layers=['region','provider','power','backbone'];
function cut(chosen,added=[]){const deps=chosen.map(s=>new Set([...layers.map(k=>k+':'+s[k]),...added.filter(d=>d.members.includes(s.id)).map(d=>'new:'+d.id)]));const domains=[...new Set(deps.flatMap(x=>[...x]))];let best=Infinity;
for(let mask=0;mask<2**domains.length;mask++){let n=0;for(let j=0;j<domains.length;j++)n+=(mask>>j)&1;if(n>=best)continue;let live=0;for(const rootDeps of deps){let dead=false;for(let j=0;j<domains.length;j++)if((mask>>j)&1&&rootDeps.has(domains[j])){dead=true;break}if(!dead)live++;}if(live<3)best=n;}return best;}
const pairs=a=>choose(a,2).map(p=>({id:'link-'+p.map(x=>x.id).join(''),members:p.map(x=>x.id)}));
const choices=choose(sites,4).map(s=>({ids:s.map(x=>x.id),cost:s.reduce((n,x)=>n+x.cost,0),nominal:cut(s),worstOneUnknownPair:Math.min(...pairs(s).map(p=>cut(s,[p])))}));
const find=id=>choices.find(c=>c.ids.join('')===id),best=(budget,metric)=>choices.filter(c=>c.cost<=budget).sort((a,b)=>b[metric]-a[metric]||a.cost-b.cost||a.ids.join('').localeCompare(b.ids.join('')))[0]??null;
assert.equal(choices.length,15);assert.deepEqual([find('ABCD').cost,find('ABCD').nominal,find('ABCD').worstOneUnknownPair],[10,1,1]);assert.deepEqual([find('ADEF').cost,find('ADEF').nominal,find('ADEF').worstOneUnknownPair],[14,2,1]);assert(choices.every(x=>x.worstOneUnknownPair===1));assert.equal(best(14,'nominal').ids.join(''),'ADEF');assert.equal(best(14,'worstOneUnknownPair').ids.join(''),'ABCD');
const discovery={id:'H053-D1',members:['A','D'],reason:'synthetic common backbone'};
const original={id:'H052',selection:'ADEF',cut:2},revised={id:'H053',parent:'H052',selection:'ADEF',cut:cut(sites.filter(x=>'ADEF'.includes(x.id)),[discovery])};assert.equal(revised.cut,1);
const hash=x=>crypto.createHash('sha256').update(JSON.stringify(x)).digest('hex'),journal=[];
function append(record){const previous=journal.length?journal.at(-1).hash:'0'.repeat(64);const entry={sequence:journal.length,previous,record};journal.push({...entry,hash:hash(entry)});}
function verify(){let prev='0'.repeat(64);return journal.every((x,i)=>{const ok=x.sequence===i&&x.previous===prev&&x.hash===hash({sequence:i,previous:prev,record:x.record});prev=x.hash;return ok;});}
append({type:'ORIGINAL_CERTIFICATE',data:original});append({type:'DEPENDENCY_DISCOVERY',data:discovery});append({type:'REVISED_CERTIFICATE',data:revised});assert(verify());assert.deepEqual(journal[0].record.data,original);const tampered=JSON.parse(JSON.stringify(journal));tampered[0].record.data.cut=1;assert.notEqual(tampered[0].hash,hash({sequence:0,previous:tampered[0].previous,record:tampered[0].record}));
console.log(JSON.stringify({status:'PASS',selections:15,nominalWinner:best(14,'nominal'),robustWinner:best(14,'worstOneUnknownPair'),newCut:revised.cut,journalEntries:journal.length,limitations:'Synthetic adversarial uncertainty; hash chain is in-memory and unauthenticated'},null,2));
