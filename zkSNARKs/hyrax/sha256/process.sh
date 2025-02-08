#!/bin/bash

mkdir -p out
XLABEL='$\log_2 M$, number of leaves in Merkle tree'

process_file () {
    echo ${XLABEL} > out/${dres}.out
    for LOGLEAVES in "${psizes[@]}"; do
        LOGFILE=${logdir}/SHA256_64_merkle_${LOGLEAVES}.log
        if [ -f ${LOGFILE} ]; then
            readarray -t -n 4 RESULTS < <(egrep '^Proof size|^Prover runtime|^Verifier runtime|^Max memory' ${LOGFILE})
            PROOFSIZE=${RESULTS[0]##* size: }
            PROOFSIZE=${PROOFSIZE%% *}
            PTIME=${RESULTS[1]##* }
            VTIME=${RESULTS[2]##* }
            MAXMEM=${RESULTS[3]##*usage: }
            MAXMEM=${MAXMEM%% *}
        else
            PROOFSIZE=-1
            PTIME=-1
            VTIME=-1
            MAXMEM=0
        fi
        echo $LOGLEAVES $PROOFSIZE $PTIME $VTIME $MAXMEM
    done >> out/${dres}.out
}

dres=merkle
logdir=log
readarray -t psizes < <(seq 1 8)
process_file

for wval in w0 w2 w225 w25 w3 w35 unopt bccgp bullet; do
    dres=merkle_${wval}
    logdir=log_${wval}
    process_file
done
