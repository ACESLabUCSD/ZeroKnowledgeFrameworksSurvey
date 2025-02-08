with open("sha256.txt", "r") as in_file:
    lines = in_file.readlines()

d = 0

for line in lines:
    if "XOR" in line:
        d += 1

print(d)