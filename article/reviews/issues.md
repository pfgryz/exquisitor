Dear Dr Gryz,

Your manuscript, "Accelerating Metagenomic Identification of DNA Sequences Using Artificial Neural Networks
", has now been assessed.

We invite you to revise your paper, carefully addressing the comments from the reviewers and the editor. When your revision is ready, please submit the updated manuscript and a point-by-point response. This will help us move to a swift decision.

Editor Comments
---------------

"After carefully considering the reviewers’ feedback, we recommend a major revision of the manuscript. The reviewers concur that the work introduces a potentially valuable approach to sequence clustering and metagenomic identification by combining contrastive learning with neural network embeddings. The topic is timely, the approach could be of interest to the audience, and the availability of a prototype tool is appreciated.

However, in its current form, the manuscript does not meet the standards required for publication. It requires substantial rewriting and expansion, with methodological clarifications, more rigorous evaluation, and clearer presentation. Please refer to the reviewers' comments for more details."


-Santiago Marco-Sola

We recommend submitting all revisions within the mentioned deadline.

If you need more time, please contact us and include your submission ID.

Kind regards,

Evan Smithson
Editor
BMC Bioinformatics


Reviewer 1
----------
Reviewer 1

# Rev1, preliminary

Gryz and Nowak present Exquisitor, a pipeline to accelerate fixed-length DNA sequence clustering using an artificial neural network (ANN). Their title suggests how this can be used to enhance metagenomic identification (downstream via BLASTn or similar methods), but their contribution is mostly upstream of this final classification task – identifying key cluster representatives via contrastive learning to quickly and accurately assign (taxonomic) labels. This is an important contribution, as the work of metagenomic classification is widely performed, computationally intensive, and the rate of DNA sequence contribution to public repositories continues to grow rapidly.

## Answer

Thank you for appreciating our work

# Rev1, issue 1

I do think the method contribution is more general and not necessarily limited to metagenomics. For example, genome assembly (read binning, de-duplication, etc.) might benefit from this approach.

## Answer, RN, +

Thank you, extend the discussion by the mentioned areas
We added the following sentence to the 'Discussion'

The presented method is not limited to metagenomics analysis; generally, it replaces BLAST in tasks when clusters of sequences are analysed. Therefore, genome assembly, identifying cell subtypes, and phylogenetic trees might benefit from the presented approach.

TODO: dodać do artykułu

# Rev1, issue 2

Overall, the paper is clearly written and easy to follow for readers. They also make their code publicly available, which is good practice and will make it more widely used by the community. I do think some key details need to be expanded to justify some model choices and how well their method might generalize on new, unseen data.

## Answer, PG

Thank you for appreciate our work
Porównać różne podejścia, ale nie robiąc badań, 
Można tak, inaczej, np. GRU, LSTM, ale są wolniejsze,

Dla danych które nie są z dystrybucji, należałoby zrobić oddzielną pracę, z wstępnych badań wynika,
że model radzi sobie przeciętnie.

# Rev1, issue 3

Minor Concerns:

- There are some minor grammatical errors that still remain (e.g., "This process require significant…" → "This process requires significant…", etc.)

## Answer, RN

konkretnie te 2 poprawić, dodatkowo Grammarly,
napisać, że poprawiliśmy XXX błędów gramatycznych

# Rev1, issue 4

- Why was the Needleman-Wunsch algorithm modified as presented? Simply for speed-up? It appears to function more as an edit-distance algorithm now.

## Answer, PG

Dziękujemy za zwrócenie uwagi.

NW ma ocenę addtytywną, czym wyższa wartość tym większe podobieństwo,
natomiast w naszym algorytmie potrzebna była miara niepodobieństwa, czyli 0 gdy podobne, wysokie wartości gdy niepodobne.

Nie dało się po prostu odwrócić znaku, ponieważ wartości w macierzy pomocniczej (wewnętrznej) mogą produkować ujemne wartości, a to jest zabronione w dostarczonej przez nas wersji algorytmu do uczenia kontrastowego

Zmodyfikowaliśmy artykuł, sekcja X, str Y, dodając następujący tekst

''xxxx''


# Rev1, issue 5

