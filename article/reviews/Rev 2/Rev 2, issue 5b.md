---
tags:
  - todo
owner: PFG
---
## Issue
More importantly, the authors should compare their method with existing well-known methods, such as kraken2, MetaPhlAn, Centrifuge, and QIIME2. And there are several ANN-based approaches, such as VAMB, MetaDecoder, and DeepBin. Please also include these methods for performance comparison.
## Answer (draft)  
podobne do Rev1, issue 5
## Answer
Dziękujemy za zwrócenie uwagi,

Our pipeline increases the speed of BLAST by using a single sequence that represents a cluster and it uses the BLAST internally. 

Porównaliśmy z ?, ?, ?, faktycznie warto byłoby porównać z kraken, metaphi, ..., Jest to działanie planowane w kolejnych etapach rozwoju projektu, ponieważ eksperyment wymaga dużo czasu (setup, przeprowadzenie etc.). Nie zrobiliśmy tego w tej wersji.

Homewer, we think ..., że te doświadczenia przeprowadzone pokazują użyteczność naszej metody.
