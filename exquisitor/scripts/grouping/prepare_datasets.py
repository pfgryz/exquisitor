from Bio import SeqIO
from tqdm import tqdm
import csv
import json
import sys
from pathlib import Path
from collections import defaultdict

def load_mapping(mapping_file_path: Path) -> dict:
    mapping = {}

    with open(mapping_file_path, newline="", encoding="utf-8") as handle:
        for row in tqdm(csv.DictReader(handle, delimiter="\t"), desc="Load mapping"):
            mapping[row["#anonymous_read_id"]] = row["tax_id"]

    return mapping

def main(args):
    sample_path =  Path(args[1]) / "reads"
    file_path = sample_path / "anonymous_reads.fq"
    mapping_file_path = sample_path / "reads_mapping.tsv"
    output_file_path = sample_path / "inter_connected.csv"

    tax_id_map = load_mapping(mapping_file_path)
    tax_id_groups = defaultdict(int)

    with open(output_file_path, "w", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=["id", "sequence", "tax_id"])

        writer.writeheader()

        for record in tqdm(SeqIO.parse(file_path, "fastq"), desc="Process sequences"):
            tax_id = tax_id_map.get(record.id)

            tax_id_groups[tax_id] += 1

            writer.writerow({
                "id": str(record.id),
                "sequence": str(record.seq),
                "tax_id": tax_id
            })

    with open(sample_path / "classes.json", "w") as handle:
        json.dump(tax_id_groups, handle)

if __name__ == "__main__":
    main(sys.argv)
