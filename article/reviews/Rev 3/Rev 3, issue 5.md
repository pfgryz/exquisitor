---
tags:
  - done
  - description
owner: RN
---
## Issue
The authors might consider using MAFFT instead of the NW algorithm, as MAFFT applies the Fourier transform and is faster.

## Answer
Thank you for this hit. We plan to benchmark our tool with MAFFT. Our baseline algorithm (NW) was used mainly for quality checking. We modified the article in section 'Future Research Directions' adding the sentence:

```
We plan to benchmark our method with MAFFT -- the algorithm using Fast Fourier Tranfsorm to alignment~\cite{katoh2013mafft},
instead of NW algorithm.
```

## Notes
Baseline był tylko po to, aby porównać jakość. Baseline był wybrany tak, aby był prosty.

Dziękujemy za sugestię, w kolejnej wersji narzędzia chcemy przetestować też MAFFT. Obecnie wspomnieliśmy o nim w dyskusji.


@article{katoh2013mafft,
  title={MAFFT multiple sequence alignment software version 7: improvements in performance and usability},
  author={Katoh, Kazutaka and Standley, Daron M},
  journal={Molecular biology and evolution},
  volume={30},
  number={4},
  pages={772--780},
  year={2013},
  publisher={Society for Molecular Biology and Evolution}
}
