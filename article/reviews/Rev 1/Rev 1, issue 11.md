---
tags:
  - done
owner: RN
---
## Issue
It’s still unclear to me if it is better to use this method or something else like Kraken or Centrifuge for a given metagenomics classification task – it would be nice to see a direct comparison (accuracy and speed) with the final pipeline against popular methods in the field on a standardized dataset.
## Answer (draft)

## Answer

Kraken is fast, BLAST is more accurate but slow. Our pipeline increases the speed of BLAST by using a single sequence that represents a cluster. It uses the BLAST internally. The representative sequence allows running BLAST externally, and the searching results and their similarity metrics can be stored.

Thank you for pointing this problem with the text. We added a text to the 'Discussion' as depicted in our answer for issue 1.
```.
