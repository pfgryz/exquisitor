# 4.2. CD-HIT

## Table of Contents

- [4.2.1. Source project](#421-source-project)
- [4.2.2. Description](#422-description)

## 4.2.1 Source project

https://github.com/weizhongli/cdhit

## 4.2.2. Description

Clustering program

### Advantages

1. Nice output files that contains information about representative of the
   cluster (marked with `*`) and all sequences that belong to that cluster with
   similarity expressed in percentages
2. Many options, can be fine-tuned for better results
3. Nice runtime information with helpful statistics (memory usage, cpu time,
   used cores)
4. Multithreading enabled with flag, utilizes all physical cores. Without flag
   is single threaded

### Drawbacks

1. Output file is not fully documented, can be misleading
2. Information about errors is not user-friendly

---

[Back](index.md) |
[Previous](dnaclust.md) |
[Next](mothur.md)