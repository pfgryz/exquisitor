---
tags:
  - todo
  - description
  - justification
owner: RN
---
## Issue
It’s still unclear to me if it is better to use this method or something else like Kraken or Centrifuge for a given metagenomics classification task – it would be nice to see a direct comparison (accuracy and speed) with the final pipeline against popular methods in the field on a standardized dataset.
## Answer (draft)
Kraken jest znacznie szybszy, dla XXX. Natomiast Blast jest dokładniejszy. Przedstawione rozwiązanie jest pośrednie, jakość nieco lepsza, a na pewno porównywalna do Kraken, szybokość działania znacznie wyższa niż Blast. Przyspiesza Blasta, nie trzeba używać innego narzędzia. Wybieramy kotwicę (reprezentantów), więc część sekwencji jest już przejrzana przez Blast, co znacznie przyspiesza obliczenia.