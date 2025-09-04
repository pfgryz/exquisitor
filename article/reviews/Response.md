Dear Editor, Dear Reviewers

Thank you for considering our manuscript for publication and for providing
constructive feedback.
Hereunder, you will find our detailed replies to all your comments.
The changes are highlighted in the output of the latexdiff file attached to this
cover letter.

# Reviewer 1
---
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
## Anwer
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

Thank you for pointing out this problem with the text. We added a text to the 'Discussion' as depicted in our answer for issue 1.

# Reviewer 2
---
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

# Reviewer 3
---
# Issue 1
The proposed method aims to convert sequences into embedding vectors. There are many existing related packages, such as DNABERT. The authors should benchmark against these tools to demonstrate that their CNN model offers an advantage.
## Answer
Thank you for this valuable suggestion. 

We agree that benchmarking against existing sequence embedding tools such as DNABERT would provide a stronger basis for comparison. In the current work, we focused on evaluating our CNN-based approach against classical methods. Benchmarking against DNABERT and similar models is part of our planned future work, as such experiments require considerable computational resources and careful adaptation of the pipeline.

# Issue 2
There is also a problem in model training, specifically in the preparation of training data. The procedure for generating positive and negative sequences is unclear and lacks biological justification: why 0–20% mutation for positives and 20–80% for negatives, which type of DNA is used (CDS or non-coding), and whether synonymous or non-synonymous mutations are considered, since mutation rates differ among these regions. The authors need to redesign the mutation model with consideration of appropriate biological factors.
## Answer
Thank you for bringing this to our attention.

The model was trained on high-quality simulated data, and the data augmentation applied was not directly biologically motivated. Although the probabilities of the generated mutations do not strictly reflect biological reality, the model was still able to perform well in our experiments. Nevertheless, the lack of incorporation of biological information may have influenced the results and potentially limited the model's overall effectiveness.

# Issue 3
It is also unclear what the sizes of the training, validation, and test datasets are (Section 3.1.2).
## Answer
Thank you for bringing this to our attention.

The training, validation, and test sets contain 1M, 10k, and 8192 sequences, respectively.

We have modified the article, Section 3.1.2, by adding the following text
```
The sizes of the subsets were as follows: training set - 1,000,000 sequences, validation set - 10,000 sequences, and test set - 8,192 sequences.
```

# Issue 4
There is a problem in benchmarking. First, the baseline methods are too simple and do not consider two types of gap penalties (Gotoh algorithm) or the two types of nucleotide substitutions, transition and transversion.
## Answer
Thank you for bringing this to our attention.

The baseline methods do not distinguish between different types of gap penalties, as implemented in the Gotoh algorithm, and therefore do not account for the differential treatment of insertions and deletions.

We have revised the article by adding Section 4.2.4, which includes the following text:
```
\subsubsection{Baseline Methods Limitations}
We used simplified baseline methods, which do not account for different gap penalties as implemented in algorithms such as Gotoh \cite{Gotoh1982}, nor do they distinguish between different nucleotide substitutions, such as transitions and transversions.
```

# Issue 5
The authors might consider using MAFFT instead of the NW algorithm, as MAFFT applies the Fourier transform and is faster.
## Answer
Thank you for this hit. We plan to benchmark our tool with MAFFT. Our baseline algorithm (NW) was used mainly for quality checking. We modified the article in the section 'Future Research Directions', adding the sentence:

```
We plan to benchmark our method with MAFFT -- the algorithm using Fast Fourier Transform for alignment~\cite{katoh2013mafft}, instead of the NW algorithm.
```

# Issue 6
In the performance measurement, it is unclear how the reference metagenomic identification was obtained—was it determined by BLASTn?
## Answer
Thank you for bringing this to our attention.

In the experiments, the reference metagenomic identification was obtained using BLAST. Although a ground truth was available, we used BLAST as the reference in order to emulate its results, since the goal of our method was to accelerate the identification process using BLAST.

We have modified the article, Section 3.5, by adding the following text
```
i.e., using raw BLAST results.
```

# Issue 7
Most importantly, state-of-the-art methods are not included in the benchmark.
## Answer
Thank you for pointing this out.

Our pipeline increases the speed of BLAST by using a single representative sequence for each cluster, while still relying on BLAST internally.

We compared our method with approaches based on Needleman–Wunsch alignment and k-mer embeddings. Indeed, it would also be valuable to compare with tools such as Kraken2, MetaPhlAn, Centrifuge, and QIIME2. This is planned for the next stages of the project, since such experiments require significant time (tool setup, running large-scale experiments, adapting the code, and establishing fair comparison procedures). We did not include this in the current version.

However, we believe that the experiments we have conducted already demonstrate the usefulness of our method.

# Issue 8
The proposed training process is similar to the triplet neural network framework (Hoffer & Ailon, 2015, Deep metric learning using triplet network, in Similarity-Based Pattern Recognition, pp. 84–92), which classifies the data with the rule: anchor, a negative, and a positive sequence.
## Answer
Thank you for bringing this to our attention.

The proposed training formula is similar to the triplet neural network framework, but it is adapted to use cosine dissimilarity.

We have added a reference to the indicated work in Section 2.1.1.

# Issue 9
The authors might revise the network structure accordingly. An intuitive explanation for Equation (2) is needed, e.g., why “margin – cosine” is used for positives but “negative – margin” for negatives.
## Answer
Thank you for bringing this to our attention.

Formula $[m_{pos} - s_{pos}]_{+}$ is used for positive pairs, because we want the loss to be zero when their similarity $s_{pos}$ exceeds the positive margin $m_{pos}$. Conversely, $[s_{neq} - m_{neq}]_{+}$ is used for negative pairs, because we want the loss to be zero when their similarity $s_{neq}$ is below the negative margin $m_{neq}$. The positive margin controls the minimum similarity between similar sequences, whereas the negative margin controls the maximum similarity between dissimilar sequences.

We have modified the article, Section 2.1.1, by adding the following text:

```
The positive margin controls the minimum similarity between similar sequences, whereas the negative margin controls the maximum similarity between dissimilar sequences.
```

# Issue 10
On page 7, the value of the hyperparameter weight decay w is missing.
## Answer
Thank you for pointing this out.
The weight decay is set to $10^{-4}$.
We updated Section 2.1.1 accordingly.

# Issue 11
Although it is good to have the GitHub link, detailed usage instructions for the package are needed.
## Answer
Thank you for bringing this to our attention.

We have added documentation on GitHub, including step-by-step instructions for training the model and running the experiments. Additionally, we uploaded the trained model to a cloud drive and provided the link in the README.md file."

# Reviewer 4
---
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

---

Kind regards,
Patryk Gryz, Robert Nowak