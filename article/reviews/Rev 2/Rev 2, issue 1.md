---
tags:
  - description
  - small
  - done
owner: PFG
---
## Issue
The contrastive learning framework needs more detailed explanation, particularly regarding how representative sequences are selected and how sequences are clustered. The description about these two parts is not very clear.
## Answer
Thank you for bringing this to our attention.

After constructing a dissimilarity matrix between sequences, with non-negative values, the sequences are clustering using an external k-medoids algorithm. The medoids identified by the algorithm are considered good representatives of their respective clusters and are used in subsequent steps. In general, any algorithm that generates representatives of clusters can be used.

We have modified the article, by addicting section 3.2 (p. 9) with the following text
```
\subsection{Clustering Algorithm}
            In the experiments, we used an external k-medoids clustering algorithm \cite{Rust:kmedoids}. The medoids identified by this algorithm were used as cluster representatives and used in subsequent steps.
```