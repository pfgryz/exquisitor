---
tags:
  - done
owner: PFG
---
## Issue
Overall, the paper is clearly written and easy to follow for readers. They also make their code publicly available, which is good practice and will make it more widely used by the community. I do think some key details need to be expanded to justify some model choices and how well their method might generalize on new, unseen data.
[[Rev 4, issue 7]]
## Answer
Thank you for bringing this to our attention.

We selected CNN architecture, because RNN- and transformer-based methods typically require longer training times, slower inference, and greater computional resources. Our goal was to implement a relatively simple network to ensure the overall pipeline remains fast. The use of contrastive learning enabled the model to quantify the dissimilarity between sequences, which is essential for clustering them effectively. For measuring similarity, we opted for cosine dissimilarity, as it has favorable properties for separating this type of data and also slightly accelerates training https://arxiv.org/abs/2005.10242. 
Due to the use of a specific dataset, the method has limited adaptability for generalizing to new, unseen data. Additional studies would be required to evaluate its performance on such data.

We have revised the article by modifying Section 2.1.1 (p. 5) with the following text:
```
\paragraph{Design Rationale}
Convolutional Neural Networks (CNNs) were chosen over Recurrent Neural Networks (RNNs) for faster training and inference. Contrastive learning allows the model to measure sequence similarity, which supports clustering.
```
and by adding Section 4.2.5 (p. 16), which includes the following text:
```
\subsubsection{Model Limitations}
                The model has several limitations. Its generalization is currently restricted to human skin microbiome data due to othe specifity of the training dataset; evaluation on other sample types would require additional studies. Being ANN-based, the method benefits from GPU acceleration, while CPU-only execution results in slower processing and longer pipeline runtime. This approach is optimized for sequences of similar length and show limited adaptability to sequences of substantially different lengths. Finally, the use of CNNs rather than RNNs or transformers may limit architectural flexibility and potentially affect performance, albeit providing faster training and inference times.
```