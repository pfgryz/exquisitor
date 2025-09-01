---
tags:
  - ongoing
  - citation
  - description
owner: RN
---
## Issue
The literature review should be expanded to compare the proposed method with recent studies using bioinformatics datasets for classification problems, such as s13040-024-00415-8, s12859-024-05917-0, and s12859-024-05978-1. This will help situate the contribution within the current state-of-the-art.
## Answer (draft)
popatrzeć na te prace, dodać jak są związane

s12859-024-05917-0.pdf - Salman Khan1, Salman A. AlQahtani2, Sumaiya Noor3 and Nijad Ahmad4*, PSSM‑Sumo: deep learning based intelligent model for prediction of sumoylation sites using discriminative features

znajdowanie białek, gdzie występują post-translational modifications, bazując procesie 'sumoylation', sieci neuronowe, znajduje pseudo-position-specific scoring matrix, który pomaga znaleźć

s13040-024-00415-8.pdf - Salman Khan1, Sumaiya Noor2, Tahir Javed3, Afshan Naseem4, Fahad Aslam4, Salman A. AlQahtani1 and

Nijad Ahmad5, XGBoost-enhanced ensemble model using discriminative hybrid features for the prediction of sumoylation sites

Post-translational modifications (PTMs), sumoylation, problem jak wcześniej, ale tutaj word embeddings, SHAP (tak jak później),

na koniec XGBoost.

s12859-024-05978-1.pdf - Sumaiya Noor1, Afshan Naseem2, Hamid Hussain Awan3, Wasiq Aslam3, Salman Khan4, Salman A. AlQahtani4 and Nijad Ahmad5*, Deep‑m5U: a deep learning‑based approach for RNA 5‑methyluridine modification prediction using optimized feature integration

znajdowanie modyfikacji w RNA (m5U), metoda 'Deep-m5U', K-tuple, pozniej SHAP -- istotność każdego atrybutu definiowana jako ilość wnoszonej informacji, jakość na podzbiorach bez cechy i z cechą znormalizowana ilością podzbiorów, tutaj Boruta SHAP, później sieć neuronowa DNN (gęsta).

Background - dodać to gdzieś, obszary.