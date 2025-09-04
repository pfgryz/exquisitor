# Issue 1
The contrastive learning framework needs more detailed explanation, particularly regarding how representative sequences are selected and how sequences are clustered. The description about these two parts is not very clear.
## Answer
Thank you for bringing this to our attention.

After constructing a dissimilarity matrix between sequences, with non-negative values, the sequences are clustered using an external k-medoids algorithm. The medoids identified by the algorithm are considered good representatives of their respective clusters and are used in subsequent steps. In general, any algorithm that generates representatives of clusters can be used.

We have modified the article by adding Section 3.2, with the following text:
```
\subsection{Clustering Algorithm}
In the experiments, we used an external k-medoids clustering algorithm \cite{Rust:kmedoids}. The medoids identified by this algorithm were used as cluster representatives and used in subsequent steps.
```

# Issue 2
Why do the authors use cosine dissimilarity to calculate the dissimilarity between sequence representations?
## Answer
Thank you for bringing this to our attention.

We used cosine dissimilarity because it has better properties for segregating this type of data, and it also slightly speeds up training.
https://arxiv.org/abs/2005.10242

# Issue 3
The training procedure, including hyperparameter selection, optimization algorithm, and convergence criteria, requires elaboration.
## Answer
Thank you for bringing this to our attention.

The model was trained for 100 epochs using data split into training and validation sets, while a separate test set was employed in subsequent experiments. Hyperparameters were selected through grid search, resulting in the following configuration: a learning rate of $10^{-6}$, weight decay of $10^{-4}$, $\gamma=0.99999$, a batch size of 256, and a dropout rate of 0.4. The optimization algorithm used was AdamW, which incorporates decoupled weight decay regularization. This choice provides more stable convergence and improved generalization compared to the standard Adam optimizer. Convergence was determined based on the absence of significant improvements in the validation loss.

We have modified the article, Section 2.1.1, by adding the following text
```
Learning
[...]
and the training was performed for 100 epochs. Both weight decay and the exponential learning rate schedule were employed to mitigate the risk of overfitting. The convergence criterion was defined as the absence of further improvement in the validation loss.
[...]

Parameters
[...]
The parameter search was performed using grid search.
[...]
a batch size of 256
```

# Issue 4
The rationale for specific design choices should be justified with supporting evidence or theoretical foundations.
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

# Issue 5a
The performance evaluation metrics mainly focus on execution time and “quality”. However, the quality is not very well-defined. To evaluate metagenomic profiling, the core evaluation metrics include:
- precision: The proportion of correctly identified taxa among all predicted taxa.
- recall/sensitivity: The proportion of true taxa that were successfully identified.
- L1 Norm Error: Measure the absolute difference between predicted and true relative abundances.
## Answer
Thank you for bringing this to our attention.

We decided to use the Jaccard index instead of standard machine learning metrics because it is not straightforward to obtain a confusion matrix with respect to a ground truth, as the reference is the BLAST result.

Reporting false positives or false negatives would be misleading, since FPs or FNs do not make sense in this context - what appears as a false positive relative to BLAST could actually be a true positive with respect to the real ground truth. Our goal was primarily to approximate BLAST results.

We have modified the article, Section 3.5 (p. 12), by adding the following text:
```
as standard classification metrics relying on false positives/negatives are not well-defined when BLAST results serve as the reference
```

# Issue 5b
More importantly, the authors should compare their method with existing well-known methods, such as kraken2, MetaPhlAn, Centrifuge, and QIIME2. And there are several ANN-based approaches, such as VAMB, MetaDecoder, and DeepBin. Please also include these methods for performance comparison.
## Answer
Thank you for pointing this out.

Our pipeline increases the speed of BLAST by using a single representative sequence for each cluster, while still relying on BLAST internally.

We compared our method with approaches based on Needleman–Wunsch alignment and k-mer embeddings. Indeed, it would also be valuable to compare with tools such as Kraken2, MetaPhlAn, Centrifuge, and QIIME2. This is planned for the next stages of the project, since such experiments require significant time (tool setup, running large-scale experiments, adapting the code, and establishing fair comparison procedures). We did not include this in the current version.

However, we believe that the experiments we have conducted already demonstrate the usefulness of our method.

# Issue 6
Code Availability: The authors mention developing "ENpquizitor," a command-line tool implementing the proposed method. However, no clear information is provided about public access to the code repository. There is no reference to reproducibility materials like configuration files or trained models. I strongly recommend making the code publicly available with comprehensive documentation.
## Answer
Thank you for bringing this to our attention.

We have added documentation on GitHub, including step-by-step instructions for training the model and running the experiments. Additionally, we uploaded the trained model to a cloud drive and provided the link in the README.md file."