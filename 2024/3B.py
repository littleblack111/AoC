import re

with open('3.txt', 'r') as f:
    data = f.read()
# Regular expressions for matching patterns
mul_pattern = r'mul\(\d+,\d+\)'
do_pattern = r'do\(\)'
dont_pattern = r"don't\(\)"

multiply = True
muls = 0

en_muls = []

# Find all mul commands and do/don't instructions
matches = re.finditer(mul_pattern + '|' + do_pattern + '|' + dont_pattern, data)

for match in matches:
    pattern = match.group()
    print(pattern)

    #identify "do()" and "dont()"

    if pattern == "do()":
        multiply = True
    elif pattern == "don't()":
        multiply = False
    elif pattern.startswith('mul'):
        if multiply:
            en_muls.append(pattern)

print(en_muls)

ints = [l.split("mul(")[1].rstrip(')') for l in en_muls]
print(ints)

for i in ints:
    n1, n2 = map(int, i.split(','))
    print(n1, n2)
    muls += n1 * n2
print(muls)
