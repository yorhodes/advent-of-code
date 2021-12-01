with open('input.txt') as f:
    depths = [int(s) for s in f.read().splitlines()]
    
    ## part 1
    prev = float('inf')
    count = 0
    for depth in depths:
        if depth > prev:
            count += 1
        prev = depth
    print("part1", count)

    ## part 2
    windowsize = 3
    count = 0
    prev = float('inf')
    for i in range(len(depths)+1-windowsize):
        window = depths[i:i+windowsize]
        curr = sum(window)
        if curr > prev:
            count += 1
        prev = curr
    print("part2", count)
