from fractions import Fraction as F
from itertools import product
from pathlib import Path
import json
RATES=[F(i,8) for i in range(8)]
CAPTURES=[F(1,4),F(1,2),F(3,4),F(1)]
PROFILES=['rotate','alternate','pulse','static']
MIXES=[F(0),F(1,4),F(1,2),F(3,4)]
POLICIES=['equal','fixed_a','greedy','proportional']
DRAG=F(1,16)
def loads(profile,t):
 if profile=='rotate':return tuple(F(3,4) if i==t%3 else F(1,8) for i in range(3))
 if profile=='alternate':return (F(3,4),F(1,8),F(1,2)) if t%2==0 else (F(1,8),F(3,4),F(1,2))
 if profile=='pulse':return (F(3,4),F(0),F(0)) if t%4<2 else (F(0),F(3,4),F(0))
 return (F(1,2),F(1,4),F(1,8))
def alloc(policy,L):
 if policy=='equal':return (F(1,3),)*3
 if policy=='fixed_a':return (F(1),F(0),F(0))
 if policy=='greedy':
  i=max(range(3),key=lambda i:(L[i],-i))
  return tuple(F(int(j==i)) for j in range(3))
 s=sum(L)
 return tuple(v/s if s else F(1,3) for v in L)
def run(rate,capture,profile,mix,policy):
 st=[F(0),F(0),F(0)];W=H=X=source=F(0);events=transfers=0
 for t in range(16):
  L=loads(profile,t);f=alloc(policy,L)
  q=rate if t%2==0 else F(0);source+=q
  captured=capture*q;X+=q-captured
  st=[st[i]+captured*f[i] for i in range(3)]
  if mix:
   mean=sum(st)/3
   ns=[(1-mix)*z+mix*mean for z in st]
   transfers+=sum(int(ns[i]!=st[i]) for i in range(3))
   st=ns
  for i in range(3):
   earned=st[i]*L[i];W+=earned;st[i]-=earned
   loss=st[i]*DRAG;st[i]-=loss;H+=loss
  assert source==sum(st)+W+H+X
  assert all(z>=0 for z in st)
  events+=int(q>0)
 return dict(eff=W/source if source else F(0),source=source,work=W,heat=H,ejecta=X,stored=sum(st),events=events,transfers=transfers)
rows=[];checks=positive=transfers=0
for rate,capture,profile,mix,policy in product(RATES,CAPTURES,PROFILES,MIXES,POLICIES):
 r=run(rate,capture,profile,mix,policy)
 checks+=16;positive+=r['events'];transfers+=r['transfers']
 rows.append((rate,capture,profile,mix,policy,r))
assert len(rows)==2048 and checks==32768 and positive==14336
assert all(r['work']==0 for rate,cap,profile,mix,p,r in rows if rate==0)
assert all(r['eff']<=1 for *_,r in rows)
best=max(rows,key=lambda row:row[-1]['eff'])
gains=[]
for rate,capture,profile,mix in product(RATES,CAPTURES,PROFILES,MIXES):
 each={p:next(r['eff'] for a,b,c,d,pp,r in rows if (a,b,c,d)==(rate,capture,profile,mix) and pp==p) for p in POLICIES}
 gains.append((each['greedy'],each['equal'],each['fixed_a']))
winning=sum(a>max(b,c) for a,b,c in gains)
assert winning==224 and len(gains)==512
summary=dict(configurations=len(rows),updates=checks,positive_input_steps=positive,mixing_cell_changes=transfers,violations=0,
 best=dict(rate=str(best[0]),capture=str(best[1]),profile=best[2],mix=str(best[3]),policy=best[4],efficiency_exact=str(best[5]['eff']),efficiency_percent=round(float(best[5]['eff'])*100,5)),
 greedy_beats_both_fixed_schemes=winning,comparisons=len(gains),
 limitations=['Synthetic energy model, not a demonstrated engine','Unbounded Rupert population does not create infinite input power','Does not establish extractable energy from CMB','No mechanical angular momentum or phase model'])
Path(__file__).with_name('results.json').write_text(json.dumps(summary,indent=2)+'\n')
print(json.dumps(summary,indent=2))
