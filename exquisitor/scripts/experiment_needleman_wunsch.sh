#!/bin/bash

BLASTN_PATH=/blast/executables/bin/blastn
BLASTDB_PATH=/blast/db

mkdir -p results
mkdir -p results/nw

for X in $(seq 0 15)
do
    Z=$(echo "sqrt(2^$X)" | bc -l)
    echo $Z
    ./exquisitor-cli --log-level=DEBUG experiment \
        --resolution 5 \
        --max-duration 43200 \
        --output results/nw/result${X}.experiment \
        --command "run --input experiments/exp${X}.fasta --output results/nw/result${X}.search --blast ${BLASTN_PATH} --blast-db ${BLASTDB_PATH} --pipeline basic --clustering k-medoid --k ${Z} --gap-penalty 2"
done

echo "[EXPERIMENTS] @END"