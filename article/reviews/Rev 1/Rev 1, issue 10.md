---
tags:
  - ongoing
owner: PFG
---
## Issue
The sequences are trimmed to 150 bp – how is the trimming performed? Always from the 5’ or 3’ end? Randomly? What if a key motif is cleaved off – how does this impact performance? More detail is needed here on how this is performed, why 150 bp was selected, and what this means in terms of impacts on downstream classification. Further details on the k-mer size and embedding parameter choices should also be included.
## Answer (draft)
opis w artykule jest nieco nadmiarowy. Dostarczyliśmy skrypt, który obcina odczyt z 5' do 3' 150bp, a jak odczyt jest zbyt krótki, to go uzupełnia losowymi symbolami. W naszych badaniach zbiór wejściowy miał odczyty o długości dokładnie 150bp każdy, więc skrypt trimmujący nie był użyty.
## Answer

Thank you for bringing this to our attention.

The description in the article is a bit redundant. In our study, we used an input dataset with reads of exactly 150 bp each, so it was not necessary to trim the sequences to this length. The 150 bp length was determined by the input dataset, but the method is applicable to sequences of any length. In the case of longer sequences trimming should be performed from 5' to 3', if read is shorter, it is filled by random symbols. The method is designed for sequences of similar length. 

The k-mer embedding is created by constructing a vector of size $4^k$, where each k-mer corresponds to a specific position in the vector. The number of occurrences of each k-mer in the sequence is then recorded at its corresponding position.

We have modified the article, section 3.2.2, page 10, by adding the following text:
```
Embedding is created by constructing a vector of size $4^k$, where each $k$-mer corresponds to a specific position in the vector. The number of occurences of each $k$-mer in the sequence is then recorded at its corresponding position. 
```
## Review
- czy dodać w tekście, że rozmiar 150bp może być zmieniony na dowolny inny? dodać, że to parametr (przy pierwszym użyciu wyjaśnić) może być footnote