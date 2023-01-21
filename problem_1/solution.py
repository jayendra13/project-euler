N = 1000
k1, k2 = 3, 5

total = sum(filter(lambda x: (x%k1==0) or (x%k2==0), range(N)))
print(total)