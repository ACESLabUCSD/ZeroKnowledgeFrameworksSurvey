docker build -t circ .
id=$(docker create circ)
# docker cp $id:/emp-tool/matmul8.txt circuits/
# docker cp $id:/emp-tool/matmul16.txt circuits/
# docker cp $id:/emp-tool/matmul32.txt circuits/
# docker cp $id:/emp-tool/matmul48.txt circuits/
# docker cp $id:/emp-tool/matmul64.txt circuits/
# docker cp $id:/emp-tool/matmul100.txt circuits/
# docker cp $id:/emp-tool/matmul128.txt circuits/
docker cp $id:/emp-tool/matmul24.txt circuits/
docker rm -v $id