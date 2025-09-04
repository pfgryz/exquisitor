# Issue 1 
The contrastive learning framework needs more detailed explanation, particularly regarding how representative sequences are selected and how sequences are clustered. The description about these two parts is not very clear.
## Answer
Thank you, extend the discussion to the mentioned areas

We added the following sentence to the 'Discussion'
```
Our pipeline increases the speed of BLAST by using a single sequence that represents a cluster, and it uses BLAST internally. The presented method is not limited to metagenomic analysis; generally, it replaces BLAST in tasks when clusters of sequences are analysed. Therefore, genome assembly, identifying cell subtypes, and phylogenetic trees might benefit from the presented approach.
```

# Issue 2
Overall, the paper is clearly written and easy to follow for readers. They also make their code publicly available, which is good practice and will make it more widely used by the community. I do think some key details need to be expanded to justify some model choices and how well their method might generalize on new, unseen data.
## Answer
Thank you for bringing this to our attention.

We selected the CNN architecture because RNN- and transformer-based methods typically require longer training times, slower inference, and greater computational resources. Our goal was to implement a relatively simple network to ensure the overall pipeline remains fast. The use of contrastive learning enabled the model to quantify the dissimilarity between sequences, which is essential for clustering them effectively. For measuring similarity, we opted for cosine dissimilarity, as it has favorable properties for separating this type of data and also slightly accelerates training https://arxiv.org/abs/2005.10242.
Due to the use of a specific dataset, the method has limited adaptability for generalizing to new, unseen data. Additional studies would be required to evaluate its performance on such data.

We have revised the article by modifying Section 2.1.1 with the following text:
```
\paragraph{Design Rationale}
Convolutional Neural Networks (CNNs) were chosen over Recurrent Neural Networks (RNNs) for faster training and inference. Contrastive learning allows the model to measure sequence similarity, which supports clustering.
```
and by adding Section 4.2.5, which includes the following text:
```
The model has several limitations. Its generalisation is currently restricted to human skin microbiome data due to the specificity of the training dataset; evaluation on other sample types would require additional studies. Being ANN-based, the method benefits from GPU acceleration, while CPU-only execution results in slower processing and longer pipeline runtime. This approach is optimised for sequences of similar length and shows limited adaptability to sequences of substantially different lengths. Finally, using CNNs rather than RNNs or transformers may limit architectural flexibility and affect performance, albeit providing faster training and inference times.
```

# Issue 3
There are some minor grammatical errors that still remain (e.g., "This process require significant…" → "This process requires significant…", etc.)
## Answer
Thank you for this issue. We rechecked the text, correcting over 30 grammatical errors and typos. The mentioned error was corrected as well.

# Issue 4
Why was the Needleman-Wunsch algorithm modified as presented? Simply for speed-up? It appears to function more as an edit-distance algorithm now.
## Answer
Thank you for bringing this to our attention.

The Needleman-Wunsch algorithm uses an additive scoring system, where higher values indicate greater similarity. In the contrastive learning algorithm we used, a measure of dissimilarity was required  - i.e, 0 for similar sequences and higher values for dissimilar ones. Simply inverting the sign was not possible because negative values are not valid in the context of the clustering algorithm we employed. In its current form, it functions as an edit-distance algorithm.

# Issue 5
How does the k-mer embedding method employed here compare to popular tools like Centrifuge, Kraken, etc.? Does it represent how this might work against state-of-the-art?
## Answer
Thank you for bringing this to our attention.

In our study, the k-mer embedding method is treated as one of the comparative approaches. This method is based on the use of k-mers, a principle also employed by popular tools such as Centrifuge and Kraken. For the purposes of this study, we consider our implementation of the k-mer embedding approach to reflect current state-of-the-art practices.

# Issue 6
I think the figures and tables could benefit from more descriptive titles to stand on their own. I often had to refer to the main text to fully understand them.
## Answer
Thank you for this comment. We modified all figure and table captions to make them self-descriptive.

# Issue 7
The term “classification quality” should be more clearly defined for readers here – especially those less familiar with the CAMI Classification Challenges. Can the authors also present performance – perhaps on an additional dataset if easier – to represent classification and speed performance? In the metagenomics use case, this could be done using standard machine learning metrics such as precision, recall, F1-score, accuracy, etc. If just focusing on clustering accuracy, alignment scores, bit scores, percent identity, etc. may be more appropriate. The use of the Jaccard index here is helpful but a bit harder to interpret, as details on the type of error (number of false positives vs. false negatives) are lost.
## Answer
Thank you for bringing this to our attention.

We decided to use the Jaccard index instead of standard machine learning metrics because it is not straightforward to obtain a confusion matrix with respect to a ground truth, as the reference is the BLAST result.

Reporting false positives or false negatives would be misleading, since FPs or FNs do not make sense in this context - what appears as a false positive relative to BLAST could actually be a true positive with respect to the real ground truth. Our goal was primarily to approximate BLAST results.

We have modified the article, Section 3.5, by adding the following text:
```
as standard classification metrics relying on false positives/negatives are not well-defined when BLAST results serve as the reference
```

