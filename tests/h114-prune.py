import math, random, itertools, json
D=2-2*math.cos(math.pi/12)+1/576
R=1-1e-12

def peak(x,p,d):
    times=sorted({0,20,*[t for a,b,f,w in p for t in (a,b)]})
    m=x
    for a,b in zip(times,times[1:]):
        f=sum(f for s,e,f,w in p if s<=a<e)
        x=x+(f-d*x)*(-math.expm1(-d*(b-a)))/d if d else x+f*(b-a)
        m=max(m,x)
    return m

def search(x,p,d):
    p=sorted(p,key=lambda v:-v[3]); n=len(p)
    suffix=[sum(v[3] for v in p[i:]) for i in range(n+1)]
    best=-1; chosen=[]; visits=unsafe=bound=0
    def dfs(i,q,value):
        nonlocal best,chosen,visits,unsafe,bound
        visits+=1
        if peak(x,q,d)>R: unsafe+=1; return
        if value+suffix[i]<=best: bound+=1; return
        if i==n: best=value; chosen=q; return
        dfs(i+1,q+[p[i]],value+p[i][3]); dfs(i+1,q,value)
    dfs(0,[],0)
    return best,chosen,visits,unsafe,bound

def exhaustive(x,p,d):
    return max(sum(v[3] for v in q) for bits in itertools.product((0,1),repeat=len(p)) if peak(x,q:=[v for v,z in zip(p,bits) if z],d)<=R)

rng=random.Random(114); stats=dict(cases=0,exhaustive_subsets=0,visited_nodes=0,unsafe_prunes=0,value_prunes=0)
def fixture(n):
    x=rng.uniform(.1,.9); d=D*rng.choice([0,.1,.5,1]); p=[]
    for _ in range(n):
        a=rng.randrange(0,19); b=rng.randrange(a+1,21); p.append((a,b,rng.uniform(0,2*D),rng.randrange(1,11)))
    return x,p,d
for _ in range(128):
    x,p,d=fixture(10); best,q,v,u,b=search(x,p,d)
    assert best==exhaustive(x,p,d) and peak(x,q,d)<=R
    stats['cases']+=1; stats['exhaustive_subsets']+=2**len(p); stats['visited_nodes']+=v; stats['unsafe_prunes']+=u; stats['value_prunes']+=b
x,p,d=fixture(24); best,q,v,u,b=search(x,p,d)
assert peak(x,q,d)<=R
stats['large_case']=dict(n=24,possible_subsets=2**24,visited_nodes=v,unsafe_prunes=u,value_prunes=b,optimal_value=best,retained=len(q),peak=peak(x,q,d))
print(json.dumps(stats,indent=2))
