from fractions import Fraction as F
from itertools import product
from pathlib import Path
import json

RATES=tuple(F(i,8) for i in range(8))
KAPPAS=tuple(F(i,8) for i in range(8))
ETAS=(F(1,4),F(1,2),F(3,4),F(1))
LOADS=(F(0),F(1,8),F(1,4),F(1,2))
DRAG=F(1,16)

def step(st,incoming,kappa,eta,load):
    supplied,rotor,work,heat,ejecta=st
    coupled=incoming*kappa
    ejecta+=incoming-coupled
    mechanical=coupled*eta
    heat+=coupled-mechanical
    rotor+=mechanical
    delivered=load*rotor
    rotor-=delivered
    work+=delivered
    drag=DRAG*rotor
    rotor-=drag
    heat+=drag
    supplied+=incoming
    assert supplied==rotor+work+heat+ejecta
    assert all(v>=0 for v in (supplied,rotor,work,heat,ejecta))
    return supplied,rotor,work,heat,ejecta

runs=[]
updates=arrivals=0
for rate,kappa,eta,load in product(RATES,KAPPAS,ETAS,LOADS):
    state=(F(0),)*5
    for t in range(16):
        q=rate if t%2==0 else F(0)
        state=step(state,q,kappa,eta,load)
        arrivals+=int(q>0)
        updates+=1
    supplied,rotor,work,heat,ejecta=state
    eff=work/supplied if supplied else F(0)
    runs.append(dict(rate=str(rate),kappa=str(kappa),eta=str(eta),
        load=str(load),input=str(supplied),work=str(work),
        stored=str(rotor),heat=str(heat),ejecta=str(ejecta),
        efficiency=str(eff),efficiency_decimal=round(float(eff),8)))
assert (len(runs),updates,arrivals)==(1024,16384,7168)
assert all(F(v['work'])==0 for v in runs if v['rate']=='0' or v['kappa']=='0' or v['load']=='0')
assert all(F(v['work'])<=F(v['input']) for v in runs)
best=max(runs,key=lambda v:F(v['efficiency']))
report=dict(title='H172 — Field Coupling & Efficiency',
  model='Synthetic open-system energy conversion; does not establish available CMB power',
  configurations=len(runs),exact_updates=updates,positive_input_events=arrivals,
  conservation_failures=0,drag_fraction=str(DRAG),best=best,
  operating_points=[v for v in runs if v['rate']=='1/2' and v['eta']=='3/4' and v['load']=='1/4'])
Path(__file__).with_name('results.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps({k:v for k,v in report.items() if k!='operating_points'},indent=2))
