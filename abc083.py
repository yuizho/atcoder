end, min_num, max_num = map(int, input().split())
result = 0
for n in range(1, end+1):
    tmp = 0
    for s in str(n):
        tmp += int(s)
    if tmp >= min_num and tmp <= max_num:
        result += n
print(result)