# Issue 8
The authors assume in 3.3.1 that “sequence clustering is deterministic” – please elaborate here. Is this because the method actually is deterministic? Or due to the time constraints you could only perform each experimental setting once? If the latter, it would be nice to see some confidence intervals, standard error, etc. for at least a few of the mid-range sample sizes to understand how much each run might differ.
## Answer
Thank you for bringing this to our attention.

In the text, we incorrectly referred to the described feature as a property of the method, whereas it is actually an implementation detail. In the experiments, a fixed seed was used. In general, the method is not deterministic, since clustering is performed by an external algorithm; however, we do not expect significant differences within groups. Each experimental setting was run only once due to time constraints. In the case of execution time, most time is spent on the BLAST query and the clustering algorithm.

We have modified the article, Section 3.3.1 and Section 3.3.2 by adding the following text:
```
Sequences clustering is deterministic with a fixed seed.
```

# Issue 9
I would like to see performance on an external validation set used with the method. The CAMI II dataset used only comes from the human skin microbiome and was further modified by the authors to create the positive and negative sequences. What specific steps were performed to prevent data leakage here? How does the method perform on a different type of sample, and are there biologically validated data that could be used to assess accuracy? If this isn’t possible, it would be good for readers to know why.
## Answer
Thank you for bringing this to our attention.

During the study, we ensured that the data were split into training, validation, and test sets. However, data leakage may have occurred because we did not verify the presence of duplicate records in the input data.

We have modified the article, Section 4.2.2, by adding the following text
```
Additionally, the absence of duplicate checking could have resulted in data leakage, potentially biasing the evaluation.
```
and added Section 3.4.3 with following text
```
\subsubsection{Performance on an unseen dataset}

\paragraph{Objective}
Assessing the quality of clustering on unseen data from a different sample.

\paragraph{Preparation}
We obtained an additional sample from a different body site (CAMI II Toy Human Microbiome Project\footnote{Sample 4, airways}). Processing steps were applied both to the training samples and to this new sample:

\begin{enumerate}
  \item Sequences in each sample were mapped to taxonomic identifiers (tax\_id).
  \item From each sample, a subset of 10,000 sequences was randomly selected, ensuring that the dominant taxon did not exceed 50\% of the subset.
  \item Sequence embeddings were generated using the ANN model.
  \item For each sample, clustering was performed with the k-medoids algorithm, where $k$ was chosen according to the number of true taxa in the subset.
  \item Clusters corresponding to taxa were treated as the ground truth.
\end{enumerate}

The clustering performance was evaluated by computing Adjusted Mutual Information (AMI), Normalised Mutual Information (NMI), and the Fowlkes--Mallows Index (FMI) between the obtained clusters and the ground truth.

\paragraph{Results}
The evaluation metrics (AMI, NMI, FMI) for clustering performance are summarised in Table~\ref{Table:Experiment:ClusterQuality}.

\paragraph{Conclusions}
Although the clustering performance on the unseen data is not optimal, it remains comparable to the results obtained for the training data. It suggests that the model's generalisation capabilities are present but constrained, and that the outcomes are not merely random.
```

# Issue 10
The sequences are trimmed to 150 bp – how is the trimming performed? Always from the 5’ or 3’ end? Randomly? What if a key motif is cleaved off – how does this impact performance? More detail is needed here on how this is performed, why 150 bp was selected, and what this means in terms of impacts on downstream classification. Further details on the k-mer size and embedding parameter choices should also be included.
## Answer
Thank you for bringing this to our attention.

The description in the article is a bit redundant. In our study, we used an input dataset with reads of exactly 150 bp each, so it was not necessary to trim the sequences to this length. The 150 bp length was determined by the input dataset, but the method is applicable to sequences of any length. In the case of longer sequences, trimming should be performed from 5' to 3', if the read is shorter, it is filled by random symbols. The method is designed for sequences of similar length.

The k-mer embedding is created by constructing a vector of size $4^k$, where each k-mer corresponds to a specific position in the vector. The number of occurrences of each k-mer in the sequence is then recorded at its corresponding position.

We have modified the article, Section 3.2.2, by adding the following text:
```
Embedding is created by constructing a vector of size $4^k$, where each $k$-mer corresponds to a specific position in the vector. The number of occurrences of each $k$-mer in the sequence is then recorded at its corresponding position.
```
and Section 2.1.1, with following footnote:
```
The sequence length of 150 is a method parameter and can be adjusted depending on the application.
```

# Issue 11
It’s still unclear to me if it is better to use this method or something else like Kraken or Centrifuge for a given metagenomics classification task – it would be nice to see a direct comparison (accuracy and speed) with the final pipeline against popular methods in the field on a standardized dataset.
## Answer
Kraken is fast, BLAST is more accurate but slow. Our pipeline increases the speed of BLAST by using a single sequence that represents a cluster. It uses the BLAST internally. The representative sequence allows running BLAST externally, and the searching results and their similarity metrics can be stored.

<<<<<<< HEAD
Thank you for pointing out this problem with the text. We added a text to the 'Discussion' as depicted in our answer for issue 1.
=======
Thank you for pointing out this problem with the text. We added a text to the 'Discussion' as depicted in our answer for issue 1.

// FOOTNOTE
>>>>>>> dedfe097394761cfad51234811cf0539389b6573
