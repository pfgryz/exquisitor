import logging
import os
import subprocess
import sys
from pathlib import Path
from typing import List, Optional
from typing_extensions import Annotated
import numpy as np
import typer
from Bio import SeqIO, Align
from Bio.Cluster import kcluster
from Bio.Seq import Seq
from Bio.SeqRecord import SeqRecord
from dotenv import dotenv_values

Sequences = List[SeqRecord]
Clusters = List[List[SeqRecord]]


# region Preprocessing

def preprocess(sequences: Sequences) -> Sequences:
    """
    Naive simulation of preprocessing sequences
    Currently it trims all sequences to common size
    """
    logging.info("BEGIN Preprocessing sequences")

    min_length = min(len(sequence.seq) for sequence in sequences)

    for sequence in sequences:
        sequence.seq = sequence.seq.upper()

        if len(sequence.seq) > min_length:
            sequence.seq = Seq(str(sequence.seq[:min_length]))

    logging.info("END Preprocessing sequences")

    return sequences


# endregion

# region Clustering

def distance(a: SeqRecord, b: SeqRecord) -> float:
    aligner = Align.PairwiseAligner()
    return aligner.score(a.seq, b.seq)


def create_distance_matrix(sequences: Sequences) -> np.ndarray:
    distance_matrix = np.zeros((len(sequences), len(sequences)))

    for i, seq_i in enumerate(sequences):
        for j, seq_j in enumerate(sequences):
            distance_matrix[i, j] = distance(seq_i, seq_j)

    return distance_matrix


def cluster(sequences: Sequences, nclusters: int = 2) -> Clusters:
    """
    Implements naive clustering using biopython
    """
    logging.info("BEGIN Clustering")

    clusters = [list() for _ in range(nclusters)]

    distance_matrix = create_distance_matrix(sequences)
    masks, score, errors = kcluster(distance_matrix, nclusters=nclusters)

    for sequence, mask in zip(sequences, masks):
        clusters[mask].append(sequence)

    logging.info("END Clustering")

    return clusters


# endregion

# region Searching

def run_blastn(filename: str) -> str:
    result = subprocess.run([
        "blastn",
        "-query",
        filename,
        "-db",
        "nt",
        "-outfmt",
        "6 std staxids sscinames"
    ], capture_output=True, text=True, env=os.environ.copy())

    if result.returncode != 0:
        raise RuntimeError("BLASTn failed")

    return result.stdout


def search(clusters: Clusters):
    logging.info("BEGIN Searching")

    reprs = []
    repr_to_cluster = {}
    for cluster in clusters:
        repr = cluster[0]
        reprs.append(repr)
        repr_to_cluster[repr.name] = (cluster, [])

    with open("temp.fasta", "w") as handle:
        SeqIO.write(reprs, handle, "fasta")

    results = run_blastn("temp.fasta")

    for line in results.split("\n"):
        if not line.strip():
            continue

        values = line.split("\t")
        source = values[0]
        target = values[-1]

        if source in repr_to_cluster:
            repr_to_cluster[source][1].append(target)

    logging.info("END Searching")

    return repr_to_cluster.values()


# endregion

# region Postprocessing
def postprocessing(clusters):
    logging.info("BEGIN Postprocessing")

    detected = {}
    count = 0
    not_matched = 0

    for cluster in clusters:
        size = len(cluster[0])
        matched = False

        for result in cluster[1]:
            if result not in detected:
                detected[result] = 0
            detected[result] += size
            matched = True

        if not matched:
            not_matched += size

        count += size

    logging.info("END Postprocessing")

    return (count, not_matched, detected)


# endregion

# region Reporting

def report(results):
    logging.info("BEGIN Generating reports")

    count, not_matched, detected = results

    response = "\n".join([
        "--------------------",
        "--- QUERY RESULT ---",
        f"Count: {count}",
        f"Not matched: {not_matched}",
        f"",
        f"Detected:",
        "\n".join([
            f"\t- {name}: {cnt}" for name, cnt in detected.items()
        ])
    ])

    logging.info("END Generating reports")

    return response


def display(rep: str):
    print(rep)


# endregion

# region Main

app = typer.Typer(no_args_is_help=True)


def read_fasta(filename: str) -> Sequences:
    logging.info(f"Reading FASTA file {filename}")
    return list(SeqIO.parse(filename, "fasta"))


@app.command(
    name="run",
    help="Run full pipeline"
)
def main(path: Annotated[
    Optional[Path],
    typer.Option(
        "--in",
        "-i",
        exists=True,
        readable=True
    )
] = "sequences/present.fasta"):
    root = logging.getLogger()
    root.setLevel(logging.DEBUG)

    handler = logging.StreamHandler(sys.stdout)
    handler.setLevel(logging.DEBUG)
    formatter = logging.Formatter(
        '%(asctime)s - %(name)s - %(levelname)s - %(message)s')
    handler.setFormatter(formatter)
    root.addHandler(handler)

    sequences = read_fasta(path)
    sequences = preprocess(sequences)
    clusters = cluster(sequences)
    searches = search(clusters)
    postprocessed = postprocessing(searches)
    human_info = report(postprocessed)
    display(human_info)


# endregion

if __name__ == '__main__':
    # Due to some weird bugs, Python have outdated Path
    config = dotenv_values(".env")
    os.environ["PATH"] += ":" + config["BLASTN"]
    os.environ["BLASTDB"] = config["BLASTDB"]
    app()
