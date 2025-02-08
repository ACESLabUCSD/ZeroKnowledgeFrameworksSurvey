#/bin/bash

# max ncopies gives ~60M gates
# 256: 1
# 128: 4
# 64: 7
# 32: 10
# 16: 13

RUN="pypy -OO ../fennel/run_fennel.py -z 3 -C m191 -n 0"
mkdir -p log

# echo "16x16 matrices"
# for i in $(seq 1 15); do
#     echo "log N="${i}
#     ${RUN} -p matmult_16.pws -c $i -L log/matmult_16x16_${i}.log
# done

echo "32x32 matrices"
for i in $(seq 1 12); do
    echo "log N="${i}
    ${RUN} -p matmult_32.pws -c $i -L log/matmult_32x32_${i}.log
done

# echo "64x64 matrices"
# for i in $(seq 1 9); do
#     echo "log N="${i}
#     ${RUN} -p matmult_64.pws -c $i -L log/matmult_64x64_${i}.log
# done

# echo "128x128 matrices"
# for i in $(seq 1 6); do
#     echo "log N="${i}
#     ${RUN} -p matmult_128.pws -c $i -L log/matmult_128x128_${i}.log
# done

#echo "256x256 matrices"
#for i in $(seq 1 1); do
#    echo "log N="${i}
#    ${RUN} -p matmult_256x256.pws -c $i -L log/matmult_256x256_${i}.log
#done