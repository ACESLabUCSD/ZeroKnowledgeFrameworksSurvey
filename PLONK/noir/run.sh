docker build -t noir .
id=$(docker create noir)
docker cp $id:/noir/matmul volume
docker rm -v $id