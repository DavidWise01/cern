import math, random, sqlite3, tempfile
from concurrent.futures import ThreadPoolExecutor
D=2-2*math.cos(math.pi/12)+1/576
R=1.; X=.5; T=30

def envelope(pulses):
    times=sorted({0,T,*[t for a,b,f in pulses for t in (a,b)]})
    x=X; out=[(0,x)]
    for a,b in zip(times,times[1:]):
        f=sum(f for s,e,f in pulses if s<=a<e)
        x=x+(f-D*x)*(-math.expm1(-D*(b-a)))/D
        out.append((b,x))
    return out

def safe(p): return max(x for _,x in envelope(p))<=R-1e-12

bad=[(0,10,2*D)]
assert envelope(bad)[-1][1]<1 and not safe(bad)
rng=random.Random(111); accepted=rejected=0
for trial in range(128):
    p=[]
    for j in range(12):
        a=rng.randrange(0,25); b=rng.randrange(a+1,31); f=rng.randrange(1,9)*D/4
        candidate=p+[(a,b,f)]
        if safe(candidate): p=candidate; accepted+=1
        else: rejected+=1
        assert max(x for _,x in envelope(p))<=1
print('random proposals',accepted,rejected)
with tempfile.TemporaryDirectory() as tmp:
    path=tmp+'/ledger.db'
    with sqlite3.connect(path) as c:c.execute('CREATE TABLE ledger(id TEXT PRIMARY KEY,a INT,b INT,f REAL,decision INT)')
    def admit(row):
        key,a,b,f=row
        with sqlite3.connect(path,timeout=10,isolation_level=None) as c:
            c.execute('BEGIN IMMEDIATE')
            old=c.execute('SELECT a,b,f,decision FROM ledger WHERE id=?',(key,)).fetchone()
            if old: result=old[3] if old[:3]==(a,b,f) else -1
            else:
                p=c.execute('SELECT a,b,f FROM ledger WHERE decision=1').fetchall()
                result=int(safe(p+[(a,b,f)]))
                c.execute('INSERT INTO ledger VALUES(?,?,?,?,?)',(key,a,b,f,result))
            c.execute('COMMIT');return result
    rows=[('jane',0,8,1.5*D),('toph',2,10,1.5*D),('patricia',4,12,1.5*D)]
    for round in range(32):
        with sqlite3.connect(path) as c:c.execute('DELETE FROM ledger')
        with ThreadPoolExecutor(3) as pool: results=list(pool.map(admit,rows))
        assert sum(results)==1
        assert list(map(admit,rows))==results
        assert admit(('jane',0,8,D))==-1
        with sqlite3.connect(path) as c:p=c.execute('SELECT a,b,f FROM ledger WHERE decision=1').fetchall()
        assert safe(p)
print('PASS: 32 concurrent rounds, 96 requests, 96 replays, 32 conflicts')
print('counterexample',envelope(bad))
