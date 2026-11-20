import numpy as np
from scipy.optimize import brentq
from pathlib import Path
import json
mu=.01
m=np.array([[-mu,0.,0.],[1-mu,0.,0.]])
w=np.array([0.,0.,1.])
def force(r,m,w):
    d=r-m
    return -sum(a*v/np.linalg.norm(v)**3 for a,v in zip([1-mu,mu],d))-np.cross(w,np.cross(w,r))
f=lambda x:force(np.array([x,0.,0.]),m,w)[0]
p=np.array([[brentq(f,a,b),0,0] for a,b in [(-mu+1e-6,1-mu-1e-6),(1-mu+1e-6,3),(-3,-mu-1e-6)]]+[[.5-mu,np.sqrt(3)/2,0],[.5-mu,-np.sqrt(3)/2,0]])
checks=0; residual=0.;work=0.
for a in np.linspace(0,2*np.pi,73):
    for b in np.linspace(-np.pi/2,np.pi/2,5):
        c,s=np.cos(a),np.sin(a); C,S=np.cos(b),np.sin(b)
        R=np.array([[c,-s,0],[s,c,0],[0,0,1]])@np.array([[1,0,0],[0,C,-S],[0,S,C]])
        assert np.allclose(R.T@R,np.eye(3)) and np.linalg.det(R)>0
        for r in p:
            q=R@r; z=R@w; v=R@np.array([.2,-.3,.4])
            residual=max(residual,float(np.linalg.norm(force(q,m@R.T,z))))
            work=max(work,abs(float(v@np.cross(z,v))))
            assert np.linalg.norm(force(q,m@R.T,z))<1e-9
            assert abs(v@np.cross(z,v))<1e-12
            checks+=1
out=dict(bodies=3,primaries=2,test_bodies=1,points=5,orientation_checks=checks,max_residual=residual,max_gyro_work=work,points_xyz=p.tolist())
Path(__file__).with_name('results.json').write_text(json.dumps(out,indent=2));print(json.dumps(out,indent=2))
