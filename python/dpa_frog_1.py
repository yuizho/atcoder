import sys

N = int(input())
dp = [sys.maxsize] * N
steps = list(map(int, input().split()))

dp[0] = 0

for n in range(1, N):
    step1_diff = abs(steps[n] - steps[n-1]) + dp[n-1]
    dp[n] = step1_diff
    if n >= 2:
        step2_diff = abs(steps[n] - steps[n-2]) + dp[n-2]
        if dp[n] > step2_diff:
            dp[n] = step2_diff
print(dp[-1])