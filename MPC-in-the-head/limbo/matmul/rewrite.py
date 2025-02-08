import sys

mat = int(sys.argv[1])
data = []
with open(f'matmul{str(mat)}.txt', 'r', encoding='utf-8') as file: 
    data = file.readlines()
  
# print(data)
data[1] = f"2 {mat*mat} {int(mat*mat*mat/2)}\n"
data[2] = f"1 {mat*mat*2}\n\n"

with open(f'matmul{sys.argv[1]}.txt', 'w', encoding='utf-8') as file:  
    file.writelines(data) 