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

### 

## Input and Output

## Usage
```
./rl_tool pc.data
./rl_tool pc.txt
```
