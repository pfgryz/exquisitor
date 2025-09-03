---
tags:
  - done
owner: PFG
---
## Issue
There is also a problem in model training, specifically in the preparation of training data. The procedure for generating positive and negative sequences is unclear and lacks biological justification: why 0–20% mutation for positives and 20–80% for negatives, which type of DNA is used (CDS or non-coding), and whether synonymous or non-synonymous mutations are considered, since mutation rates differ among these regions. The authors need to redesign the mutation model with consideration of appropriate biological factors.
## Answer (draft) 
model był trenowany na danych rzeczywistych, augmentacja faktycznie nie była inspirowana biologicznie; pomimo braku związku prawdopodobieństw generowanych mutacji z biologią generalizacja powinna mieć sens i ma, jak wykazaliśmy w badaniach.

Uruchomiliśmy jedno przykładowe badanie ze zmienionymi rozkładami, tak jak zasugerował recenzent, wyniki modelu były grupowane, zaś grupy zostały porównane z poprzedniomi naszymi wynikami. Zmiany były XXX.
## Answer
Thank you for bringing this to our attention.

The model was trained on high-quality simulated data, and the data augmentation applied was not directly biologically motivated. Although the probabilities of the generated mutations do not strictly reflect biological reality, the model was still able to perform well in our experiments. Nevertheless, the lack of incorporation of biological information may have influenced the results and potentially limited the model's overall effectiveness.