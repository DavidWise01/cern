import json
from pathlib import Path
known=[2,10,18,36,54,86,118]
z=0
for p,k in enumerate(known,1):
 z+=2*((p+2)//2)**2
 assert z==k
blocks=[('8s',2),('5g',18),('8p1/2',2),('6f',14),('7d',10),('9s',2),('9p1/2',2),('8p3/2',4)]
assert sum(v for _,v in blocks)==54
assert 118+54==172
assert 118+2*25==168
rows=[dict(Z=z,pairs=z//2,added_pairs=(z-118)//2,extra_vs_168=z-168) for z in [168,172,184]]
assert rows[-1]['pairs']==92 and rows[-1]['added_pairs']==33
out=dict(known_checks=7,blocks=blocks,candidates=rows,identity='184 = 118 + 50 + 16 = 172 + 12')
Path(__file__).with_name('results.json').write_text(json.dumps(out,indent=2));print(json.dumps(out,indent=2))
