import math
from itertools import product
D=2-2*math.cos(math.pi/12)+1/576

def step(x,d,f,t):
    return x+f*t if d==0 else x+(f-d*x)*(-math.expm1(-d*t))/d

def limit(x,d,t,r=1):
    return (r-x)/t if d==0 else d*(r-x*math.exp(-d*t))/(-math.expm1(-d*t))

checks=accept=reject=0
for x,d,t,m in product([0,.25,.5,.9,1],[0,D/2,D],[.1,1,5,10],[0,.5,.99,1.01,2]):
    f=m*limit(x,d,t)
    y=step(x,d,f,t)
    allowed=max(x,y)<=1+1e-12
    assert allowed==(m<=1 or f==0)
    committed=y if allowed else x
    if allowed: assert committed<=1+1e-12;accept+=1
    else: assert committed==x;reject+=1
    checks+=1
x=.5
for t in [1,5,10]: print('duration',t,'max force',limit(x,D,t),'ratio',limit(x,D,t)/D)
for f,t in [(2*D,5),(2*D,5),(0,10),(2*D,5)]:
    y=step(x,D,f,t); ok=max(x,y)<=1
    print('pulse',f,t,'before',x,'candidate',y,'accepted',ok)
    if ok:x=y
print('PASS',checks,'accepted',accept,'rejected',reject,'gap',D)
