cnt500 = int(input())
cnt100 = int(input())
cnt50 = int(input())
X = int(input())

result = 0
for a in range(cnt500+1):
    for b in range(cnt100+1):
        for c in range(cnt50+1):
           if (a*500 + b*100 + c*50) == X:
               result += 1
print(result)
