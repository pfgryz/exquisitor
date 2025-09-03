from math import floor, ceil
from tqdm import tqdm
import json
import csv
import sys
from pathlib import Path
from collections import defaultdict
import random

def balance_counts(counts: dict, max_total: int = 10_000) -> dict:
    dominant_class = max(counts, key=counts.get)
    other_classes = {k: v for k, v in counts.items() if k != dominant_class}

    counts[dominant_class] = floor(sum(other_classes.values()) / 2)
    total = sum(other_classes.values()) + counts[dominant_class]

    if sum(counts.values()) > max_total:
        factor = max_total / total
        counts = {k: max(1, ceil(v * factor)) for k, v in counts.items()}

    return counts

def main(args):
    sample_path = Path(args[1]) / "reads"

    with open(sample_path / "classes.json", "r") as handle:
        counts = json.load(handle)

    balanced = balance_counts(counts)
    chance = {
        k: balanced[k] / counts[k] + (1 / 10_000)
        for k in counts.keys()
    }

    taken = defaultdict(int)

    with open(sample_path / "inter_connected.csv", "r", newline="", encoding="utf-8") as infile, \
        open(sample_path / "for_grouping.csv", "w", newline="", encoding="utf-8") as outfile:

        reader = csv.DictReader(infile)
        writer = csv.DictWriter(outfile, fieldnames=reader.fieldnames)
        writer.writeheader()

        for idx, row in tqdm(enumerate(reader)):
            tax_id = row["tax_id"]

            if taken[tax_id] < balanced.get(tax_id, 0):
                if random.random() < chance[tax_id]:
                    writer.writerow(row)
                    taken[tax_id] += 1

            if idx % 1000 and all(taken[c] >= balanced[c] for c in balanced.keys()):
                break

if __name__ == "__main__":
    main(sys.argv)
