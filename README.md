# Intro
This is a tool in rust to analyze the reuse latency of a set of PCs.
Let's assume that sequence is a,b,c,d,a,a,b,c,e.......
For the first a, there is b,c,d between the next appearence. so the reuse latency is unique([b,c,d]) = 3. We didn't count the repetitive PCs. We denote it as RL[0] = (a, 3).
Similairy,  RL[1]=(b, 3), RL[2]=(c, 3), RL[3] = (d, ∞), RL[4] = (a, 0) and so on. For `d`, there is no resusage, so we use ∞ to represent it.

## Algorithm

### Naive
We use two `for` loops to finish this work:
```python
S = [1,2,3,4,1,1,2,3,5]
RL = []
L = len(PC)
def RL_uniq(i, j):
    printf("TODO: implement")
    return 1

for i in range(L):
    for j in range(i+1, L):
        if S[i]!=S[j]:
            continue
        RL.append(RL_uniq(i, j));

print(RL)
# use RL to create pdf and cdf
# Use rust to implement the algorithm; left the rest to python;
```

### Optimizes
(1) [Todo]Multi-thread in the pre-parse phase
(2) [Done]Set search limits, the instrustion working set is (), lets assume 100K instructions at max for one core. 100KB = 100*1024/3.5 = 29959314 ≈ 30M instructions;
(3)

## Input and Output

input format
```
83910 ffffffffa4e96106  2  66 90
83910 ffffffffa4e96108  1  c3
83910 ffffffffa4e1b854  2  66 90
83910 ffffffffa4e1b856  7  4c 89 a3 60 01 00 00
83910 ffffffffa4e1b85d  1  5b
83910 ffffffffa4e1b85e  2  41 5c
83910 ffffffffa4e1b860  1  5d
83910 ffffffffa4e1b861  1  c3
83910 ffffffffa4e1c68d  4  48 83 c4 20
```
## Usage
```
./rl_tool pc.data
./rl_tool pc.txt
```
