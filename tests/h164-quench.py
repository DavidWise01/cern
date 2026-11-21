from fractions import Fraction as F
from pathlib import Path
import json
def step(u,s,r,rupture=False):
    q=u/4;u-=q;s+=q
    if rupture:
        release=s/2;s-=release;r+=release
    return u,s,r
initial=steps=triggered=0
for i in range(21):
    for j in range(21-i):
        for mode in (False,True):
            u,s,r=F(i,20),F(j,20),F(0)
            total=u+s+r;initial+=1
            for t in range(12):
                rupture=mode and t==6 and s+u/4>=F(1,4)
                u,s,r=step(u,s,r,rupture)
                assert u>=0 and s>=0 and r>=0 and u+s+r==total
                triggered+=int(rupture);steps+=1
assert (initial,steps,triggered)==(462,5544,211)
report={'scope':'illustrative toy model, not physical cosmology','initial_configurations':initial,'exact_updates':steps,'rupture_events':triggered,'violations':0,'quench_law':'U′=3U/4; S′=S+U/4','rupture_law':'S′=S/2; R′=R+S/2','conserved':'U+S+R'}
Path(__file__).with_name('results.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps(report))
