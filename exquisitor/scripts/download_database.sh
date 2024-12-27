#!/bin/bash

# Create folder for BLASTn
mkdir /blast
cd /blast

# Download and extract executables
wget https://ftp.ncbi.nlm.nih.gov/blast/executables/blast+/LATEST/ncbi-blast-2.15.0+-x64-linux.tar.gz
tar -zxvf ncbi-blast-*.tar.gz
mv ncbi-blast-* executables

# Download database
mkdir db
cd db
update_blastdb.pl --decompress nt