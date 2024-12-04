import re

with open('3.txt', 'r') as f:
    data = f.read()

mul = r'mul\(\d+,\d+\)'
matches = re.findall(mul, data)  

lmuls = []
lmuls.extend(matches)

ints = [l.split("mul(")[1].rstrip(')') for l in lmuls]
print(ints)

muls = 0

for i in ints:
    n1, n2 = map(int, i.split(','))
    print(n1, n2)
    muls += n1 * n2
print(muls)
