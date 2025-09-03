---
tags:
  - done
owner: PFG
---
## Issue
The authors should provide more details on computational complexity, including time and memory requirements, and clarify the reliance on specific benchmark datasets during evaluation.
## Answer
Thank you for bringing this to our attention.

The computational complexity of the ANN-based method depends linearly on the number of sequences, while memory usage grows quadratically with the number of sequences. Running the model requires at least 8 GB of RAM and a GPU with at least 6 GB of memory (e.g., GTX 1060) or better. It should be noted that the model was trained on a specific dataset, which limits its adaptability and generalization to new, unseen data.

We have revised the article by adding Section 4.2.5 (p. 15), which includes the following text:
```
\subsubsection{Model Limitations}
                The model has several limitations. Its generalization is currently restricted to human skin microbiome data due to othe specifity of the training dataset; evaluation on other sample types would require additional studies. Being ANN-based, the method benefits from GPU acceleration, while CPU-only execution results in slower processing and longer pipeline runtime. This approach is optimized for sequences of similar length and show limited adaptability to sequences of substantially different lengths. Finally, the use of CNNs rather than RNNs or transformers may limit architectural flexibility and potentially affect performance, albeit providing faster training and inference times.
