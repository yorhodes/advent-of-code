with open('input.txt') as f:
    depths = [int(s) for s in f.read().splitlines()]
    prev = float('inf')
    count = 0
    for depth in depths:
        if depth > prev:
            count += 1
        prev = depth
    print(count)