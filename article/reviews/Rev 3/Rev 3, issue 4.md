---
tags:
  - done
owner: PFG
---
## Issue
There is a problem in benchmarking. First, the baseline methods are too simple and do not consider two types of gap penalties (Gotoh algorithm) or the two types of nucleotide substitutions, transition and transversion.
## Answer
Thank you for bringing this to our attention.

The baseline methods do not distinguish between different types of gap penalties, as implemented in the Gotoh algorithm, and therefore do not account for the differential treatment of insertions and deletions.

We have revised the article by adding Section 4.2.4 (p. 15), which includes the following text:
```
\subsubsection{Baseline Methods Limitations}
We used simplified baseline methods, which do not account for different gap penalties as implemented in algorithms such as Gotoh \cite{Gotoh1982}, nor do they distinguish between different types of nucleotide subsitutions, such as transitions and transversions.
```