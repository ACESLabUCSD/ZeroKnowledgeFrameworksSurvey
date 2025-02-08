docker build -t circom .
id=$(docker create circom)
docker cp $id:/circom/src/results ./matmul/results
docker rm -v $id
