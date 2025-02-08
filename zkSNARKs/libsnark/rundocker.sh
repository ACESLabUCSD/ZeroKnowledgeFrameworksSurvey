#! /bin/bash

docker rm -f libsnark-test 2>/dev/null || true

docker run -it \
    --name libsnark-test \
    libsnark
