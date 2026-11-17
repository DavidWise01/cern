from itertools import product, combinations
bands=(-1,0,1)
axes='xyzw'
planes=list(combinations(axes,2))
states=list(product(bands,(-1,1),(-1,1),(-1,1)))
flip=lambda s:(s[0], *(-v for v in s[1:]))
assert len(states)==len(set(states))==24
assert len(axes)+len(planes)==10
assert all(flip(flip(s))==s and flip(s)!=s for s in states)
assert len({frozenset((s,flip(s))) for s in states})==12
print('PASS: 3 bands, 10 geometric labels, 24 states, 12 complementary pairs')
