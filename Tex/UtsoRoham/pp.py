import operator as op
from functools import reduce

def ncr(n, r):
    r = min(r, n-r)
    numer = reduce(op.mul, range(n, n-r, -1), 1)
    denom = reduce(op.mul, range(1, r+1), 1)
    return numer // denom  # or / in Python 2

x = 0.40
n = 5

meg = 0
for k in range(0,3):
    megd = x**k * (1-x)**(n-k) *ncr(n,k)
    print("$$k=0 \\rightarrow "+str(megd)+"$$")
    meg += megd

print("$$\\sum k \\rightarrow "+str(meg)+"$$")
