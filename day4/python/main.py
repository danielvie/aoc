from enum import Enum

class File(Enum):
    Test = "../data/test.txt",
    Puzzle = "../data/puzzle.txt",

with open(File.Puzzle.value[0]) as f:
    lines = f.read().strip().split("\n");

lins = len(lines)
cols = len(lines[0])

print('lins,cols:', lins, cols)

# generate directions
dd = []
for dx in range(-1, 2):
    for dy in range(-1, 2):
        if dx != 0 or dy != 0:
            dd.append((dx, dy))

def has_xmas(i, j, d):
    dx, dy = d
    for k, x in enumerate("XMAS"):
        ii = i + k*dx
        jj = j + k*dy
        if not (0 <= ii < lins and 0 <= jj < cols):
            return False
        if lines[ii][jj] != x:
            return False
    return True

if __name__ == "__main__":
    # count every cell in every direction
    total = 0
    for i in range(lins):
        for j in range(cols):
            for d in dd:
                total += has_xmas(i, j, d)
                
    print(f'total: {total}')
        