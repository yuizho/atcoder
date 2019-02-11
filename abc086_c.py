N = int(input())

start_x, start_y = (0, 0)
isReachAble = False
for n in range(N):
    t, x, y = map(int, input().split())
    dist = abs(x-start_x) + abs(y-start_y)
    if (dist > t):
        print('No')
        exit()
    isTimeEven = t % 2 == 0
    isDestEven = (x + y) % 2 == 0
    if (isTimeEven and isDestEven) or (not isTimeEven and not isDestEven):
        isReachAble = True
    else:
        print('No')
        exit()

print('Yes' if isReachAble else 'No')
