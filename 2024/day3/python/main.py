import re

def run1(filename: str) -> int:
    # reading file
    with open(filename, 'r') as f:
        txt = f.read()
        
    # finding patterns
    m = re.findall(r'mul\((\d+),(\d+)\)', txt)

    # multiplying elements
    mul = sum(int(a)*int(b) for a,b in m)
    return mul

def run2(filename: str) -> int:
    # reading file
    with open(filename, 'r') as f:
        txt = f.read()
    
    # removing `don't`
    txt = ''.join(txt.split('\n'))
    m = re.sub(r"don't\(\).*?do\(\)", "", txt)

    # finding patterns
    m = re.findall(r'mul\((\d+),(\d+)\)', m)

    # multiplying elements
    mul = sum(int(a)*int(b) for a,b in m)
    return mul

def part1():
    print('\npart1:')
    res_test = run1("../data/test.txt")
    print(f'res test: {res_test}')

    res_run = run1("../data/puzzle.txt")
    print(f'res run.: {res_run}')

def part2():
    print('\npart2:')
    res_test = run2("../data/test2.txt")
    print(f'res test: {res_test}')

    res_run = run2("../data/puzzle.txt")
    print(f'res run.: {res_run}')

if __name__ == "__main__":
    part1()
    part2()