import csv
import sys
from sklearn.metrics import (
    adjusted_rand_score,
    normalized_mutual_info_score,
    fowlkes_mallows_score,
    homogeneity_completeness_v_measure
)
from sklearn.metrics.cluster import contingency_matrix
import numpy as np

from pathlib import Path


def load_ground_truth(filepath: Path) -> dict:
    data = {}

    with open(filepath, "r", newline="", encoding="utf-8") as handle:
        reader = csv.DictReader(handle)

        for row in reader:
            data[row["sequence"]] = row["tax_id"]

    return data

def load_predictions(filepath: Path) -> dict:
    data = {}

    with open(filepath, "r", newline="", encoding="utf-8") as handle:
        reader = csv.DictReader(handle)

        for row in reader:
            data[row["sequence"]] = row["cluster_id"]

    return data

def compare_clusterings(dict1, dict2):
    common_keys = sorted(set(dict1.keys()) & set(dict2.keys()))

    if not common_keys:
        raise ValueError("No common sequences")

    labels1 = [dict1[key] for key in common_keys]
    labels2 = [dict2[key] for key in common_keys]

    metrics = {
        'Adjusted Rand Index': adjusted_rand_score(labels1, labels2),
        'Normalized Mutual Information': normalized_mutual_info_score(labels1, labels2),
        'Fowlkes-Mallows Index': fowlkes_mallows_score(labels1, labels2),
        'Homogeneity': homogeneity_completeness_v_measure(labels1, labels2)[0],
        'Completeness': homogeneity_completeness_v_measure(labels1, labels2)[1],
        'V-Measure': homogeneity_completeness_v_measure(labels1, labels2)[2],
        'Contingency Matrix': contingency_matrix(labels1, labels2)
    }

    return metrics

def print_metrics(metrics):
    print("Metrics:")
    print("=" * 50)

    for name, value in metrics.items():
        if name != 'Contingency Matrix':
            print(f"{name:30s}: {value:.6f}")

    print("\nContigency Matrix:")
    print(metrics['Contingency Matrix'])

def main(args):
    sample_path = Path(args[1])

    ref = load_ground_truth(sample_path / "for_grouping.csv")
    pred = load_predictions(sample_path / "groups.csv")
    metrics = compare_clusterings(ref, pred)
    print_metrics(metrics)

if __name__ == "__main__":
    main(sys.argv)
