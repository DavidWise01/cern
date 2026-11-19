import hashlib,json,copy,sys
from pathlib import Path
from fractions import Fraction as F
import importlib.util

def canonical(x): return json.dumps(x,sort_keys=True,separators=(',',':'),ensure_ascii=True,allow_nan=False).encode('ascii')
def digest(x): return hashlib.sha256(canonical(x)).hexdigest()
MODEL={'schema':'h117/v1','a':'9/10','x':'1/4','horizon':12,'boundary':'1','objective':'integer weighted value','verifier':'h116/v1'}
def append(chain,kind,payload):
 body={'seq':len(chain),'prev':chain[-1]['hash'] if chain else '0'*64,'kind':kind,'payload':copy.deepcopy(payload)}
 chain.append(dict(body,hash=digest(body)))
def build(proof):
 c=[];append(c,'model',MODEL);append(c,'input',proof['items']);append(c,'winner',{'best':proof['best'],'winner':proof['winner']})
 for cert in proof['certificate']:append(c,'certificate',cert)
 append(c,'seal',{'count':len(proof['certificate']),'input':c[1]['hash'],'winner':c[2]['hash']})
 return c
def verify(c,checkpoint,proof_verifier):
 if len(c)<4 or (len(c),c[-1]['hash'])!=tuple(checkpoint):raise ValueError('checkpoint')
 prev='0'*64
 for i,r in enumerate(c):
  body={k:v for k,v in r.items() if k!='hash'}
  if r['seq']!=i or r['prev']!=prev or digest(body)!=r['hash']:raise ValueError('chain')
  prev=r['hash']
 if [r['kind'] for r in c[:3]]!=['model','input','winner'] or c[0]['payload']!=MODEL or c[-1]['kind']!='seal' or any(r['kind']!='certificate' for r in c[3:-1]):raise ValueError('schema')
 if c[-1]['payload']!={'count':len(c)-4,'input':c[1]['hash'],'winner':c[2]['hash']}:raise ValueError('seal')
 items=[(s,l,F(f),w) for s,l,f,w in c[1]['payload']];w=c[2]['payload']
 proof_verifier(items,w['best'],w['winner'],[r['payload'] for r in c[3:-1]])
 return True
def main():
 if not __debug__:raise RuntimeError('assertions required by H116 verifier')
 src=Path(sys.argv[1]);spec=importlib.util.spec_from_file_location('h116',src);mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
 proof=json.loads(Path(sys.argv[2]).read_text());c=build(proof);cp=[len(c),c[-1]['hash']];assert verify(c,cp,mod.verify)
 attacks={}
 def reject(name,chain,anchor=cp):
  try:verify(chain,anchor,mod.verify)
  except (ValueError,AssertionError,KeyError,TypeError,IndexError):attacks[name]='rejected'
  else:raise AssertionError(name)
 for name,i,key,value in [('model',0,'a','1'),('input',1,None,[]),('winner',2,'best',25),('certificate',3,None,[]),('seal',len(c)-1,'count',0)]:
  q=copy.deepcopy(c)
  if key is None:q[i]['payload']=value
  else:q[i]['payload'][key]=value
  reject(name,q)
 reject('truncation',c[:-1]);reject('replay',c[:4]+[c[3]]+c[4:]);reject('reorder',c[:3]+[c[4],c[3]]+c[5:]);reject('deletion',c[:3]+c[4:])
 p=copy.deepcopy(proof);p['best']=25;q=build(p);reject('rehash_forgery',q);reject('false_math_new_anchor',q,[len(q),q[-1]['hash']])
 p=copy.deepcopy(proof);p['certificate'].pop();q=build(p);reject('missing_proof_new_anchor',q,[len(q),q[-1]['hash']])
 # A freshly rehashed valid but different history is internally valid; the original anchor rejects it.
 p=copy.deepcopy(proof);p['certificate'].reverse();q=build(p);assert verify(q,[len(q),q[-1]['hash']],mod.verify);reject('alternate_valid_history',q)
 out={'records':len(c),'certificates':len(proof['certificate']),'optimum':proof['best'],'checkpoint':cp,'attacks':attacks,'covered':sum(2**(len(proof['winner'])-len(z)) for z,_,_ in proof['certificate'])}
 root=Path(__file__).parent
 for name,obj in [('ledger.json',c),('results.json',out)]: (root/name).write_text(json.dumps(obj,indent=2)+'\n')
 print(json.dumps(out,indent=2))
if __name__=='__main__':main()
