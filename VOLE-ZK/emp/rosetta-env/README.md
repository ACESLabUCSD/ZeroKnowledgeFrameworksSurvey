# Making Rosetta Environment

## Clone Repo
```
git clone https://github.com/nickshey/rosetta-env.git
```

## Build Image
```
docker build -t rosetta .
```

## Enter Image
```
docker run -t rosetta
```

## Train ResNet101
```
docker-compose up -f docker-compose-train.yaml
```

## Build and Train
```
docker build -t rosetta .; docker-compose -f docker-compose-train.yaml up
```

## Build Plaintext Testing Image and Run
```
docker build -f Dockerfile2 . -t rosetta:0.1; docker-compose -f docker-compose-plain-predict.yaml up
```

## Build ZK Testing Image and Run
```
docker build -f Dockerfile2 . -t rosetta:0.1; docker-compose -f docker-compose-zk-predict.yaml up
```