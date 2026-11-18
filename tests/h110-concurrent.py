import sqlite3, tempfile, math, threading
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
D=2-2*math.cos(math.pi/12)+1/576
# One fixed future interval, integer conservative accounting units.
CAP=1000000
RHO=.5; TAU=5
B=1-RHO*math.exp(-D*TAU)
requests=[('jane',600000),('toph',600000),('patricia',600000)]
assert all(q<=CAP for _,q in requests) and sum(q for _,q in requests)>CAP
with tempfile.TemporaryDirectory() as tmp:
 path=Path(tmp)/'reserve.db'
 with sqlite3.connect(path) as c:
  c.executescript('CREATE TABLE ledger(id TEXT PRIMARY KEY, units INTEGER NOT NULL, decision TEXT NOT NULL);')
 def reserve(key,units,barrier=None):
  if type(units) is not int or not 0<units<=CAP: return 'INVALID'
  if barrier: barrier.wait()
  with sqlite3.connect(path,timeout=10,isolation_level=None) as c:
   c.execute('BEGIN IMMEDIATE')
   old=c.execute('SELECT units,decision FROM ledger WHERE id=?',(key,)).fetchone()
   if old: result=old[1] if old[0]==units else 'CONFLICT'
   else:
    used=c.execute("SELECT COALESCE(SUM(units),0) FROM ledger WHERE decision='ADMIT'").fetchone()[0]
    result='ADMIT' if used+units<=CAP else 'REJECT'
    c.execute('INSERT INTO ledger VALUES (?,?,?)',(key,units,result))
   c.execute('COMMIT'); return result
 for round in range(64):
  with sqlite3.connect(path) as c: c.execute('DELETE FROM ledger') # isolated test fixture reset only
  barrier=threading.Barrier(3)
  with ThreadPoolExecutor(3) as pool:
   out=list(pool.map(lambda item:reserve(*item,barrier),requests))
  assert sorted(out)==['ADMIT','REJECT','REJECT']
  winner=requests[out.index('ADMIT')][0]
  assert reserve(winner,600000)=='ADMIT'
  assert reserve(winner,500000)=='CONFLICT'
  assert reserve('extra',400000)=='ADMIT'
  assert reserve('overflow',1)=='REJECT'
  with sqlite3.connect(path) as c:
   used=c.execute("SELECT SUM(units) FROM ledger WHERE decision='ADMIT'").fetchone()[0]
   assert used==CAP
  assert RHO*math.exp(-D*TAU)+B*used/CAP<=1+1e-15
 print('PASS: 64 rounds, 192 concurrent requests, 64 winners, 128 initial rejections; replay, conflict, fill and overflow checked each round')
print('naive radius',RHO*math.exp(-D*TAU)+1.8*B,'atomic radius',RHO*math.exp(-D*TAU)+.6*B,'budget',B)
