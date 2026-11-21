from fractions import Fraction as F
from pathlib import Path
import json
I=F(2);J=F(1,20)
def K(L): return L*L/(2*I)
def advance(s,charge,extract,drag):
    fuel,lr,lh,le,work,heat,shots=s
    if charge:
        cost=K(lr+J)+K(lh-J)-K(lr)-K(lh)
        if 0<=cost<=fuel:
            fuel-=cost;lr+=J;lh-=J;shots+=1
    if extract:
        old=K(lr);next_lr=lr*F(7,8)
        le+=lr-next_lr;lr=next_lr;work+=old-K(lr)
    if drag:
        old=K(lr)+K(lh);transfer=lr*F(1,16)
        lr-=transfer;lh+=transfer
        loss=old-K(lr)-K(lh)
        assert loss>=0
        heat+=loss
    return fuel,lr,lh,le,work,heat,shots
counts=dict(initial_states=0,updates=0,charge_events=0,work_events=0,heat_events=0,conservation_failures=0)
for k in range(13):
 for mask in range(8):
    s=(F(k,10),F(0),F(0),F(0),F(0),F(0),0)
    counts['initial_states']+=1;initial=s[0]
    for n in range(12):
        before=s;s=advance(s,bool(mask&1),bool(mask&2),bool(mask&4))
        fuel,lr,lh,le,work,heat,shots=s
        assert fuel>=0 and work>=0 and heat>=0
        assert lr+lh+le==0
        assert fuel+K(lr)+K(lh)+work+heat==initial
        counts['charge_events']+=shots-before[-1]
        counts['work_events']+=int(work>before[4])
        counts['heat_events']+=int(heat>before[5])
        counts['updates']+=1
assert counts==dict(initial_states=104,updates=1248,charge_events=564,work_events=288,heat_events=288,conservation_failures=0)
print(json.dumps(counts,indent=2))
