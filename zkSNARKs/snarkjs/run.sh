docker build -t circom .
id=$(docker create circom)
docker cp $id:/circom/src/results results
docker rm -v $id