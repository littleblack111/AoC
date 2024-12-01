loc1 = []
loc2 = []

with open('1A.txt', 'r') as file:
    for line in file:
        parts = line.strip().split("   ")
        if len(parts) == 2:
            val1 = int(parts[0])
            val2 = int(parts[1])

            loc1.append(val1)
            loc2.append(val2)

print(sorted(loc1))
print(sorted(loc2))
sorted_loc1 = sorted(loc1)
sorted_loc2 = sorted(loc2)

final_distance = 0

for i in range(len(loc1)):
    distance = sorted_loc2[i] - sorted_loc1[i]
    abs_distance = abs(distance)
    final_distance += abs_distance
    
print(final_distance)
