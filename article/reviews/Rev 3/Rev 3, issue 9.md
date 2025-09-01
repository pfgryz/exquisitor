---
tags:
  - description
  - small
  - done
  - review
owner: PFG
---
## Issue
The authors might revise the network structure accordingly. An intuitive explanation for Equation (2) is needed, e.g., why “margin – cosine” is used for positives but “negative – margin” for negatives.
## Answer (draft) 
poprawki w tekście
## Answer
Thank you for bringing this to our attention.

Formula $[m_{pos} - s_{pos}]_{+}$ is used for positive pairs, because we want the loss to be zero when their similarity $s_{pos}$ exceeds the positive margin $m_{pos}$. Conversely, $[s_{neq} - m_{neq}]_{+}$ is used for negative pairs, because we want the loss to be zero when their similarity $s_{neq}$ is below the negative margin $m_{neq}$. The positive margin controls the minimum similarity between similar sequences, whereas the negative margin controls the maximum similarity between dissimilar sequences.
## Review
- czy dodawać jakieś zmiany w tekście? wydaje się dość jasne?
- ``` The positive margin controls the minimum similarity between similar sequences, whereas the negative margin controls the maximum similarity between dissimilar sequences.
```