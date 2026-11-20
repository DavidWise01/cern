from fractions import Fraction as F
from itertools import product
from pathlib import Path
import json

def step(x,b):
    a,c,e=x
    return (a/2+b*c,a/4+c/2+b*e,c/4+e/2)

def gain(b):
    # Exact solution of (I-A)g=1.
    m=(3+4*b)/(1-2*b)
    return (2+2*b*m,m,2+m/2)

checks=0; counter=0; rows=[]
for b in map(F,['0','1/8','1/4','3/8','7/16','49/100']):
    g=gain(b); assert tuple(x-y for x,y in zip(g,step(g,b)))==(1,1,1)
    eps=1/max(g); h=tuple(eps*x for x in g)
    for n in range(-36,37):
        s=F(80829,50000)*F(10)**n
        for signs in product((-1,1),repeat=3):
            x=tuple(s*a*z for a,z in zip(h,signs))
            for ds in product((-1,1),repeat=3):
                y=tuple(v+s*eps*z for v,z in zip(step(x,b),ds))
                assert all(abs(v)<=s*a for v,a in zip(y,h)); checks+=1
        # Any larger uniform disturbance eventually violates the unit cap from zero.
        ep=eps+F(1,1000); equilibrium=tuple(ep*v for v in g)
        assert max(equilibrium)>1; counter+=1
    rows.append(dict(beta=str(b),gain=list(map(str,g)),epsilon=str(eps),weights=list(map(str,h))))
out=dict(scales=73,feedback_values=len(rows),corner_checks=checks,above_bound_equilibria=counter,rows=rows)
Path(__file__).with_name('results.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out,indent=2))
