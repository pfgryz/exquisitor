---
tags:
  - done
owner: PFG
---
## Issue
The training procedure, including hyperparameter selection, optimization algorithm, and convergence criteria, requires elaboration.
[[Rev 1, issue 2]]
## Answer
Thank you for bringing this to our attention.

The model was trained for 100 epochs using data split into training and validation sets, while a separate test set was employed in subsequent experiments. Hyperparameters were selected through grid search, resulting in the following configuration: a learning rate of $10^{-6}$, weight decay of $10^{-4}$, $\gamma=0.99999$, a batch size of 256, and a dropout rate of 0.4. The optimization algorithm used was AdamW, which incorporates decoupled weight decay regularization. This choice provides more stable convergence and improved generalization compared to the standard Adam optimizer. Convergence was determined based on the absence of significant improvements in the validation loss.

We have modified the article, Section 2.1.1 (p. 7), by adding the following text
```
Learning
[...]
and the training was performed for 100 epochs. To mitigate the risk of overfitting, both weight decay and the exponential learning rate schedule were employed. The convergence criterion was defined as the absence of further improvement in the validation loss.
[...]

Parameters
[...]
The parameter search was performed using grid search.
[...]
a batch size of 256
```