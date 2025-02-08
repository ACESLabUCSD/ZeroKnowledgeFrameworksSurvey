docker build -t zokrates .
id=$(docker create zokrates)
docker cp $id:/home/zokrates/. volume
docker rm -v $id