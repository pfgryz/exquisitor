#!/bin/bash

BLASTN_PATH=/blast/execultables/bin/blastn
BLASTDB_PATH=/blast/db

mkdir -p results

for X in $(seq 0 10)
do
    ./exquisitor-cli --log-level=DEBUG experiment \
        --resolution 5 \
        --output results/result${X}.experiment \
        --command "search --input experiments/exp${X}.fasta --output results/result${X}.search --blast ${BLASTN_PATH} --blast-db ${BLASTDB_PATH}"
done

echo "[EXPERIMENTS] @END"