#remove all lines containing a single letter
with open('words.txt', 'r') as f:
    lines = f.readlines()
    lines = [line for line in lines if len(line) > 2]

#remove all lines containing a single letter
with open('words2.txt', 'w') as f:
    f.writelines(lines)