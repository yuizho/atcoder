N = int(input())
cards = list(map(int, input().split()))
cards.sort()
alice = []
bob = []
for i in range(N):
    if i % 2 == 0:
        alice.append(cards.pop())
    else:
        bob.append(cards.pop())
print(sum(alice) - sum(bob))
