#!/bin/bash

BLASTN_PATH=/blast/executables/bin/blastn
BLASTDB_PATH=/blast/db

mkdir -p results
mkdir -p results/neural

for X in $(seq 0 15)
do
    Z=$(echo "sqrt(2^$X)" | bc -l)
    Z=$(echo "$Z/1" | bc)
    if (( Z < 1 )); then
      Z=1
    fi

    ./exquisitor-cli --log-level=DEBUG experiment \
        --resolution 5 \
        --max-duration 43200 \
        --output results/neural/result${X}.experiment \
        --command "run --input experiments/exp${X}.fasta --output results/neural/result${X}.search --blast ${BLASTN_PATH} --blast-db ${BLASTDB_PATH} --pipeline neural --clustering k-medoid --k ${Z} --model model --save-clusters"
done

echo "[EXPERIMENTS] @END"