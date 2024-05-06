# 3. GitHub Wiki

## Table of Contents

- [3.1. NCBI "Nucleotide"](#31-ncbi-nucleotide)
- [3.2. EMBL - ENA/EBI](#32-embl---enaebi)
- [3.3. DDBJ](#33-ddbj)
- [3.4. CSC Genome Browser](#34-csc-genome-browser)
- [3.5. Ensembl](#35-ensembl)

## 3.1. NCBI "Nucleotide"

URL: https://www.ncbi.nlm.nih.gov/nucleotide/

API: https://www.ncbi.nlm.nih.gov/books/NBK25501/

- c. 1B records for sequences (300k of 1k length)
- shares data with:
  - EMBL (European Molecular Biology Laboratory)
  - DDBJ (DNA Data Bank of Japan)

- example usage: https://github.com/widdowquinn/Notebooks-Bioinformatics/blob/master/Biopython_NCBI_Entrez_downloads.ipynb
- integration with biopython in module `biopython.Entrez`
- requests limit: 3/s (10/s with token)
- well documented API with many examples
- can download whole BLAST and run locally


## 3.2. EMBL - ENA/EBI

URL: https://www.ebi.ac.uk/ena/browser/home

API: REST-API + browser

- documentation is a little buggy and not finished 
- requests limit: 50/s


## 3.3. DDBJ

URL: https://www.ddbj.nig.ac.jp/index-e.html

API: https://www.ddbj.nig.ac.jp/services/wabi-e.html

- shares data with ENA and Genbank
- integration with biopython in module `Bio.TogoWS`

## 3.4. CSC Genome Browser

URL: https://genome.ucsc.edu/

API: https://genome.ucsc.edu/FAQ/FAQdownloads.html

## 3.5. Ensembl

URL: https://www.ensembl.org/index.html

API: https://rest.ensembl.org/

- wrapper in Python to REST-API https://pypi.org/project/ensembl-rest/
- available example requests directly on page with documentation (Swagger)

---

[Back](index.md)