- How does the k-mer embedding method employed here compare to popular tools like Centrifuge, Kraken, etc.? Does it represent how this might work against state-of-the-art?

## Answer, PG

W naszej pracy metoda 'k-mer embedding' jest traktowana jako jedna z metod porównawczych, do której się odnosimy. Metoda ta reprezentuje to co jest w popularnych narzędziach, takich jak tools like Centrifuge, Kraken, natomiast narzędzia mają tę metodę zoptymalizowaną.

W naszej pracy przyjeliśmy, że proponowana przez nas implementacja 'k-mer embedding' to 'state-of-the-art'. W artykule akapit:

'k-mer embedding' jest stosowany m.in. w narzędziach Centrafuge, Kraken i innych.

//Jeżeli recenzent uważa za zasadne, to możemy dodatkowo uruchomić Kraken...

# Rev1, issue 6, RN

- I think the figures and tables could benefit from more descriptive titles to stand on their own. I often had to refer to the main text to fully understand them.

## answer

przejrzeć, poprawić, podziękować

# Rev1, issue 7, PG

Major Concerns:

- The term “classification quality” should be more clearly defined for readers here – especially those less familiar with the CAMI Classification Challenges. Can the authors also present performance – perhaps on an additional dataset if easier – to represent classification and speed performance? In the metagenomics use case, this could be done using standard machine learning metrics such as precision, recall, F1-score, accuracy, etc. If just focusing on clustering accuracy, alignment scores, bit scores, percent identity, etc. may be more appropriate. The use of the Jaccard index here is helpful but a bit harder to interpret, as details on the type of error (number of false positives vs. false negatives) are lost.

## answer

lepej wyjaśnić dlaczego taka miara, że nie da się w prosty sposób macierzy pomyłek, bo referencją jest wynik Blast, nie ma 'ground truth', więc miary typu FP, FN mogą wprowadzać czytelnika w błąd

# Rev1, issue 8, PG

- The authors assume in 3.3.1 that “sequence clustering is deterministic” – please elaborate here. Is this because the method actually is deterministic? Or due to the time constraints you could only perform each experimental setting once? If the latter, it would be nice to see some confidence intervals, standard error, etc. for at least a few of the mid-range sample sizes to understand how much each run might differ.

## answer

w tekście niepoprawnie użyliśmy wskazanego opisu jako cecha algorytmu, a jest to jedynie cecha implementacji. W doświadczeniach jest stały 'seed'. W ogóloności metoda nie jest determistyczna, ponieważ to zewnętrzny algorytm tworzy grupy, chociaż nie spodziewamy się dużych różnic w obrębach grup.

Ponieważ był stały 'seed' każde uruchomienie zwraca to samo, więc każde doświadcznie było uruchamiane raz.

# Rev1, issue 9, PG

- I would like to see performance on an external validation set used with the method. The CAMI II dataset used only comes from the human skin microbiome and was further modified by the authors to create the positive and negative sequences. What specific steps were performed to prevent data leakage here? How does the method perform on a different type of sample, and are there biologically validated data that could be used to assess accuracy? If this isn’t possible, it would be good for readers to know why.

## answer

Podczas badań dbaliśmy o rozdzielenie zbiorów trenujących i testujących. Recenzent ma racje, że mogły zaistnieć zjawiska typu 'data leakage'. Aby dodatkowo zwalidować rozwiązanie wzieliśmy dodatkową próbkę z CAMI II z jelit i powtórzyliśmy pipeline. Wynik (podobieństwo) jest porówywalny z tym co pokazaliśmy w pracy.

# Rev1, issue 10, PG

- The sequences are trimmed to 150 bp – how is the trimming performed? Always from the 5’ or 3’ end? Randomly? What if a key motif is cleaved off – how does this impact performance? More detail is needed here on how this is performed, why 150 bp was selected, and what this means in terms of impacts on downstream classification. Further details on the k-mer size and embedding parameter choices should also be included.

## answer

opis w artykule jest nieco nadmiarowy. Dostarczyliśmy skrypt, który obcina odczyt z 5' do 3' 150bp, a jak odczyt jest zbyt krótki, to go uzupełnia losowymi symbolami. W naszych badaniach zbiór wejściowy miał odczyty o długości dokładnie 150bp każdy, więc skrypt trimmujący nie był użyty.

