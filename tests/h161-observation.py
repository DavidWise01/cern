from itertools import combinations,product
from hashlib import sha256
from pathlib import Path
from copy import deepcopy
from collections import Counter
import json
from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey
from cryptography.exceptions import InvalidSignature
ROOTS={'P_M':(1,),'P_I':(2,),'P_U':(4,),'P_B':(8,),'A':(3,12),'B':(5,10),'C':(9,6)}
KEYS={f'W{i}':Ed25519PrivateKey.from_private_bytes(sha256(f'H161 DEMO {i}'.encode()).digest()) for i in range(3)}
PUB={k:v.public_key() for k,v in KEYS.items()}
def canonical(x):return json.dumps(x,sort_keys=True,separators=(',',':')).encode()
def digest(x):return sha256(canonical(x)).hexdigest()
def rank(vs):
 pivots={}
 for v in vs:
  while v:
   bit=v.bit_length()-1
   if bit in pivots:v^=pivots[bit]
   else:pivots[bit]=v;break
 return len(pivots)
def origin(graph,node):
 active=set();seen=set();roots=set()
 def walk(x):
  if x not in graph or x in active:raise ValueError('unknown or cyclic ancestor')
  if x in seen:return
  active.add(x)
  if not graph[x]:roots.add(x)
  for parent in graph[x]:walk(parent)
  active.remove(x);seen.add(x)
 walk(node);return roots
def sign(c):
 for item in c['statements']:
  item['payload']['digest']=digest(c['graph'])
  key=item['payload']['key']
  item['signature']=KEYS[key].sign(canonical(item['payload'])).hex()
 return c
def fixture(cut,pattern):
 survivors=[g for name,group in ROOTS.items() if name not in cut for g in group]
 if rank(survivors)!=3:return None
 mask=next(m for m in range(1,16) if all((m&g).bit_count()%2==0 for g in survivors))
 gs=[g for g in range(1,16) if (m&mask).bit_count()%2][:3];graph={};statements=[]
 for i,g in enumerate(gs):
  graph[f'bridge{i}']=[f'Broot{i}'];graph[f'Broot{i}']=[]
  graph[f'witness{i}']=[f'Wroot{i}'];graph[f'Wroot{i}']=[]
  graph[f'observation{i}']=[f'Oroot{pattern[i]}'];graph[f'Oroot{pattern[i]}']=[]
  statements.append({'payload':{'cut':list(cut),'mask':mask,'gate':g,'bridge':f'bridge{i}','witness':f'witness{i}','observation':f'observation{i}','key':f'W{i}'},'signature':''})
 return sign({'graph':graph,'statements':statements,'cut':list(cut),'mask':mask,'survivors':survivors})
def check(c):
 try:
  if len(c['statements'])!=3 or rank(c['survivors'])!=3:return 'bad_structure'
  mask=c['mask']
  if not mask or any((mask&g).bit_count()%2 for g in c['survivors']):return 'bad_mask'
  keys=set();source=[];witness=[];info=[]
  for item in c['statements']:
   p=item['payload'];key=p['key']
   if p['digest']!=digest(c['graph']):return 'stale_graph'
   if p['cut']!=c['cut'] or p['mask']!=mask:return 'mismatched_claim'
   if key not in PUB:return 'unknown_key'
   try:PUB[key].verify(bytes.fromhex(item['signature']),canonical(p))
   except (ValueError,InvalidSignature):return 'invalid_signature'
   if key in keys:return 'duplicate_key'
   keys.add(key)
   if not ((mask&p['gate']).bit_count()%2):return 'gate_does_not_cross'
   source.append(origin(c['graph'],p['bridge']))
   witness.append(origin(c['graph'],p['witness']))
   info.append(origin(c['graph'],p['observation']))
  for groups,label in [(source,'shared_source'),(witness,'shared_witness'),(info,'copied_observation')]:
   if any(a&b for a,b in combinations(groups,2)):return label
  return 'accepted_synthetic'
 except (ValueError,KeyError,TypeError,RecursionError):return 'malformed_ancestry'
patterns=[(0,0,0),(0,0,1),(0,1,0),(0,1,1),(0,1,2)]
tally=Counter();fractures=0;independently_signed=0
for cut in combinations(ROOTS,4):
 base=fixture(cut,patterns[-1])
 if base is None:continue
 fixtures=[fixture(cut,p) for p in patterns]
 for f in fixtures:
  for item in f['statements']:
   PUB[item['payload']['key']].verify(bytes.fromhex(item['signature']),canonical(item['payload']))
   independently_signed+=1
 forged=deepcopy(base);forged['statements'][0]['signature']='00'*64
 stale=deepcopy(base);stale['graph']['observation1']=['Oroot0']
 cyclic=deepcopy(base);cyclic['graph']['Oroot0']=['observation0'];sign(cyclic)
 missing=deepcopy(base);del missing['graph']['Oroot1']
 codes=[check(f) for f in fixtures+[forged,stale,cyclic,missing]]
 assert codes==['copied_observation']*4+['accepted_synthetic','invalid_signature','stale_graph','malformed_ancestry','stale_graph']
 tally.update(codes);fractures+=1
assert fractures==11 and sum(tally.values())==99 and independently_signed==165
assert tally['accepted_synthetic']==11 and tally['copied_observation']==44
result=dict(fractures=fractures,cases=sum(tally.values()),valid_signatures_in_lineage_patterns=independently_signed,decisions=dict(tally),real_world_independence_verified=False)
Path(__file__).with_name('results.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(result,indent=2))
