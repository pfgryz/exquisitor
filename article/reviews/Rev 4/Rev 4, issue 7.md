---
tags:
  - description
  - large
  - done
owner: PFG
---
## Issue
A dedicated “Model Limitations” subsection is recommended to ensure an unbiased evaluation of the work. This should highlight both the strengths and constraints of the proposed approach.
## Answer
Thank you for bringing this to our attention.

We have revised the article by adding Section 4.2.5 (p. 15), which includes the following text:
```
\subsubsection{Model Limitations}
                The model has several limitations. Its generalization is currently restricted to human skin microbiome data due to othe specifity of the training dataset; evaluation on other sample types would require additional studies. Being ANN-based, the method benefits from GPU acceleration, while CPU-only execution results in slower processing and longer pipeline runtime. This approach is optimized for sequences of similar length and show limited adaptability to sequences of substantially different lengths. Finally, the use of CNNs rather than RNNs or transformers may limit architectural flexibility and potentially affect performance, albeit providing faster training and inference times.
```