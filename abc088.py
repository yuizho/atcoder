N = int(input())
cards = list(map(int, input().split()))
alice = []
bob = []
for i in range(N):
    max_card = cards.pop(cards.index(max(cards)))
    if i % 2 == 0:
        alice.append(max_card)
    else:
        bob.append(max_card)
print(sum(alice) - sum(bob))
