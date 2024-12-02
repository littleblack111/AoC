loc1 = []
loc2 = []

with open('1.txt', 'r') as file:
    for line in file:
        parts = line.strip().split("   ")
        if len(parts) == 2:
            val1 = int(parts[0])
            val2 = int(parts[1])

            loc1.append(val1)
            loc2.append(val2)
res = 0
dic = {}

for i in loc1:
    dic[i] = 0
    for j in loc2:
        dic[j] = 0

for i in loc1:
    dic[i] += 1

for j in loc2:
    res += j * dic[j]

print(res)
