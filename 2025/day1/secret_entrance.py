def puzzle_1(rotations: list[str]) -> int:
    SIZE = 100
    curr = 50
    count = 0
    for rotation in rotations:
        dir, num = rotation[0], int(rotation[1:])
        if dir == 'R':
            curr = (curr + num) % SIZE
        else:
            curr = (curr - num) % SIZE
        
        if curr == 0:
            count += 1
    return count

def puzzle_2(rotations: list[str]) -> int:
    SIZE = 100
    curr = 50
    count = 0
    for rotation in rotations:
        dir, num = rotation[0], int(rotation[1:])
        if dir == 'R':
            count += (curr + num) // SIZE
        else:
            first = curr if curr != 0 else SIZE
            if num >= first:
                count += 1 + (num - first) // SIZE
        curr = (curr + num if dir == 'R' else curr - num) % SIZE
        
    return count
    
if __name__ == '__main__':
    with open('input.txt') as f:
        rotations = f.read().split()
    print(puzzle_1(rotations), puzzle_2(rotations))