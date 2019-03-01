import sys

# TODO: this code exceeds time limit in some case...
N, K = list(map(int, input().split()))
steps = list(map(int, input().split()))
dp = [sys.maxsize] * N

dp[0] = 0


def min_step(current_index):
    costs = []
    for step in range(1, K+1):
        if current_index >= step:
            current_step_cost = dp[current_index - step]
            this_step_cost = abs(steps[current_index] - steps[current_index - step])
            costs.append(this_step_cost + current_step_cost)
        else:
            return min(costs)
    return min(costs)


for n in range(1, N):
    dp[n] = min_step(n)

print(dp[-1])
