import json, hashlib, copy, importlib.util, sys
from pathlib import Path
from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey
from cryptography.exceptions import InvalidSignature

def enc(x): return json.dumps(x,sort_keys=True,separators=(',',':'),allow_nan=False).encode('ascii')
def load(path):
 s=importlib.util.spec_from_file_location(path.stem,path); m=importlib.util.module_from_spec(s);s.loader.exec_module(m);return m

def main(base):
 h=load(base/'h117/h117.py'); p=load(base/'h116/h116.py')
 ledger=json.loads((base/'h117/ledger.json').read_text()); cp=[143,'228dba7ae919a855718e3bb6048b0877522ddad0eba2a83430866dd4ebae3d3b']
 assert h.verify(ledger,cp,p.verify)
 key=Ed25519PrivateKey.generate(); rogue=Ed25519PrivateKey.generate(); pub=key.public_key()
 state={'epoch':1,'used':set()}; context={'domain':'cern/H118/continue/v1','checkpoint':cp,'epoch':1,'successor':'H119','agent':'patricia','nonce':'001'}
 def sign(body,k=key):return {'body':copy.deepcopy(body),'signature':k.sign(enc(body)).hex()}
 def admit(token):
  body=token['body'];pub.verify(bytes.fromhex(token['signature']),enc(body))
  expected=dict(context,epoch=state['epoch'])
  if set(body)!=set(expected) or any(body[k]!=v for k,v in expected.items() if k!='nonce'):raise ValueError('scope')
  if not isinstance(body['nonce'],str) or not body['nonce'] or body['nonce'] in state['used']:raise ValueError('nonce')
  state['used'].add(body['nonce']);return True
 token=sign(context);results={}
 def reject(name,t):
  before=state['used'].copy()
  try:admit(t)
  except (ValueError,InvalidSignature):results[name]='rejected'
  else:raise AssertionError(name)
  assert before==state['used']
 for field,value in [('checkpoint',[143,'0'*64]),('epoch',2),('successor','H120'),('agent','jane'),('domain','other'),('nonce','002')]:
  t=copy.deepcopy(token);t['body'][field]=value;reject('tamper_'+field,t)
 for field,value in [('checkpoint',[142,cp[1]]),('agent','jane'),('successor','H120'),('domain','other'),('nonce','')]:reject('signed_wrong_'+field,sign(dict(context,**{field:value})))
 reject('rogue_key',sign(context,rogue));assert admit(token);reject('replay',token)
 fresh_old=sign(dict(context,nonce='003'));state['epoch']=2;reject('stale_unused',fresh_old)
 assert admit(sign(dict(context,epoch=2,nonce='004')))
 out={'checkpoint':cp,'mathematical_verification':True,'accepted':2,'rejected':len(results),'attacks':results,'final_epoch':2}
 (Path(__file__).parent/'results.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out,indent=2))
if __name__=='__main__':main(Path(sys.argv[1]) if len(sys.argv)>1 else Path('/mnt/data'))
