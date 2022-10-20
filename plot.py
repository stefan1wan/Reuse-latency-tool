#!/usr/bin/python3
import struct
from turtle import right
import matplotlib.pyplot as plt
from matplotlib.pyplot import MultipleLocator

bins_data = "bins.data"
# Read data to a dic
dic = {}
with open(bins_data, "rb") as f:
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

# change count to probiblity
total = 0
max_rl = 0
for k, count in dic.items():
    total += count
    if k > max_rl:
        max_rl = k


dic_pdf = {}
next_max_rl = 0
for k, count in dic.items():
    # print(k, count, total)
    dic_pdf[k] = count/total
    assert(dic_pdf[k]>0)
    if k < max_rl and k>next_max_rl:
        next_max_rl = k


print("max_rl:%lu, next_max_rl: %lu"%(max_rl, next_max_rl))
# print(dic_pdf)

BINS_LEN = 1000
dic_pdf_bins = {}

bins = [x//BINS_LEN for x in range(next_max_rl+1)]
bins.append(max_rl//BINS_LEN)


for K in bins:
    dic_pdf_bins[K] = 0.0

for k, p in dic_pdf.items():
    K = k//BINS_LEN
    dic_pdf_bins[K] += p

lists = sorted(dic_pdf_bins.items()) # sorted by key, return a list of tuples
x, y = zip(*lists) # unpack a list of pairs into two tuples

plt.figure(figsize=(15,8))
plt.subplot(1, 2, 1)
# plt.plot(x[:-1],  [ k*100 for k in y[:-1]])
# plt.scatter(x[:-1],  [ k*100 for k in y[:-1]], color="y", marker='x')
plt.bar(x[:-1], [k*100 for k in y[:-1]])  # 100 is for percentage
plt.xlabel("Reuse Distance(%d)"%BINS_LEN)
# ax=plt.gca()
# x_major_locator=MultipleLocator(50)
# ax.xaxis.set_major_locator(x_major_locator)
# plt.xlim(right=300)
plt.ylabel("Probability(%)")
plt.title("Probability density function(PDF)")


# ===============
### cdf 
y_ = [0] * len(x)
y_[0] = y[0]
for i in range(1, len(x)):
    y_[i] = y[i] + y_[i-1]

plt.subplot(1, 2, 2)
plt.plot(x[:-1], [ k*100 for k in y_[:-1]] ) # 100 is for percentage
# plt.scatter(x[:-1], [ k*100 for k in y_[:-1]], color="y", marker='x')
# plt.bar(x[:-1], [k*100 for k in y_[:-1]])
plt.xlabel("Reuse Distance(%d)"%BINS_LEN)
# ax=plt.gca()
# x_major_locator=MultipleLocator(50)
# ax.xaxis.set_major_locator(x_major_locator)
# plt.xlim(right=300)
# plt.ylabel("Probability of these lentency(%)")
plt.title("Cumulative distribution function(CDF)")
plt.show()

print("%lu, %lf"%(x[-1], y[-1]))

print("Instructions not reused: %f %%"%(y[-1]*100))