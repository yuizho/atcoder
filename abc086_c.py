N = int(input())


def canReach(start, T, x, y):
    target = (x, y)
    tmp = start
    for t in range(T):
        x_diff = target[0] - tmp[0]
        y_diff = target[1] - tmp[1]
        if abs(x_diff) >= abs(y_diff):
            tmp = (tmp[0]-1, tmp[1]) if x_diff < 0 else (tmp[0]+1, tmp[1])
        else:
            tmp = (tmp[0], tmp[1]-1) if y_diff < 0 else (tmp[0], tmp[1]+1)
    return tmp == target


start = (0, 0)
isReachAble = False
T = 0
for n in range(N):
    t, x, y = map(int, input().split())
    isReachAble = canReach(start, t-T, x, y)
    if (not isReachAble):
        print('No')
        exit()
    start = (x, y)
    T += t

print('Yes' if isReachAble else 'No')
