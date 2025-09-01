---
tags:
  - description
  - documentation
  - small
  - done
owner: PFG
---
## Issue
It is also unclear what the sizes of the training, validation, and test datasets are (Section 3.1.2).
## Answer
Thank you for bringing this to our attention.

The training, validation and test sets contain 1M, 10k, and 8192 sequences, respectively.

We have modified the article, Section 3.1.2 (p. 9), by adding the following text
```
The sizes of the subsets were as follows: training set - 1,000,000 sequences, validation set - 10,000 sequences, and test set - 8,192 sequences.
```