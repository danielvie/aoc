from enum import Enum

class File(Enum):
    Test = "../data/test.txt",
    Puzzle = "../data/puzzle.txt",

with open(File.Test.value[0]) as f:
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

n = lins
m = cols
def has_x_mas(i, j):
    if not (1 <= i < n - 1 and 1 <= j < m -1):
        return False
    if lines[i][j] != "A":
        return False
        
    # check diagonals
    diag1 = f"{lines[i-1][j-1]}{lines[i+1][j+1]}"
    diag2 = f"{lines[i-1][j+1]}{lines[i+1][j-1]}"
    
    res = diag1 in ["MS", "SM"] and diag2 in ["MS", "SM"]
    if res:
        print(i, j,'->', diag1, diag2)
    
    return res


if __name__ == "__main__":

    # PART 1
    total = 0
    for i in range(lins):
        for j in range(cols):
            for d in dd:
                total += has_xmas(i, j, d)
                
    print(f'part1: {total}')
        

    # PART 2
    total = 0
    for i in range(lins):
        for j in range(cols):
            total += has_x_mas(i, j)
                
    print(f'part2: {total}')
        