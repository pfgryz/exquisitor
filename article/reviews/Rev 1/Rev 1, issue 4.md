---
tags:
  - done
owner: PFG
---
## Issue
Why was the Needleman-Wunsch algorithm modified as presented? Simply for speed-up? It appears to function more as an edit-distance algorithm now.
## Answer
Thank you for bringing this to our attention.

The Needleman-Wunsch algorithm uses an additive scoring system, where higher values indicate greater similarity. In the contrastive learning algorithm we used, a measure of dissimilarity was required  - i.e, 0 for similar sequences and higher values for dissimilar ones. Simply inverting the sign was not possible, because negative values are not valid in the context of the clustering algorithm we employed. In current form it function as edit-distance algorithm.