// HEADER

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

// FOOTNOTE