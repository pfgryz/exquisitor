---
tags:
  - done
owner: PFG
---
## Issue
In the performance measurement, it is unclear how the reference metagenomic identification was obtained—was it determined by BLASTn?
## Answer
Thank you for bringing this to our attention.

In the experiments, the reference metagenomic identification was obtained using BLAST. Although a ground truth was available, we used BLAST as the reference in order to emulate its results, since the goal of our method was to accelerate the identification process using BLAST.

We have modified the article, Section 3.5 (p. 12) by adding the following text
```
i.e., using raw BLAST results.
```