# Rev1, issue 11, RN

- It’s still unclear to me if it is better to use this method or something else like Kraken or Centrifuge for a given metagenomics classification task – it would be nice to see a direct comparison (accuracy and speed) with the final pipeline against popular methods in the field on a standardized dataset.

## answer

Kraken jest znacznie szybszy, dla XXX. Natomiast Blast jest dokładniejszy. Przedstawione rozwiązanie jest pośrednie, jakość nieco lepsza, a na pewno porównywalna do Kraken, szybokość działania znacznie wyższa niż Blast. Przyspiesza Blasta, nie trzeba używać innego narzędzia. Wybieramy kotwicę (reprezentantów), więc część sekwencji jest już przejrzana przez Blast, co znacznie przyspiesza obliczenia.

Reviewer 2
----------
Manuscript Review: "Accelerating Metagenomic Identification of DNA Sequences Using Artificial Neural Networks"

# Rev2, preliminary

The paper is generally well-structured. This manuscript presents a promising approach for metagenomic identification using ANNs. However, several issues and experiment results require clarification.

## Answer

Thank you for appreciating our work.

thank you for effort and valuable comments, below detailed answer for issues

# Rev2, issue 1, PG

1. The contrastive learning framework needs more detailed explanation, particularly regarding how representative sequences are selected and how sequences are clustered. The description about these two parts is not very clear.

## answer

opisać dokładniej to co wskazano

# Rev2, issue 2, PG

2. Why do the authors use cosine dissimilarity to calculate the dissimilarity between sequence representations?

## answer

wyjaśnić

# Rev2, issue 3, PG

3. The training procedure, including hyperparameter selection, optimization algorithm, and convergence criteria, requires elaboration.

## answer

wyjaśnić i do tego uwzględnić Rev1, issue 2

# Rev2, issue 4, PG

4. The rationale for specific design choices should be justified with supporting evidence or theoretical foundations.

## answer

podobnie jak Rev2, issue 3 i Rav1, issue 2

# Rev2, issue 5a, PG

5. The performance evaluation metrics mainly focus on execution time and “quality”. However, the quality is not very well-defined. To evaluate metagenomic profiling, the core evaluation metrics include:
- precision: The proportion of correctly identified taxa among all predicted taxa.
- recall/sensitivity: The proportion of true taxa that were successfully identified.
- L1 Norm Error: Measure the absolute difference between predicted and true relative abundances.

## answer

indeks Jaccarda, FP, TP, Rev1, issue 7

# Rev2, issue 5b, PG

More importantly, the authors should compare their method with existing well-known methods, such as kraken2, MetaPhlAn, Centrifuge, and QIIME2. And there are several ANN-based approaches, such as VAMB, MetaDecoder, and DeepBin. Please also include these methods for performance comparison.

## answer

podobne do Rev1, issue 5

# Rev2, issue 6, PG

6. Code Availability: The authors mention developing "ENpquizitor," a command-line tool implementing the proposed method. However, no clear information is provided about public access to the code repository. There is no reference to reproducibility materials like configuration files or trained models. I strongly recommend making the code publicly available with comprehensive documentation.

## answer

model na dysku, dokumentacja - Readme

Reviewer 3
----------
Reviewer 3

# Rev3, preliminary

The authors proposed a new method to calculate the similarity between two DNA sequences based on the embedding output of an ANN, incorporating this step into a metagenomic identification problem. They benchmarked it against two baseline methods and showed improvement. However, the manuscript does not meet the standard of BMC Bioinformatics for the following reasons.

# Rev3, issue 1, PG

The proposed method aims to convert sequences into embedding vectors. There are many existing related packages, such as DNABERT. The authors should benchmark against these tools to demonstrate that their CNN model offers an advantage.

## answer

DNABERT jest znacznie większy obliczeniowo, tutaj została zaproponowana metoda, która używa głównie Blast.

# Rev3, issue 2, PG + RN

