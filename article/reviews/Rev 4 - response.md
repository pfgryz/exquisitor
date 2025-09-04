// HEADER

# Issue 1
The abstract could be made more concise and sharply focused on the core contributions namely the use of contrastive learning within an ANN for representative sequence selection, its performance gains over classical methods, and the development of the Exquisitor tool.
## Answer
Thank you for this comment, we updated the abstract focusing on the cored contributions.  
Moreover, we corrected 4 grammatical errors.

# Issue 2
The manuscript does not sufficiently address the potential limitations of the proposed ANN-based system (e.g., generalizability to diverse datasets, dependence on GPU acceleration, or adaptability to varying sequence lengths). These should be discussed to present a balanced and transparent view.
## Answer
Thank you for bringing this to our attention.

We have revised the article by adding Section 4.2.5, which includes the following text:
```
\subsubsection{Model Limitations}
The model has several limitations. Its generalisation is currently restricted to human skin microbiome data due to the specificity of the training dataset; evaluation on other sample types would require additional studies. Being ANN-based, the method benefits from GPU acceleration, while CPU-only execution results in slower processing and longer pipeline runtime. This approach is optimised for sequences of similar length and shows limited adaptability to sequences of substantially different lengths. Finally, using CNNs rather than RNNs or transformers may limit architectural flexibility and affect performance, albeit providing faster training and inference times.
```

# Issue 3
The authors should provide more details on computational complexity, including time and memory requirements, and clarify the reliance on specific benchmark datasets during evaluation.
## Answer
Thank you for bringing this to our attention.

The computational complexity of the ANN-based method depends linearly on the number of sequences, while memory usage grows quadratically with the number of sequences. Running the model requires at least 8 GB of RAM and a GPU with at least 6 GB of memory (e.g., GTX 1060) or better. It should be noted that the model was trained on a specific dataset, which limits its adaptability and generalization to new, unseen data.

We added a text to Section 4.2.5, as depicted in our answer for issue 2.

# Issue 4
The literature review should be expanded to compare the proposed method with recent studies using bioinformatics datasets for classification problems, such as s13040-024-00415-8, s12859-024-05917-0, and s12859-024-05978-1. This will help situate the contribution within the current state-of-the-art.
## Answer
We cited the proposed work in 'Future Research Directions' section; we now mention the feature selection techniques. We also cited the work from issue 8.

# Issue 5
While the manuscript’s language is generally understandable, there are grammatical and stylistic issues that should be corrected to improve readability and ensure technical clarity.
## Answer
Thank you for this issue. We rechecked the text, correcting over 30 grammatical errors and typos.

# Issue 6
The paper would benefit from strengthening its technical depth. Currently, some sections are presented in a more theoretical or descriptive style; these should be rewritten to include more technical detail, experimental reasoning, and methodological rigor.
## Answer
Thank you for bringing this to our attention. In the revised version of the paper, we have included a number of improvements and clarifications:  
- We provided a better description of the training procedure  
- We added a clearer explanation of hyperparameter selection and included missing hyperparameters  
- We improved the descriptions of the tables for better readability  
- We explained the design rationale, including why CNN was chosen, and why contrastive learning was used  
- We added details about dataset sizes  
- We included information about the clustering algorithm usage  
- We described the process of creating $k$-embeddings  
- We conducted an additional experiment on unseen data  
- We added information about baseline method limitations  
- We discussed the limitations of our model

# Issue 7
A dedicated “Model Limitations” subsection is recommended to ensure an unbiased evaluation of the work. This should highlight both the strengths and constraints of the proposed approach.
## Answer
Thank you for bringing this to our attention.
We added a text to Section 4.2.5, as depicted in our answer for issue 2.

# Issue 8 
The “Future Work” section could be enriched by discussing how the method might be extended to large-scale bioinformatics tasks using parallel computing for big data in gene regulatory networks. For instance, parallel deep learning approaches described in s00607-025-01441-y could be considered.
## Answer
Yes, we cited the proposed work in 'Future Research Directions' section, because we plan to use our tool in a distributed computing environment. We also cited one work from issue 4.


// FOOTNOTE