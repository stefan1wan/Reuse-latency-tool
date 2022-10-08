#!/usr/bin/python3

from pwn import *

test_pcs = [0x1, 0x2, 0x3, 0x4, 0x1,0x1,0x3,0x5, 0x3]


with open("pc.data", "wb") as f:
    for x in test_pcs:
        f.write(p32(x))
    f.close()