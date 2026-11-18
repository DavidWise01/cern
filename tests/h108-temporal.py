import math
from itertools import product
D=2-2*math.cos(math.pi/12)+1/576
# Exact scalar envelope for piecewise constant forcing and damping.
def step(x,d,f,t):
    return x+f*t if d==0 else x+(f-d*x)*(-math.expm1(-d*t))/d
checks=0
for x,d,f,t in product([0,.5,1],[0,D/2,D],[0,D/2,D,2*D],[.1,1,10,50]):
    y=step(x,d,f,t)
    assert y>=0
    assert abs(step(step(x,d,f,t/2),d,f,t/2)-y)<1e-12
    if f<=d and x<=1: assert y<=1+1e-12
    checks+=1
for name,x,d,f,t in [('short overload',.5,D,2*D,1),('long overload',.5,D,2*D,10),('gap outage',.5,0,D,10),('recovery',.9,D,0,10)]:
    y=step(x,d,f,t); print(name,format(y,'.12f'), 'contained' if max(x,y)<=1 else 'breach')
T=math.log(1.5)/D
assert abs(step(.5,D,2*D,T)-1)<1e-12
print('PASS',checks,'segment cases; crossing time',T,'gap',D)
# Equal unweighted total exposure and recovery does not guarantee containment.
peak=step(1,D,2*D,1)
end=step(peak,D,0,1)
assert peak>1 and end<1
print('equal-total counterexample',peak,end)
