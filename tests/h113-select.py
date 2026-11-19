import math, random, itertools, json
D=2-2*math.cos(math.pi/12)+1/576
R=1.; T=20

def peak(x,now,pulses,d):
    times=sorted({now,T,*[t for a,b,f,w in pulses for t in (a,b) if now<t<T]})
    m=x
    for a,b in zip(times,times[1:]):
        f=sum(f for s,e,f,w in pulses if s<=a<e)
        x=x+(f-d*x)*(-math.expm1(-d*(b-a)))/d if d else x+f*(b-a)
        m=max(m,x)
    return m

def select(x,now,p,d):
    best=(-1,-1,()); chosen=[]
    for bits in itertools.product((0,1),repeat=len(p)):
        q=[v for v,z in zip(p,bits) if z]
        score=(sum(v[3] for v in q),len(q),bits)
        if score>best and peak(x,now,q,d)<=R-1e-12: best=score;chosen=q
    return chosen

rng=random.Random(113); stats=dict(cases=256,full_safe=0,partial=0,none=0,baseline_value=0,selected_value=0,baseline_count=0,selected_count=0,unsafe=0,subsets=0)
for _ in range(stats['cases']):
    x=rng.uniform(.1,.95); p=[]
    for j in range(8):
        a=rng.randrange(0,19); b=rng.randrange(a+1,21); f=rng.uniform(0,2*D); w=rng.randrange(1,11)
        q=p+[(a,b,f,w)]
        if peak(x,0,q,D)<=R-1e-12:p=q
    # Recover observed scalar envelope at the cut without rewriting history.
    cut=rng.randrange(1,15); times=sorted({0,cut,*[t for a,b,f,w in p for t in (a,b) if 0<t<cut]})
    for a,b in zip(times,times[1:]):
        f=sum(f for s,e,f,w in p if s<=a<e);x+=(f-D*x)*(-math.expm1(-D*(b-a)))/D
    p=[v for v in p if v[1]>cut]; d=D*rng.choice([0,.1,.5,1]); q=select(x,cut,p,d)
    base=p if peak(x,cut,p,d)<=R-1e-12 else []
    assert peak(x,cut,q,d)<=R and sum(v[3] for v in q)>=sum(v[3] for v in base)
    stats['subsets']+=2**len(p);stats['baseline_value']+=sum(v[3] for v in base);stats['selected_value']+=sum(v[3] for v in q)
    stats['baseline_count']+=len(base);stats['selected_count']+=len(q)
    stats['full_safe' if len(q)==len(p) else 'partial' if q else 'none']+=1
# Concrete counterexample: three equal simultaneous requests, priorities 3,2,1.
p=[(0,10,.035,3),(0,10,.035,2),(0,10,.035,1)]
q=select(.2,0,p,0);assert len(q)==2 and sum(v[3] for v in q)==5
stats['example']={'all_peak':peak(.2,0,p,0),'selected_peak':peak(.2,0,q,0),'value':5}
print(json.dumps(stats,indent=2))
