with open("matmul128.txt", "r") as in_file:
    lines = in_file.readlines()

x = 0
a = 0

for line in lines:
    if "XOR" in line:
        x += 1
    if "AND" in line:
        a += 1

print("Number of mult gates", a)
print("Number of add gates", x)