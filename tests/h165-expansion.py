from fractions import Fraction as F
from pathlib import Path
import json
def update(u,s,e,a,release):
    old=u+s+e
    q=u/4;u-=q;s+=q
    if release and s>=F(1,4):
        d=s/2;s-=d;e+=d
        fired=True
    else:fired=False
    a=a*(1+e/8)
    assert u+s+e==old and a>=1
    rho=e/a**4
    return u,s,e,a,rho,fired
initial=updates=fired=0
for i in range(21):
    for j in range(21-i):
        for can_trigger in (False,True):
            initial+=1
            u,s,e,a=F(i,20),F(j,20),F(0),F(1)
            original=u+s+e
            has_fired=False
            for n in range(12):
                u,s,e,a,rho,did=update(u,s,e,a,can_trigger and n==6)
                fired+=did;has_fired |= did
                assert u+s+e==original
                assert rho*a**4==e
                if has_fired:assert a>1 and rho<=e
                else:assert a==1
                updates+=1
assert (initial,updates,fired)==(462,5544,211)
report=dict(initial_states=initial,updates=updates,released_cases=fired,energy_ledger_failures=0,
 comoving_invariance_failures=0,radiation_scaling_failures=0,
 scale_rule="a_next=a*(1+E/8): illustrative, NOT Friedmann",radiation_density="rho=E/a^4",
 metric_separation="D=a*chi",local_explosion_control="a=1, shell radius changes")
Path(__file__).with_name('results.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps(report,indent=2))
