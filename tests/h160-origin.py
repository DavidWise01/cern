"""H160: signed synthetic evidence; NOT third-party source attestation."""
from itertools import combinations
from hashlib import sha256
from json import dumps
from pathlib import Path
from copy import deepcopy
from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey
from cryptography.exceptions import InvalidSignature
ROOTS={'P_M':(1,),'P_I':(2,),'P_U':(4,),'P_B':(8,),'A':(3,12),'B':(5,10),'C':(9,6)}
def enc(x):return dumps(x,sort_keys=True,separators=(',',':')).encode()
def ancestry(graph,node):
 visiting=set();seen=set();roots=set()
 def walk(k):
  if k not in graph or k in visiting:raise ValueError('bad_lineage')
  if k in seen:return
  visiting.add(k)
  if not graph[k]:roots.add(k)
  for parent in graph[k]:walk(parent)
  visiting.remove(k);seen.add(k)
 walk(node);return roots
def rank(gs):
 p={}
 for x in gs:
  while x:
   j=x.bit_length()-1
   if j in p:x^=p[j]
   else:p[j]=x;break
 return len(p)
SIGNERS={f'w{i}':Ed25519PrivateKey.from_private_bytes(sha256(f'H160-public-fixture-ONLY-{i}'.encode()).digest()) for i in range(3)}
PUB={k:v.public_key() for k,v in SIGNERS.items()}
def sign(case):
 digest=sha256(enc(case['graph'])).hexdigest()
 case['evidence']=[]
 for g,src,obs,key in case['assignments']:
  msg=dict(gate=g,source=src,observer=obs,witness=key,graph_sha256=digest,cut=case['cut'])
  case['evidence'].append(dict(claim=msg,signature=SIGNERS[key].sign(enc(msg)).hex()))
 return case
def check(case):
 try:
  graph=case['graph'];assign=case['assignments'];evidence=case['evidence']
  if len(assign)!=3 or len(evidence)!=3:return 'invalid_cardinality'
  digest=sha256(enc(graph)).hexdigest()
  source_roots=[];witness_roots=[];names=[];gates=[]
  for item,(g,src,obs,key) in zip(evidence,assign):
   expected=dict(gate=g,source=src,observer=obs,witness=key,graph_sha256=digest,cut=case['cut'])
   if item['claim']!=expected:return 'mismatched_claim'
   if key not in PUB:return 'unknown_witness'
   try:PUB[key].verify(bytes.fromhex(item['signature']),enc(expected))
   except (ValueError,InvalidSignature):return 'invalid_signature'
   source_roots.append(ancestry(graph,src));witness_roots.append(ancestry(graph,obs))
   names.append(key);gates.append(g)
  if len(set(names))<3:return 'repeated_signer'
  if any(a&b for a,b in combinations(source_roots,2)):return 'shared_source_root'
  if any(a&b for a,b in combinations(witness_roots,2)):return 'shared_witness_ancestry'
  if any((case['mask']&g).bit_count()%2!=1 for g in gates):return 'invalid_parity_gate'
  if rank(case['survivors'])!=3:return 'invalid_fracture'
  return 'accepted_simulated'
 except (KeyError,ValueError,TypeError,RecursionError):return 'malformed_ancestry'
def fixture(cut,source_shared=False,witness_shared=False):
 gs=[g for name,vs in ROOTS.items() if name not in cut for g in vs]
 if rank(gs)!=3:return None
 mask=next(m for m in range(1,16) if all((m&g).bit_count()%2==0 for g in gs))
 candidates=[g for g in range(1,16) if (mask&g).bit_count()%2][:3]
 graph={};assign=[]
 for i,g in enumerate(candidates):
  graph[f's{i}']=[f'root_{0 if source_shared and i==2 else i}']
  graph[f'root_{i}']=[]
  graph[f'o{i}']=[f'observation_{0 if witness_shared and i==2 else i}']
  graph[f'observation_{i}']=[]
  assign.append((g,f's{i}',f'o{i}',f'w{i}'))
 return sign(dict(cut=list(cut),mask=mask,survivors=gs,graph=graph,assignments=assign))
stats={};details={};count=0
for cut in combinations(ROOTS,4):
 baseline=fixture(cut)
 if baseline is None:continue
 cases={'independent':baseline,'shared_source':fixture(cut,source_shared=True),
        'shared_witness':fixture(cut,witness_shared=True)}
 forged=deepcopy(baseline);forged['evidence'][1]['signature']='00'*64;cases['forged_signature']=forged
 stale=deepcopy(baseline);stale['graph']['s2']=['root_0'];cases['stale_graph']=stale
 missing=deepcopy(baseline);missing['evidence'].pop();cases['missing_witness']=missing
 cycle=deepcopy(baseline);cycle['graph']['root_0']=['s0'];cases['cyclic_lineage']=sign(cycle)
 dup=deepcopy(baseline);a=list(dup['assignments'][2]);a[3]='w0';dup['assignments'][2]=tuple(a);cases['repeated_signer']=sign(dup)
 out={name:check(value) for name,value in cases.items()}
 assert out=={'independent':'accepted_simulated','shared_source':'shared_source_root',
 'shared_witness':'shared_witness_ancestry','forged_signature':'invalid_signature',
 'stale_graph':'mismatched_claim','missing_witness':'invalid_cardinality',
 'cyclic_lineage':'malformed_ancestry','repeated_signer':'repeated_signer'}
 for k in out.values():stats[k]=stats.get(k,0)+1
 details['+'.join(cut)]=out;count+=len(out)
assert len(details)==11 and count==88 and stats['accepted_simulated']==11
report=dict(fractures=11,cases=88,counts=stats,keys='deterministic Ed25519 test-only fixture keys',real_world_attestation=False,results=details)
Path(__file__).with_name('results.json').write_text(dumps(report,indent=2)+'\n')
print(dumps({k:report[k] for k in ('fractures','cases','counts')},indent=2))
