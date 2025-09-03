---
tags:
  - done
owner: PFG
---
## Issue
I would like to see performance on an external validation set used with the method. The CAMI II dataset used only comes from the human skin microbiome and was further modified by the authors to create the positive and negative sequences. What specific steps were performed to prevent data leakage here? How does the method perform on a different type of sample, and are there biologically validated data that could be used to assess accuracy? If this isn’t possible, it would be good for readers to know why.
[[Rev 4, issue 7]]
## Answer (draft)
Podczas badań dbaliśmy o rozdzielenie zbiorów trenujących i testujących. Recenzent ma racje, że mogły zaistnieć zjawiska typu 'data leakage'. Aby dodatkowo zwalidować rozwiązanie wzieliśmy dodatkową próbkę z CAMI II z jelit i powtórzyliśmy pipeline. Wynik (podobieństwo) jest porówywalny z tym co pokazaliśmy w pracy.
## Answer
Thank you for bringing this to our attention.

During the study, we ensured that the data were split into training, validation and tet sets. Homever, data leakage may have occurred because we did not verify the presence of duplicate records in the input data.

We have modified the article, Section 4.2.2 (p. 15), by adding the following text
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

  

                The clustering performance was evaluated by computing Adjusted Mutual Information (AMI), Normalized Mutual Information (NMI), and the Fowlkes--Mallows Index (FMI) between the obtained clusters and the ground truth.

  

                \paragraph{Results}

                The evaluation metrics (AMI, NMI, FMI) for clustering performance are summarized in Table\ref{Table:Experiment:ClusterQuality}.

  

                \begin{table}\centering

                    \caption{Comparison of clustering quality on seen and unseen data.}\label{Table:Experiment:ClusterQuality}

  

                    \begin{tabular}{|c|c|c|c|}

                        \hline

                        \textbf{Sample} & \textbf{AMI} & \textbf{NMI} & \textbf{NMI} \\ \hline

                        Sample 1 (seen) & 0.288 & 0.286 & 0.424 \\ \hline

                        Sample 4 (unseen) & 0.219 & 0.243 & 0.379 \\ \hline

                    \end{tabular}

                \end{table}

  

                \paragraph{Conclusions}

                Although the clustering performance on the unseen data is not optimal, it remains comparable to the results obtained for the training data. This suggests that the model's generalization capabilities are present but constrained, and that the outcomes are not merely random.
```