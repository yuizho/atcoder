V = int(input())



def has_odd(L):
    for l in L:
        if l % 2 != 0:
            return True
    return False

def dev2(l):
    return l // 2

nums = list(map(int, input().split()))
times = 0
while not has_odd(nums):
    nums = list(map(dev2, nums))
    times += 1
print(times)
