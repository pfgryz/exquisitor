# BLAST+ Setup

- download BLAST+ executables
```shell
wget https://ftp.ncbi.nlm.nih.gov/blast/executables/blast+/LATEST/ncbi-blast-2.15.0+-x64-linux.tar.gz
```

- extract archive with executable
```shell
tar -zxvf ncbi-blast-*.tar.gz
```

- add BLAST+ to path
```shell
nano ~/.bashrc
export PATH=$PATH:~/blast/ncbi-blast-2.15.0+/bin
```

## Database setup
```shell
mkdir ~/blast/db
cd ~/blast/db
```

- download part of database (stop after first part) or manually downl
```shell
update_blastdb.pl --decompress nt
```

- add DB to path
```shell
nano ~/.bashrc
export BLASTDB=~/blast/db
```