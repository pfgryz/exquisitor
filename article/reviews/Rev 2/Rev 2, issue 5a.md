---
tags:
  - description
  - justification
  - medium
  - done
  - review
owner: PFG
---
## Issue
The performance evaluation metrics mainly focus on execution time and “quality”. However, the quality is not very well-defined. To evaluate metagenomic profiling, the core evaluation metrics include:
- precision: The proportion of correctly identified taxa among all predicted taxa.
- recall/sensitivity: The proportion of true taxa that were successfully identified.
- L1 Norm Error: Measure the absolute difference between predicted and true relative abundances.
[[Rev 1, issue 7]]
## Answer (draft)
indeks Jaccarda, FP, TP, Rev1, issue 7
## Answer
Thank you for bringing this to our attention.

We decided to use the Jaccard index instead of standard machine learning metrics, because it is not straightforward to obtain a confusion matrix with respect to a ground truth, as the reference is the BLAST result.

Reporting false positives or false negatives would be misleading, since FPs or FNs do not make sense in this context - what appears as a false positive relative to BLAST could actually be a true positive with respect to the real ground truth. Our goal was primarily to approximate BLAST results.
## Review
- czy coś dodawać do tekstu?