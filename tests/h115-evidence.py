from fractions import Fraction as F
from random import Random
from itertools import product
import json
# Discrete Jacobi envelope: x[k+1] = a*x[k] + sum selected loads.
# All coefficients are nonnegative rational numbers.
def fixture(n,rng):
    return F(1,4), F(9,10), [(rng.randrange(6),rng.randrange(1,7),F(rng.randrange(1,9),100),rng.randrange(1,11)) for _ in range(n)]
def matrix(items,a):
    return [[sum((a**(k-1-t)*f for t in range(k) if s<=t<s+l),F(0)) for s,l,f,w in items] for k in range(1,13)]
def solve(x,a,items):
    scale=10**14; assert all((v*scale).denominator==1 for row in matrix(items,a) for v in row); M=[[int(v*scale) for v in row] for row in matrix(items,a)]; w=[i[3] for i in items]; n=len(w); b=[int((1-a**k*x)*scale) for k in range(1,13)]
    best=-1; winner=[]; cert=[]; visits=0
    def dfs(p):
        nonlocal best,winner,visits
        visits+=1; j=len(p); v=sum(w[i]*z for i,z in enumerate(p)); loads=[sum(row[i] for i,z in enumerate(p) if z) for row in M]
        bad=next((k for k in range(12) if loads[k]>b[k]),None)
        if bad is not None: cert.append([p,'unsafe',bad]);return
        ub=v+sum(w[j:])
        if ub<=best: cert.append([p,'value',ub]);return
        if j==n: best=v;winner=p;cert.append([p,'leaf',v]);return
        dfs(p+[1]);dfs(p+[0])
    dfs([]);return best,winner,cert,visits
# Independent verifier: reconstruct all covered branches, recompute constraints.
def verify(x,a,items,best,winner,cert):
    n=len(items);w=[i[3] for i in items]
    def trajectory(p):
        y=x;out=[]
        for t in range(12):
            y=a*y+sum((f*z for (s,l,f,_),z in zip(items,p) if s<=t<s+l),F(0));out.append(y)
        return out
    assert len(winner)==n and all(z in (0,1) for z in winner)
    assert max(trajectory(winner))<=1 and sum(z*v for z,v in zip(winner,w))==best
    records={tuple(p):(kind,k) for p,kind,k in cert};assert len(records)==len(cert)
    used=set()
    def walk(p):
        if p in records:
            used.add(p);kind,k=records[p];v=sum(z*w[i] for i,z in enumerate(p))
            if kind=='unsafe':assert 0<=k<12 and trajectory(p)[k]>1
            elif kind=='value':assert k==v+sum(w[len(p):]) and k<=best
            elif kind=='leaf':assert len(p)==n and k==v and v<=best
            else:raise AssertionError('unknown certificate')
        else:
            assert len(p)<n;walk(p+(0,));walk(p+(1,))
    walk(());assert used==set(records)
    return True
if __name__=='__main__':
    rng=Random(115);stats={'cases':32,'nodes':0,'certificates':0,'tamper_rejected':0}
    for _ in range(32):
        x,a,p=fixture(8,rng);v,z,c,n=solve(x,a,p);assert verify(x,a,p,v,z,c)
        stats['nodes']+=n;stats['certificates']+=len(c)
        try:verify(x,a,p,v,z,c[:-1])
        except AssertionError:stats['tamper_rejected']+=1
    x,a,p=F(1,4),F(9,10),[(0,6,F(1,10),24-i) for i in range(24)];v,z,c,n=solve(x,a,p);assert verify(x,a,p,v,z,c)
    stats['large']={'value':v,'retained':sum(z),'nodes':n,'certificates':len(c),'covered_leaves':sum(2**(24-len(q)) for q,_,_ in c)}
    assert stats['large']['covered_leaves']==2**24
    print(json.dumps(stats,indent=2));open('/mnt/data/h115/results.json','w').write(json.dumps(stats,indent=2))
    open('/mnt/data/h115/certificate.json','w').write(json.dumps({'x':str(x),'a':str(a),'items':[[s,l,str(f),w] for s,l,f,w in p],'best':v,'winner':z,'certificate':c}))