There is also a problem in model training, specifically in the preparation of training data. The procedure for generating positive and negative sequences is unclear and lacks biological justification: why 0–20% mutation for positives and 20–80% for negatives, which type of DNA is used (CDS or non-coding), and whether synonymous or non-synonymous mutations are considered, since mutation rates differ among these regions. The authors need to redesign the mutation model with consideration of appropriate biological factors.

## answer

model był trenowany na danych rzeczywistych, augmentacja faktycznie nie była inspirowana biologicznie; pomimo braku związku prawdopodobieństw generowanych mutacji z biologią generalizacja powinna mieć sens i ma, jak wykazaliśmy w badaniach.

Uruchomiliśmy jedno przykładowe badanie ze zmienionymi rozkładami, tak jak zasugerował recenzent, wyniki modelu były grupowane, zaś grupy zostały porównane z poprzedniomi naszymi wynikami. Zmiany były XXX.

Rev 1, issue 9

# Rev3, issue 3, PG

It is also unclear what the sizes of the training, validation, and test datasets are (Section 3.1.2).

## answer

przejrzeć i wpisać odp. wartości

# Rev3, issue 4, PG

There is a problem in benchmarking. First, the baseline methods are too simple and do not consider two types of gap penalties (Gotoh algorithm) or the two types of nucleotide substitutions, transition and transversion.

## answer

opisać, że baseline nie rozróżnia różnych kar za przerwę (Gotoh)


# Rev3, issue 5, RN

The authors might consider using MAFFT instead of the NW algorithm, as MAFFT applies the Fourier transform and is faster.

## answer

Baseline był tylko po to, aby porównać jakość. Baseline był wybrany tak, aby był prosty.
Dziękujemy za sugestię, w kolejnej wersji narzędzia chcemy przetestować też MAFFT. Obecnie wspomnieliśmy o nim w dyskusji.

# Rev3, issue 6, PG

In the performance measurement, it is unclear how the reference metagenomic identification was obtained—was it determined by BLASTn?

## answer

opisać, że bez 'ground truth'

# Rev3, issue 7, PG

Most importantly, state-of-the-art methods are not included in the benchmark.

## answer

Rev1 miał podobne issue
Rev2 miał podobne issue

# Rev3, issue 8, PG

Minor:
The proposed training process is similar to the triplet neural network framework (Hoffer & Ailon, 2015, Deep metric learning using triplet network, in Similarity-Based Pattern Recognition, pp. 84–92), which classifies the data with the rule: anchor, a negative, and a positive sequence.

## answer

zostanie dodana referencja do pracy

# Rev3, issue 9, PG

The authors might revise the network structure accordingly. An intuitive explanation for Equation (2) is needed, e.g., why “margin – cosine” is used for positives but “negative – margin” for negatives.

## answer

poprawki w tekście

# Rev3, issue 10, PG
On page 7, the value of the hyperparameter weight decay w is missing.

## answer

dodać ten parametr

# Rev3, issue 11, PG

Although it is good to have the GitHub link, detailed usage instructions for the package are needed.

## answer

chceme mieć readme
Rev2, issue 6

Reviewer 4
----------
Reviewer 4

# Rev4, issue 1, RN, +

1. The abstract could be made more concise and sharply focused on the core contributions namely the use of contrastive learning within an ANN for representative sequence selection, its performance gains over classical methods, and the development of the Exquisitor tool.

## answer

przejrzeć abstrakt, zaproponować zmiany

zmieniony abstrakt, 4 poprawki z grammarly, przeniosłem zdanie o implementacji, uwzględnione wskazówki.

# Rev4, issue 2, PG

2. The manuscript does not sufficiently address the potential limitations of the proposed ANN-based system (e.g., generalizability to diverse datasets, dependence on GPU acceleration, or adaptability to varying sequence lengths). These should be discussed to present a balanced and transparent view.

## answer

opisać problemy i ograniczenia, w 'discussion'

# Rev4, issue 3, PG

3. The authors should provide more details on computational complexity, including time and memory requirements, and clarify the reliance on specific benchmark datasets during evaluation.

## answer

jest w pr. inzynierskiej, dodać akapit do artykułu

# Rev4, issue 4, RN, +

