N = int(input())

happinesses_each_day = []
for n in range(N):
    happinesses_each_day.append(list(map(int, input().split())))

dp = [[0, 0, 0] for x in range(N+1)]

# this is my understanging memo
# https://photos.app.goo.gl/Mbn1e7TctyhozFEb6
for i, happinesses in enumerate(happinesses_each_day):
    for j, happiness in enumerate(happinesses):
        for k in range(3):
            
            if j == k:
                continue
            if dp[i + 1][k] < (dp[i][j] + happiness):
                dp[i + 1][k] = (dp[i][j] + happiness)

print(max(dp[N]))
