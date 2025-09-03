---
tags:
  - done
owner: PFG
---
## Issue
The authors assume in 3.3.1 that “sequence clustering is deterministic” – please elaborate here. Is this because the method actually is deterministic? Or due to the time constraints you could only perform each experimental setting once? If the latter, it would be nice to see some confidence intervals, standard error, etc. for at least a few of the mid-range sample sizes to understand how much each run might differ.
## Answer
Thank you for bringing this to our attention.

In the text, we incorrectly referred to the described feature as a property of the method, whereas it is actually an implementation detail. In the experiments, a fixed seed was used. In general, the method is not deterministic, since clustering is performed by an external algorithm; homever, we do not expect significant differences within groups. Each experimental setting was run only once due to time constriants. In the case of execution time, the most time is spent on BLAST query and clustering algorithm. 

We have modified the article, Section 3.3.1 (p. 10) and Section 3.3.2 (p. 11) by adding the following text:
```
Sequences clustering is deterministic with a fixed seed.
```