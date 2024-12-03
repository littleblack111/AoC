res = 0

with open("2.in") as f:
    for i in f:
        increasing = False
        decreasing = False
        safe = True
        n = i.split()
        prev = -1
        for j in n:
            j = int(j)
            if prev == -1:
                prev = j
                continue
            if prev < j:
                increasing = True
            elif prev > j:
                decreasing = True
            if (abs(prev - j) > 3 or prev == j or (j > prev and decreasing or j < prev and increasing or increasing and decreasing or not increasing and not decreasing)):
                safe = False
                # break
            prev = j
        if safe:
            res += 1

print(res)
