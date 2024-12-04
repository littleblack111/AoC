res = 0


vals = []
with open('2.txt', 'r') as file:
    for line in file:
        parts = line.strip().split(" ")
        inlist = []
        for num in parts:
            inlist.append(int(num))
        vals.append(inlist)


slists = []


for val in vals:
    print(val)  
    isafe = False  

    for i in range(len(val)): 
        row = val.copy()  
        row.pop(i)  
        print(row)  
        

        increasing = False
        decreasing = False
        safe = True
        prev = -1
        
        for j in row:
            if prev == -1:
                prev = j
                continue
            if prev < j:
                increasing = True
            elif prev > j:
                decreasing = True
            if (abs(prev - j) > 3 or prev == j or 
                (j > prev and decreasing) or (j < prev and increasing) or 
                (increasing and decreasing) or (not increasing and not decreasing)):
                safe = False
                break 
            prev = j
        
        if safe:
            isafe = True  
            break  

    slists.append(isafe) 

res = sum(slists)

print(res)
