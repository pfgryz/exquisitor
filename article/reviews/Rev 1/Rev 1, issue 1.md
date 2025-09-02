---
tags:
  - done
owner: RN
---
## Issue
I do think the method contribution is more general and not necessarily limited to metagenomics. For example, genome assembly (read binning, de-duplication, etc.) might benefit from this approach.
## Answer
Thank you, extend the discussion by the mentioned areas

We added the following sentence to the 'Discussion'
```
Our pipeline increases the speed of BLAST by using a single sequence that represents a cluster and it uses the BLAST internally. The presented method is not limited to metagenomics analysis; generally, it replaces BLAST in tasks when clusters of sequences are analysed. Therefore, genome assembly, identifying cell subtypes, and phylogenetic trees might benefit from the presented approach.
```
