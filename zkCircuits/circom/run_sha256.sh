docker build -t circom .
id=$(docker create circom)
rm -rf ./sha256/results
docker cp $id:/circom/src/results ./sha256/results
docker rm -v $id
