from fractions import Fraction as F
from itertools import combinations
import json
from pathlib import Path
names=('wx','wy','wz','xy','xz','yz'); d=(30,18,12,8,5,2); e=10; cap=40; switch=12
L=tuple((cap-e)//x for x in d)
def run(i,l):return e*l+d[i]*l*(l+1)//2+switch
candidates=[(F(run(i,a)+run(j,b),a+b),i,j,a,b) for i,j in combinations(range(6),2) for a in range(1,L[i]+1) for b in range(1,L[j]+1)]
mu,i,j,a,b=min(candidates)
mins=[min(run(k,l)-mu*l for l in range(1,L[k]+1)) for k in range(6)]
assert mins[j]<0 and all(x>=-mins[j] for k,x in enumerate(mins) if k!=j)
seq=([j]*b+[i]*a)*2000;r=[e]*6;peak=0;total=0;changes=0;last=i
for k in seq:
 r=[r[h]+d[h] if h==k else e for h in range(6)];m=max(r);assert m<=cap;peak=max(peak,m);total+=m;changes+=int(k!=last);last=k
assert F(total+switch*changes,len(seq))==mu
out=dict(dwell=L,candidates=len(candidates),mean_cost=str(mu),cycle=[names[k] for k in seq[:a+b]],run_excess=[str(x) for x in mins],generations=len(seq),peak=peak,mean_radius=str(F(total,len(seq))),switches=changes)
print(json.dumps(out,indent=2));Path(__file__).with_name('results.json').write_text(json.dumps(out,indent=2))