4. The literature review should be expanded to compare the proposed method with recent studies using bioinformatics datasets for classification problems, such as s13040-024-00415-8, s12859-024-05917-0, and s12859-024-05978-1. This will help situate the contribution within the current state-of-the-art.

## answer

popatrzeć na te prace, dodać jak są związane

s12859-024-05917-0.pdf - Salman Khan1, Salman A. AlQahtani2, Sumaiya Noor3 and Nijad Ahmad4*, PSSM‑Sumo: deep learning based intelligent model for prediction of sumoylation sites using discriminative features

znajdowanie białek, gdzie występują post-translational modifications, bazując procesie 'sumoylation', sieci neuronowe, znajduje pseudo-position-specific scoring matrix, który pomaga znaleźć

s13040-024-00415-8.pdf - Salman Khan1, Sumaiya Noor2, Tahir Javed3, Afshan Naseem4, Fahad Aslam4, Salman A. AlQahtani1 and
Nijad Ahmad5, XGBoost-enhanced ensemble model using discriminative hybrid features for the prediction of sumoylation sites

Post-translational modifications (PTMs), sumoylation, problem jak wcześniej, ale tutaj word embeddings, SHAP (tak jak później),
na koniec XGBoost.

s12859-024-05978-1.pdf - Sumaiya Noor1, Afshan Naseem2, Hamid Hussain Awan3, Wasiq Aslam3, Salman Khan4, Salman A. AlQahtani4 and Nijad Ahmad5*, Deep‑m5U: a deep learning‑based approach for RNA 5‑methyluridine modification prediction using optimized feature integration

znajdowanie modyfikacji w RNA (m5U), metoda 'Deep-m5U', K-tuple, pozniej SHAP -- istotność każdego atrybutu definiowana jako ilość wnoszonej informacji, jakość na podzbiorach bez cechy i z cechą znormalizowana ilością podzbiorów, tutaj Boruta SHAP, później sieć neuronowa DNN (gęsta).


# Rev4, issue 5, RN
5. While the manuscript’s language is generally understandable, there are grammatical and stylistic issues that should be corrected to improve readability and ensure technical clarity.

# answer

Rev1, issue 3, uruchomic Grammarly

# Rev4, issue 6, PG
6. The paper would benefit from strengthening its technical depth. Currently, some sections are presented in a more theoretical or descriptive style; these should be rewritten to include more technical detail, experimental reasoning, and methodological rigor.

# answer

byly wczeniej u kazdego Rev1, Rev2, Rev3

# Rev4, issue 7, PG

7. A dedicated “Model Limitations” subsection is recommended to ensure an unbiased evaluation of the work. This should highlight both the strengths and constraints of the proposed approach.

## answer

pojawiało się wcześniej u innych Rev

# Rev4, issue 8, RN, +

8. The “Future Work” section could be enriched by discussing how the method might be extended to large-scale bioinformatics tasks using parallel computing for big data in gene regulatory networks. For instance, parallel deep learning approaches described in s00607-025-01441-y could be considered.

## answer

przjrzeć papier, zobaczyć czycoś wnosi

s00607-025-01441-y.pdf Sumaiya Noor, Hamid Hussain Awan, Amber Sarwar Hashmi, Aamir Saeed, Salman Khan & Salman A. AlQahtani, Optimizing performance of parallel computing platforms for large-scale genome data analysis

nowy algorytm, 'Spark-Pi-DNN', do klasyfikacji dużych RNA, używa distributed computing and cluster computing platforms,
porównany do Apache Hadoop, Apache Spark (czas obliczeń) i różne miary jakości.


Ending
------
How to submit your revision
The original submitting author must upload a point-by-point response to the comments as a PDF file. This must include a description of any additional experiments that were carried out and a detailed rebuttal of any criticisms or requested revisions that you disagreed with.

Any files (including the manuscript) that have changed based on the comments will also need to be uploaded again. Do not include tracked changes in your manuscript file. If you need to upload a marked-up version of the revised manuscript with the changes highlighted you can upload it on the related file section.

Please note the original submitting author may be different from the corresponding author.

Do you need more time to submit your revision?

If you need an extension, please contact us and include your submission ID. You can find this ID in the revision request email or on the right-hand side of this page.
