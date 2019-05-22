N, W = map(int, input().split())
items = []
for n in range(N):
    items.append(tuple(map(int, input().split())))

dp = [[0] * (W+1) for x in range(N+1)]

for i, item in enumerate(items):
    wait, value = item
    for w in range(W+1):
        # choiced case
        if w - wait >= 0:
            dp[i+1][w] = max(dp[i+1][w], dp[i][w - wait] + value)
        # not choiced case
        dp[i+1][w] = max(dp[i+1][w], dp[i][w])

print(max(sum(dp, [])))
