#!/bin/bash

mkdir -p out
XLABEL='$\log_2 N$, number of copies'

process_file () {
    echo ${XLABEL} > out/${oname}.out
    for LOGCOPIES in "${psizes[@]}"; do
        LOGFILE=${logdir}/matmult_${dres}x${dres}_${LOGCOPIES}.log
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
        echo $LOGCOPIES $PROOFSIZE $PTIME $VTIME $MAXMEM
    done >> out/${oname}.out
}


dres=128
logdir=log
oname=128
readarray -t psizes < <(seq 1 9)
process_file

for w in bccgp bullet unopt; do
    logdir=log_${w}
    oname=128_${w}
    process_file
done