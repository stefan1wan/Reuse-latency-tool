# Intro
This is a tool in rust to analyze the reuse latency(RL) of a set of PCs.
Let's assume that sequence is `a,b,c,d,a,a,b,c,e.......`
For the first a, there is b,c,d between the next appearence. so the reuse latency is `unique([b,c,d]) = 3`. We don't count the repetitive PCs. We denote it as `RL[0] = (a, 3)`.
Similairy,  `RL[1]=(b, 3), RL[2]=(c, 3), RL[3] = (d, ∞), RL[4] = (a, 0)` and so on. For `d`, there is no resusage, so we use ∞ to represent it.
The resue latency is useful to analyze instruction working sets later.

## Algorithm

### Naive
We use two `for` loops to do this work:
```python
PC = [1,2,3,4,1,1,2,3,5]
RL = []
L = len(PC)

for i in range(L):
    for j in range(i+1, L):
        if PC[i]!=PC[j]:
            continue
        RL.append(RL_uniqe(i, j))

print(RL)
```

### Optimization
Use a hashmap to record all the PCs and its indexes. Then we just need to traverse the log once. We will also maintain some states to speedup RL calculation for basic bolcks. For details please see the code. 
```python
PC = [1,2,3,4,1,1,2,3,5]
RL = [0xffff]*len(PC)
L = len(PC)
hashmap = {}

for i in range(L):
    pc = PC[i]
    if pc in hashmap.keys():
        last_i = hashmap[pc]
        RL[last_i] = RL_uniqe(last_i + 1, i)

    hashmap[PC[i]] = i
print(RL) # 0xffff in RL means that pc is not reused
```

## File format

### "pc.txt": txt file
The file pc.txt is a dummy file for tests
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
### "pc.data": binary file
Each pc is a little-Endian, 8-byte, unsigined integer. 
### "rl.data": binary file
Each rl(reuse latency) is a little-Endian, 8-byte, unsigined integer. 

### "bins.data": binary file
It keeps the counts for every rl. Each rl(reuse latency) and each count is a little-Endian, 8-byte, unsigined integer, like this  [rl][count][rl][count].......
## Usage
```
./rl_tool pc.txt
```
