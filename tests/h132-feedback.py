from fractions import Fraction as F
from itertools import product
import json
from pathlib import Path
# H131 inward coupling 1/4; H132 outward feedback b.
def step(x,d,b):
 a,c,e=x
 return (a/2+b*c+d[0],a/4+c/2+b*e+d[1],c/4+e/2+d[2])
def scale(n):return F(80829,50000)*F(10)**n
checks=0; failures=0
for n in range(-36,37):
 s=scale(n)
 for b in (F(0),F(1,16),F(1,8),F(1,4)):
  budget=F(1,4)-b
  for x in product((F(-1),F(1)),repeat=3):
   for d in product((-budget,budget),repeat=3):
    y=step(x,d,b)
    assert max(map(abs,y))<=1
    assert step(tuple(s*v for v in x),tuple(s*v for v in d),b)==tuple(s*v for v in y)
    checks+=1
  # A disturbance only 1/1000 above the sharp limit violates containment.
  d=(budget+F(1,1000),)*3
  assert step((F(1),)*3,d,b)[1]>1
  failures+=1
# Characteristic polynomial: (lambda-1/2)*((lambda-1/2)^2-b/2).
# Spectral radius < 1 iff b < 1/2 for b >= 0.
import sympy as S
b,l=S.symbols('b l'); A=S.Matrix([[S.Rational(1,2),b,0],[S.Rational(1,4),S.Rational(1,2),b],[0,S.Rational(1,4),S.Rational(1,2)]])
assert S.expand(A.charpoly(l).as_expr()-(l-S.Rational(1,2))*((l-S.Rational(1,2))**2-b/2))==0
out=dict(scales=73,feedback_values=4,corner_checks=checks,above_boundary_rejected=failures,violations=0,sharp_budget='1/4-b',spectral_threshold='b<1/2')
Path(__file__).with_name('results.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out,indent=2))
