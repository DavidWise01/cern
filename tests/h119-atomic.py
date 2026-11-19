import json, sqlite3, tempfile, hashlib, multiprocessing as mp, os
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey, Ed25519PublicKey
from cryptography.hazmat.primitives.serialization import Encoding, PublicFormat
from cryptography.exceptions import InvalidSignature
ROOT='228dba7ae919a855718e3bb6048b0877522ddad0eba2a83430866dd4ebae3d3b'
SCOPE=dict(domain='cern/H119/continue/v1',checkpoint=ROOT,epoch=1,successor='H120',agent='patricia')
def enc(x): return json.dumps(x,sort_keys=True,separators=(',',':'),allow_nan=False).encode()
def init(db):
    with sqlite3.connect(db) as c:
        c.execute('PRAGMA journal_mode=WAL')
        c.execute('CREATE TABLE IF NOT EXISTS commits(nonce TEXT PRIMARY KEY, token TEXT NOT NULL, receipt TEXT NOT NULL)')
def admit(db,pub,token,crash=None):
    body=token['body']
    try: Ed25519PublicKey.from_public_bytes(pub).verify(bytes.fromhex(token['sig']),enc(body))
    except (InvalidSignature,ValueError): return 'INVALID',None
    if set(body)!=set(SCOPE)|{'nonce'} or any(body[k]!=v for k,v in SCOPE.items()) or not isinstance(body['nonce'],str) or not body['nonce']:return 'SCOPE',None
    digest=hashlib.sha256(enc(token)).hexdigest()
    c=sqlite3.connect(db,timeout=15,isolation_level=None)
    try:
        c.execute('PRAGMA synchronous=FULL');c.execute('BEGIN IMMEDIATE')
        old=c.execute('SELECT token,receipt FROM commits WHERE nonce=?',(body['nonce'],)).fetchone()
        if old:
            c.execute('COMMIT');return ('REPLAY',old[1]) if old[0]==digest else ('CONFLICT',None)
        receipt=hashlib.sha256(enc(['H119/receipt',digest])).hexdigest()
        c.execute('INSERT INTO commits VALUES(?,?,?)',(body['nonce'],digest,receipt))
        if crash=='before':os._exit(71)
        c.execute('COMMIT')
        if crash=='after':os._exit(72)
        return 'COMMIT',receipt
    finally:c.close()
def count(db):
    with sqlite3.connect(db) as c:return c.execute('SELECT COUNT(*) FROM commits').fetchone()[0]
def main():
    key=Ed25519PrivateKey.generate();pub=key.public_key().public_bytes(Encoding.Raw,PublicFormat.Raw)
    def sign(n,**kw):
        body=dict(SCOPE,nonce=n);body.update(kw);return {'body':body,'sig':key.sign(enc(body)).hex()}
    with tempfile.TemporaryDirectory() as tmp:
        db=str(Path(tmp)/'ledger.db');init(db)
        for i in range(32):
            token=sign(str(i))
            with ThreadPoolExecutor(8) as pool: out=list(pool.map(lambda _:admit(db,pub,token),range(8)))
            assert sum(s=='COMMIT' for s,r in out)==1 and sum(s=='REPLAY' for s,r in out)==7
            assert len({r for s,r in out})==1
        assert count(db)==32
        for phase,code in [('before',71),('after',72)]:
            token=sign(phase);p=mp.Process(target=admit,args=(db,pub,token,phase));p.start();p.join(15);assert p.exitcode==code
            assert count(db)==(32 if phase=='before' else 34)
            status,receipt=admit(db,pub,token);assert status==('COMMIT' if phase=='before' else 'REPLAY')
            assert admit(db,pub,token)==('REPLAY',receipt)
        assert count(db)==34
        t=sign('bad',epoch=2);assert admit(db,pub,t)[0]=='SCOPE'
        t=sign('bad');t['body']['agent']='jane';assert admit(db,pub,t)[0]=='INVALID'
        assert count(db)==34
    result=dict(rounds=32,concurrent_requests=256,concurrent_commits=32,concurrent_replays=224,crash_before='rollback then commit',crash_after='durable receipt replay',final_commits=34,invalid_rejected=2)
    print(json.dumps(result,indent=2));Path(__file__).with_name('results.json').write_text(json.dumps(result,indent=2)+'\n')
if __name__=='__main__':main()
