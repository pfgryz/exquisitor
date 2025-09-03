python3 -m venv .venv
source .venv/bin/activate


mkdir -p raw

wget -q -O raw/sample_1.tar.gz https://frl.publisso.de/data/frl:6425518/airskinurogenital/sample_1.tar.gz
wget -q -O raw/sample_4.tar.gz https://frl.publisso.de/data/frl:6425518/airskinurogenital/sample_4.tar.gz

mkdir -p extracted/sample_1
tar -xvzf raw/sample_1.tar.gz -C extracted/sample_1 --strip-components=1
rm -rf extracted/sample_1/bam
rm -rf extracted/sample_1/contigs

mkdir -p extracted/sample_4
tar -xvzf raw/sample_4.tar.gz -C extracted/sample_4 --strip-components=1
rm -rf extracted/sample_4/bam
rm -rf extracted/sample_4/contigs

gunzip extracted/sample_1/reads/anonymous_reads.fq.gz
gunzip extracted/sample_1/reads/reads_mapping.tsv.gz

gunzip extracted/sample_4/reads/anonymous_reads.fq.gz
gunzip extracted/sample_4/reads/reads_mapping.tsv.gz

python3 prepare_datasets.py sample_1
python3 prepare_datasets.py sample_4

python3 split_datasets.py sample_1
python3 split_datasets.py sample_4

./infer sample_1
./infer sample_4

python3 score.py sample_1
python3 score.py sample_4
