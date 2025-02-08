from picozk import *
import numpy as np

n = 8

p = 2**61-1

def int_matrix(m):
    intv = np.vectorize(int)
    return np.array(intv(m).tolist(), dtype=object)

a = int_matrix(np.random.randint(0, p, (n, n)))
b = int_matrix(np.random.randint(0, p, (n, n)))
c = a @ b

zk_matrix = np.vectorize(SecretInt, otypes=[Wire])

with PicoZKCompiler('picozk_test'):
    zk_a = zk_matrix(a)
    zk_b = zk_matrix(b)

    zk_c = zk_a @ zk_b
    
    reveal_matrix = np.vectorize(reveal)
    reveal_matrix(zk_c)