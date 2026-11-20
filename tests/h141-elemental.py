import json
from pathlib import Path
rows=[('He',2,[2]),('Ne',10,[2,8]),('Ar',18,[2,8,8]),('Kr',36,[2,8,18,8]),('Xe',54,[2,8,18,18,8]),('Rn',86,[2,8,18,32,18,8]),('Og',118,[2,8,18,32,32,18,8])]
prev=0; out=[]
for symbol,Z,shells in rows:
 assert sum(shells)==Z and Z%2==0
 assert all(k<=2*n*n for n,k in enumerate(shells,1))
 out.append(dict(symbol=symbol,Z=Z,shells=shells,pairs=Z//2,added_pairs=(Z-prev)//2));prev=Z
Path(__file__).with_name('results.json').write_text(json.dumps(out,indent=2))
print(json.dumps(out))
