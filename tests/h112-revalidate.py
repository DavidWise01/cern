import math, random
D=2-2*math.cos(math.pi/12)+1/576

def envelope(x,now,end,pulses,gaps):
    times=sorted({now,end,*[t for a,b,f in pulses+gaps for t in (a,b) if now<t<end]})
    out=[(now,x)]
    for a,b in zip(times,times[1:]):
        d=next(v for s,e,v in gaps if s<=a<e)
        f=sum(v for s,e,v in pulses if s<=a<e)
        x=x+(f-d*x)*(-math.expm1(-d*(b-a)))/d if d else x+f*(b-a)
        out.append((b,x))
    return out

def peak(x,now,end,p,g):return max(v for t,v in envelope(x,now,end,p,g))
# Already admitted pulse; at t=4, loss of all damping.
p=[(0,10,D)]
x=envelope(.5,0,4,p,[(0,20,D)])[-1][1]
old=peak(x,4,20,p,[(4,20,D)])
new=peak(x,4,20,p,[(4,20,0)])
assert old<1<new
print('counterexample',x,old,new)
# A policy that can stop controllable future forcing: preserve active commitments only if safe.
rng=random.Random(112); retained=paused=0
for _ in range(256):
    x=rng.uniform(.1,.95); p=[]
    for j in range(6):
        a=rng.randrange(0,19);b=rng.randrange(a+1,21);f=rng.uniform(0,2*D)
        q=p+[(a,b,f)]
        if peak(x,0,20,q,[(0,20,D)])<=1-1e-12:p=q
    cut=rng.randrange(1,15)
    observed=envelope(x,0,cut,p,[(0,20,D)])[-1][1]
    g=[(cut,20,D*rng.choice([0,.1,.5,1]))]
    if peak(observed,cut,20,p,g)<=1-1e-12:retained+=1; chosen=p
    else:paused+=1;chosen=[]
    assert peak(observed,cut,20,chosen,g)<=1+1e-12
    assert envelope(x,0,cut,p,[(0,20,D)])[-1][1]==observed
print('PASS',retained,paused,'of 256 revalidations')
