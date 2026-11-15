'use strict';
// H041 -- exhaustive erasure and Hamming-distance witness robustness
const assert=require('node:assert/strict');
const ps=[[0,1,2],[0,2,1],[1,0,2],[1,2,0],[2,0,1],[2,1,0]];
const parity=p=>((p[0]>p[1])+(p[0]>p[2])+(p[1]>p[2]))%2?-1:1;
const R=[];for(const p of ps)for(const a of [-1,1])for(const b of [-1,1])for(const c of [-1,1])if(parity(p)*a*b*c===1)R.push({p,s:[a,b,c]});
const V=[];for(const x of [-1,1])for(const y of [-1,1])for(const z of [-1,1])V.push([x,y,z]);
const key=x=>JSON.stringify(x),apply=(v,r)=>r.p.map((j,i)=>r.s[i]*v[j]);
const opposite=(a,b)=>a.every((x,i)=>x===-b[i]);
const code=pts=>R.map(r=>pts.map(v=>key(apply(v,r))));
const distance=(a,b)=>a.reduce((s,v,i)=>s+(v!==b[i]),0);
const minDistance=pts=>{const C=code(pts);let d=pts.length;for(let i=0;i<24;i++)for(let j=i+1;j<24;j++)d=Math.min(d,distance(C[i],C[j]));return d};
const combinations=(n,k)=>{const out=[];function go(start,a){if(a.length===k){out.push([...a]);return;}for(let i=start;i<n;i++)go(i+1,[...a,i]);}go(0,[]);return out};
let triples=0,allErasureSafe=0,erasurePass=0,erasureFail=0,detectSafe=0,detectUnsafe=0;
for(const indices of combinations(8,3)){
 triples++;const pts=indices.map(i=>V[i]);const hasOpposite=pts.some((v,i)=>pts.some((w,j)=>j>i&&opposite(v,w)));
 const md=minDistance(pts);assert.equal(md,hasOpposite?1:2);
 if(md>=2)detectSafe++;else detectUnsafe++;
 let safe=true;
 for(let missing=0;missing<3;missing++){const remaining=pts.filter((_,i)=>i!==missing),C=code(remaining);if(new Set(C.map(key)).size===24)erasurePass++;else{erasureFail++;safe=false;}}
 if(safe)allErasureSafe++;
}
let quads=0,oneErrorCorrectable=0,oneErrorUncorrectable=0,exactCorrectionTrials=0;
for(const indices of combinations(8,4)){
 quads++;const pts=indices.map(i=>V[i]);const hasOpposite=pts.some((v,i)=>pts.some((w,j)=>j>i&&opposite(v,w)));
 const C=code(pts),md=minDistance(pts);
 assert.equal(md,hasOpposite?2:3);
 if(md>=3){
  oneErrorCorrectable++;
  // Try all one-symbol substitutions for every rotation on all four coordinates.
  for(let source=0;source<24;source++)for(let slot=0;slot<4;slot++){
    for(const candidate of V){const symbol=key(candidate);if(symbol===C[source][slot])continue;
      const observed=[...C[source]];observed[slot]=symbol;
      const matches=C.map((candidateCode,r)=>({r,d:distance(observed,candidateCode)})).filter(x=>x.d<=1);
      assert.deepEqual(matches,[{r:source,d:1}]); exactCorrectionTrials++;
    }
  }
 }else oneErrorUncorrectable++;
}
assert.deepEqual({triples,allErasureSafe,erasurePass,erasureFail,detectSafe,detectUnsafe,quads,oneErrorCorrectable,oneErrorUncorrectable,exactCorrectionTrials},{triples:56,allErasureSafe:32,erasurePass:144,erasureFail:24,detectSafe:32,detectUnsafe:24,quads:70,oneErrorCorrectable:16,oneErrorUncorrectable:54,exactCorrectionTrials:10752});
console.log(JSON.stringify({status:'PASS',triples,allErasureSafe,erasurePass,erasureFail,detectSafe,detectUnsafe,quads,oneErrorCorrectable,oneErrorUncorrectable,exactCorrectionTrials,scope:'Noiseless discrete cube rotation observations; this proves geometry/error-code properties, not independent evidence provenance.'},null,2));
