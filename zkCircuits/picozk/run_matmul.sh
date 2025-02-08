docker build -t picozk .
id=$(docker create picozk)
docker cp $id:/picozk/ ./picozk/results
docker rm -v $id
