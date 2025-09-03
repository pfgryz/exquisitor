---
tags:
  - ongoing
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
## Review
- zaktualizować main.tex o przeprowadzony eksperyment (sekcja 3.5)
- wkleić ten sam tekst tutaj w odpowiedź