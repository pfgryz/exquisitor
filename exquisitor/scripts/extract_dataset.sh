#!/bin/bash

OUTPUT=data
DATASET=dataset.fq
DATASET_FORMAT=fastq
TRAINING_SIZE=1000000
VALIDATION_SIZE=10000

mkdir -p ./data/experiments

for X in $(seq 0 10);
do
    Y=$((1 << X))
    ./generate-data dataset \
        --output $OUTPUT \
        --input $DATASET \
        --file-format $DATASET_FORMAT \
        --experiments-file-name experiments/exp${X}.fasta \
        --experiments ${Y} \
        --exclude ./data/.exclude

    echo "Generated experiment #${X} of size ${Y}"
done

cp .exclude .exclude_experiments
./generate-data dataset \
    --output $OUTPUT \
    --input $DATASET \
    --file-format $DATASET_FORMAT \
    --training $TRAINING_SIZE \
    --validation $VALIDATION_SIZE