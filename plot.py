#!/usr/bin/python3
import struct
import matplotlib.pyplot as plt

# Read data to dic
dic = {}
with open("bins.data", "rb") as f:
    while True:
        rl = f.read(8)
        if not rl:
            break;
        rl_u64 = struct.unpack("L",rl)[0]
        count = f.read(8)
        count_u64 = struct.unpack("L", count)[0]
        dic[rl_u64] = count_u64
        assert(count_u64<0xffffffffffff)
    f.close()

print(dic)

# ===============
# Draw pdf
total = 0
for k, count in dic.items():
    total += count

max_key = 0
dic_pdf = {}
for k, count in dic.items():
    # print(k, count, total)
    dic_pdf[k] = count/total
    assert(dic_pdf[k]>0)
    if k > max_key:
        k = max_key

print(dic_pdf)

bins_len = 50
dic_pdf_bins = {}

for k, p in dic_pdf.items():
    K = k//bins_len
    if K not in dic_pdf_bins.keys():
        dic_pdf_bins[K] = 0.0
    dic_pdf_bins[K] += p

print(dic_pdf_bins)


lists = sorted(dic_pdf_bins.items()) # sorted by key, return a list of tuples
x, y = zip(*lists) # unpack a list of pairs into two tuples
plt.plot(x[:-1], y[:-1])
plt.scatter(x[:-1], y[:-1], color="g")
plt.xlabel("each %d instructions. unused: %lf"%(bins_len, y[-1]))
plt.show()

# ===============
### cdf 
y_ = [0] * len(x)
y_[0] = y[0]
for i in range(1, len(x)):
    y_[i] = y[i] + y_[i-1]

plt.plot(x[:-1], [ k*100 for k in y_[:-1]] )
plt.scatter(x[:-1], [ k*100 for k in y_[:-1]], color="y", marker='x')
plt.xlabel("each %d instructions. unused: %lf"%(bins_len, y[-1]))
plt.ylabel("probability(%)")
plt.title("cdf")
plt.show()
