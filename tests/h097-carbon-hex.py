import sympy as s
n=7
L=s.zeros(n)
for a,b in [(i,(i+1)%6) for i in range(6)]+[(i,6) for i in range(6)]:
 L[a,a]+=1;L[b,b]+=1;L[a,b]-=1;L[b,a]-=1
p=s.Matrix([-1,1,-1,1,-1,1,0]); assert L*p==5*p
assert L.eigenvals()=={0:1,2:2,4:2,5:1,7:1}
assert L*s.ones(7,1)==s.zeros(7,1)
# rotation and antipodal conservation
R=s.zeros(7);R[6,6]=1
for i in range(6):R[(i+1)%6,i]=1
assert R**6==s.eye(7) and R**3*p==-p and R*L==L*R
# exact symbolic center coupling
k=s.symbols('k', nonnegative=True);t=s.Symbol('t')
M=s.diag(*([2+k]*6),6*k)
for i in range(6):M[i,(i+1)%6]=-1;M[(i+1)%6,i]=-1;M[i,6]=M[6,i]=-k
assert s.factor(M.charpoly(t).as_expr().subs(M.charpoly(t).gen,t)-t*(t-7*k)*(t-k-1)**2*(t-k-3)**2*(t-k-4))==0
print('PASS: spectrum',L.eigenvals(),'polarity eigenvalue 5; symbolic coupling polynomial verified')
