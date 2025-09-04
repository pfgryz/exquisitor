---
tags:
  - done
owner: PFG
---
## Issue
More importantly, the authors should compare their method with existing well-known methods, such as kraken2, MetaPhlAn, Centrifuge, and QIIME2. And there are several ANN-based approaches, such as VAMB, MetaDecoder, and DeepBin. Please also include these methods for performance comparison.
[[Rev 1, issue 5]]
## Answer
Thank you for pointing this out.

Our pipeline increases the speed of BLAST by using a single representative sequence for each cluster, while still relying on BLAST internally.

We compared our method with approaches based on Needleman–Wunsch alignment and k-mer embeddings. Indeed, it would also be valuable to compare with tools such as Kraken2, MetaPhlAn, Centrifuge, and QIIME2. This is planned for the next stages of the project, since such experiments require significant time (tool setup, running large-scale experiments, adapting the code, and establishing fair comparison procedures). We did not include this in the current version.

However, we believe that the experiments we have conducted already demonstrate the usefulness of our method.