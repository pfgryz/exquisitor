---
tags:
  - review
  - ongoing
owner: PFG
---
## Issue
The term “classification quality” should be more clearly defined for readers here – especially those less familiar with the CAMI Classification Challenges. Can the authors also present performance – perhaps on an additional dataset if easier – to represent classification and speed performance? In the metagenomics use case, this could be done using standard machine learning metrics such as precision, recall, F1-score, accuracy, etc. If just focusing on clustering accuracy, alignment scores, bit scores, percent identity, etc. may be more appropriate. The use of the Jaccard index here is helpful but a bit harder to interpret, as details on the type of error (number of false positives vs. false negatives) are lost.
## Answer (draft)
lepej wyjaśnić dlaczego taka miara, że nie da się w prosty sposób macierzy pomyłek, bo referencją jest wynik Blast, nie ma 'ground truth', więc miary typu FP, FN mogą wprowadzać czytelnika w błąd
## Answer
Thank you for bringing this to our attention.

We decided to use the Jaccard index instead of standard machine learning metrics, because it is not straightforward to obtain a confusion matrix with respect to a ground truth, as the reference is the BLAST result.

Reporting false positives or false negatives would be misleading, since FPs or FNs do not make sense in this context - what appears as a false positive relative to BLAST could actually be a true positive with respect to the real ground truth. Our goal was primarily to approximate BLAST results.
## Review
- czy coś dodawać do tekstu?
- dodać informacje o tej metryce (gdzieś)