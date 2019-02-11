S = input()
patterns = ['dreamer', 'dream', 'eraser', 'erase']


while len(S) != 0:
    isShaved = False
    for p in patterns:
        if S.endswith(p):
            S = S[:len(S)-len(p)]
            isShaved = True
            break
    if not isShaved:
        break

print('YES' if len(S) == 0 else 'NO')
