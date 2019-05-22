N, Y = map(int, input().split())

if N*10000 == Y:
    print(' '.join(map(str, (N, 0, 0))))
    exit()

for a in range(N):
    for b in range(N+1-a):
        c = N-a-b
        if a*10000 + b * 5000 + c * 1000 == Y:
            print(' '.join(map(str, (a, b, c))))
            exit()
print('-1 -1 -1')
