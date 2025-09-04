---
tags:
  - done
owner: PFG
---
## Issue
Most importantly, state-of-the-art methods are not included in the benchmark.
[[Rev 2, issue 5b]] [[Rev 3, issue 1]]
## Answer (draft) 
Rev1 miał podobne issue
Rev2 miał podobne issue
## Answer
Thank you for pointing this out.

Our pipeline increases the speed of BLAST by using a single representative sequence for each cluster, while still relying on BLAST internally.

We compared our method with approaches based on Needleman–Wunsch alignment and k-mer embeddings. Indeed, it would also be valuable to compare with tools such as Kraken2, MetaPhlAn, Centrifuge, and QIIME2. This is planned for the next stages of the project, since such experiments require significant time (tool setup, running large-scale experiments, adapting the code, and establishing fair comparison procedures). We did not include this in the current version.

However, we believe that the experiments we have conducted already demonstrate the usefulness of our method.