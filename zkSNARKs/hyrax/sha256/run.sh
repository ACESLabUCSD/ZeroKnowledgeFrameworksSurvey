#!/bin/bash

if [[ $1 == "" ]]; then
    echo "usage: ./run.sh k"
    exit -1
fi

RUN="pypy -OO ./run_fennel.py -z 3 -C m191 -n 0 -w $1"
LOGDIR=log_w$(tr -d '.' <<< $1)
mkdir -p ${LOGDIR}

# single SHA256 instance
echo "running single SHA256 instance"
${RUN} -p SHA256_4.pws -r SHA256_4_rdl.pws -c 4 -L ${LOGDIR}/SHA256_4.log -R 7261,8324

# merkle instances
for i in {1..8}; do
    echo "running Merkle $i"
    ${RUN} -p SHA256_64.pws -r SHA256_64_merkle_${i}_rdl.pws -c $((${i}+1)) -L ${LOGDIR}/SHA256_64_merkle_${i}.log -R 306,7353
done
