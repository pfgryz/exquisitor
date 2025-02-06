import csv
from dataclasses import dataclass
from enum import Enum
import json
import os
import subprocess
from typing import Dict, List, Optional
import matplotlib.pyplot as plt

# region Translation

class Language(Enum):
    PL = "PL"
    EN = "EN"

LANG = Language.EN

def translate(language: Language, polish, english):
    return polish if language == Language.PL else english

# endregion

class ESearchType(Enum):
    Base = 0
    Advanced = 1

# region Parameters

    # Number of experiments
N = 13 

    # Methods
METHODS = {
    "bs": {
        "name": "BZ",
        "search_type": ESearchType.Base,
    },
    "nw": {
        "name": "NW",
        "search_type": ESearchType.Advanced,
    },
    "kmer": {
        "name": "$k$-mer",
        "search_type": ESearchType.Advanced,
    },
    "neural": {
        "name": "SSN",
        "search_type": ESearchType.Advanced,
    }
}

    # Max experiment duration
MAX_DURATION = 12 * 60 * 60 # 12h

    # Size of figures
FIGSIZE = (8, 6)

# endregion

# region Constants

DURATION_EXCEEDED = "ABORTED_MAX_DURATION"
KB = 1024

# endregion

# region Helpers

def parse_int(obj: object, default_value: int = 0.0) -> int:
    """
    Try parse int

    Parameters
    ----------
    obj : object
        Object to parse to int
    default_value : int = 0.0
        Default value returned if cannot parse to int

    Returns
    -------
    int
    """
    try:
        return int(obj)
    except:
        return default_value

def parse_float(obj: object, default_value: float = 0.0) -> float:
    """
    Try parse float 

    Parameters
    ----------
    obj : object
        Object to parse to float
    default_value : float = 0.0
        Default value returned if cannot parse to float

    Returns
    -------
    float
    """
    try:
        return float(obj)
    except:
        return default_value

# endregion

# region .experiment file

@dataclass
class ExperimentResult:
    success: bool
    duration: int
    avg_cpu_usage: float
    max_cpu_usage: float
    max_memory_usage: float


def read_experiment_file(file_path: str) -> ExperimentResult:
    """
    Reads .experiment file
    
    Parameters
    ----------
    file_path : str
        Path to file with experiment monitor results

    Returns
    -------
    ExperimentResult
    """

    cpu_usage = 0 

    avg_cpu_usage = 0
    max_cpu_usage = 0
    last_memory_usage = 0
    max_memory_usage = 0
    
    with open(file_path, "r") as handle:
        reader = csv.DictReader(handle)

        for row_count, row in enumerate(reader):
            cpu = parse_float(row.get("cpu", 0), 0.0)
            cpu_usage += cpu 
            max_cpu_usage = max(max_cpu_usage, cpu)

            memory = parse_float(row.get("memory", 0), 0.0)
            last_memory_usage = max_memory_usage
            max_memory_usage = max(max_memory_usage, memory)

    if row_count != 0:
        avg_cpu_usage = cpu_usage / row_count
        duration = row.get("timestamp", 0)

    if duration == DURATION_EXCEEDED:
        success = False
        duration = None
    else:
        success = True
        duration = parse_int(duration)
    
    return ExperimentResult(
        success, duration, avg_cpu_usage, max_cpu_usage, last_memory_usage
    )

# endregion

# region .search file

@dataclass
class SearchResult:
    organisms: List[str]


def read_search_file(file_path: str, search_type: ESearchType) -> SearchResult:
    """
    Reads .search file

    Parameters
    ----------
    file_path : str
        Path to file with search results

    Returns
    -------
    SearchResult
    """
    organisms = []

    with open(file_path, "r") as handle:
        
        if search_type == ESearchType.Base:
            for line in handle.readlines():
                elements = line.split("\t")

                if len(elements) == 3:
                    organisms.append(elements[1])

        elif search_type == ESearchType.Advanced:
            objects = json.load(handle)

            organisms = [
                obj["name"]
                for obj in objects
            ]
        else:
            raise ValueError("Not recognized search type")
    
    return SearchResult(organisms)

# endregion

# region .clusters file

@dataclass
class Cluster:
    representative_id: int
    elements_id: List[int]

    def to_dict(self):
        return {
            "representative_id": self.representative_id,
            "elements_ids": self.elements_id
        }

@dataclass
class ClustersResult:
    clusters: List[Cluster]


def read_clusters_file(file_path: str) -> ClustersResult:
    """
    Reads .clusters file

    Parameters
    ----------
    file_path : str
        Path to file with clusters

    Returns
    -------
    ClustersResult
    """
    with open(file_path, "r") as handle:
        clusters = [
            Cluster(obj["representative_id"], obj["elements_ids"])
            for obj in json.load(handle)
        ]

    return ClustersResult(clusters)

# endregion

# region Read experiments

@dataclass
class Experiment:
    result: ExperimentResult
    search: Optional[SearchResult]
    clusters: Optional[ClustersResult]


Experiments = Dict[str, List[Experiment]]

def read_experiment(file_path: str, search_type: ESearchType) -> Experiment:
    """
    Reads whole experiment
    
    Parameters
    ----------
    file_path : str
        File path to experiment

    Returns
    -------
    Experiment
    """

    experiment_path = file_path + ".experiment"
    search_path = file_path + ".search"
    clusters_path = file_path + ".clusters"

    if not os.path.isfile(experiment_path):
        raise RuntimeError("Cannot find .experiment file with given experiment path")

    experiment = read_experiment_file(experiment_path)
    search = None
    clusters = None

    if os.path.isfile(search_path):
        search = read_search_file(search_path, search_type)

    if os.path.isfile(clusters_path):
        clusters = read_clusters_file(clusters_path)

    return Experiment(experiment, search, clusters)

def read_experiments(directory: str, methods: dict, count: int) -> Experiments:
    """
    Reads all experiments
    
    Parameters
    ----------
    directory : str
        Path to directory with experiments results
    methods : str
        Methods informatin
    count : str
        Count of experiments per method
    
    Returns
    -------
    Dict[str, List[Experiment]]
    """
    
    output = {}

    for method in methods.keys():
        results = [
            read_experiment(
                f"{directory}/{method}/result{n}",
                methods[method]["search_type"]
            )
            for n in range(count)
        ]

        output[method] = results
    
    return output

# endregion

# region Plot helpers

def generate_xs(count):
    x = list(range(count))
    x_labels = [f"$2^{{{i}}}$" for i in range(count)]

    return x, x_labels

def generate_jitter(base, change):
    count = 0
    while True:
        yield base + change * count
        count += 1

def markers():
    for marker in ["o", "v", "s", "*", "1"]:
        yield marker

def save_plot(plot, file_path: str, dpi: int = 300):
    plot.savefig(file_path, dpi=dpi)
    plot.clf()

# endregion

# region Plot execution duration

def plot_execution_duration(plot, methods, count, experiments: Experiments):
    plot.figure(figsize=FIGSIZE)

    x, x_labels = generate_xs(count)
    jitter = generate_jitter(-0.03, 0.02)
    m = markers()

    for (method_name, executions) in experiments.items():
        label = methods.get(method_name, {}).get("name", "")

        jit = next(jitter)
        
        line = [
            e.result.duration
            for e in executions
        ]

        plot.plot([xi + jit for xi in x], line, "--" + next(m), label=label, alpha = 0.7)

    # plot.title("Czas wykonania klasyfikacji taksonomicznej")
    plot.legend()
    plot.xlabel(translate(LANG, "Liczba sekwencji", "Number of sequences"))
    plot.xticks(x, x_labels)
    plot.ylabel(translate(LANG, "Czas wykonania [s]", "Execution time [s]"))
    # plot.show()

    return plot

def generate_resource_usage_latex_table(count, experiments):

    def cpu(e):
        return int(round(e.result.avg_cpu_usage, 0))
    
    def memory(e):
        return int(round(e.result.max_memory_usage / KB, 0))
    
    def duration(e):
        if e.result.duration is None:
            return -1
        
        return int(round(e.result.duration, 0))
    
    def bold(x, format, func=max):
        max_value = func(x)

        return [
            format(value) 
            if max_value != value
            else 
            f"\\textbf{{{format(value)}}}"
            for value in x
        ]

    resources = ""
    durations = ""

    for (i, (b, nw, kmer, neural)) in enumerate(zip(
            experiments["bs"],
            experiments["nw"],
            experiments["kmer"],
            experiments["neural"]
        )):

        # region Resources
        cpus = [
            cpu(b),
            cpu(nw),
            cpu(kmer),
            cpu(neural),
        ]
        cpus = bold(cpus, lambda x: f"{x}")

        memorys = [
            memory(b),
            memory(nw),
            memory(kmer),
            memory(neural)
        ]
        memorys = bold(memorys, lambda x: f"{x}" if x != -1 else "-")

        resources_line = [
            2 ** i,
            *cpus,
            *memorys
        ]

        resources += " & ".join(map(str, resources_line)) + "\\\\ \\hline \n"

        # endregion

        # region Duration

        durs = [
            duration(b),
            duration(nw),
            duration(kmer),
            duration(neural)
        ]
        durs = bold(durs, lambda x: f"{x}", min)

        duration_line = [
            2 ** i,
            *durs
        ]

        durations += " & ".join(map(str, duration_line)) + "\\\\ \\hline \n"

        # endregion

    return resources, durations

# endregion

# region Plot quality

def quality(base: SearchResult, other: SearchResult) -> float:
    if other == None:
        return 0.0
    
    unique_base = set(base.organisms)
    unique_other = set(other.organisms)

    count_base = {}

    for o in base.organisms:
        count_base[o] = count_base.get(o, 0) + 1

    matches = sum(
        count_base[unique]
        for unique in unique_base.intersection(unique_other)
    )

    return matches / len(base.organisms)

def NMI(first: Experiment, second: Experiment) -> float:
    if first.clusters is None or second.clusters is None:
        return 0.0, 0.0
    
    if not os.path.isfile("exquisitor-cli.exe"):
        raise RuntimeError("Missing exquisitor-core.exe - cannot calcculate NMI")

    def save_clusters(filepath, clusters):
        with open(filepath, "w") as handle:
            handle.write(
                json.dumps(
                    [cluster.to_dict() for cluster in clusters]
                )
            )

    save_clusters("a.tmp", first.clusters.clusters)
    save_clusters("b.tmp", second.clusters.clusters)

    command = ["exquisitor-cli.exe", "compare-clusters", "--reference", "a.tmp", "--second", "b.tmp", "--output", "c.tmp"]
    subprocess.run(command, check=True)

    with open("c.tmp", "r") as file:
        data = json.load(file)

    fmi, nmi = data.get("fmi", 0), data.get("nmi", 0)

    if nmi is None:
        nmi = 1

    return round(nmi, 3), round(fmi, 3)

def plot_quality(plot, methods, count, experiments: Experiments, reference: str):
    plot.figure(figsize=FIGSIZE)

    x, x_labels = generate_xs(count)
    references = experiments.get(reference)
    jitter = generate_jitter(-0.02, 0.02)
    m = markers()

    qualities = []

    for (method_name, executions) in experiments.items():
        if method_name == reference:
            continue

        label = methods.get(method_name, {}).get("name", "")
        jit = next(jitter)
        
        line = [
            quality(ref.search, e.search)
            for e, ref in zip(executions, references)
        ]
        plot.plot([xi + jit for xi in x], line, "--" + next(m), label=label, alpha = 0.7)

        weighted_average_quality = 0
        normalization = 0
        for i, qual in enumerate(line):
            if qual == 0: 
                continue

            normalization += (2 ** i)
            weighted_average_quality += (2 ** i) * qual
    
        weighted_average_quality /= normalization

        print(label, "WAQ:", weighted_average_quality, round(weighted_average_quality, 3))
        qualities.append(line)

    # plot.title("Jakość klasyfikacji taksonomicznej względem wykonania bez potoku przetwarzania")
    plot.legend()
    plot.xlabel(translate(LANG, "Liczba sekwencji", "Number of sequences"))
    plot.xticks(x, x_labels)
    plot.ylabel(translate(LANG, "Jakość względna", "Relative quality"))
    # plot.show()

    return plot, qualities

def generate_quality_latex_table(qualities):
    result = ""

    def quality(q):
        return round(q, 2)

    def bold(x, format, func=max):
        max_value = func(x)

        return [
            format(value) 
            if max_value != value
            else 
            f"\\textbf{{{format(value)}}}"
            for value in x
        ]

    for i, (nw, kmer, neural) in enumerate(zip(*qualities)):
        result += " & ".join(
            map(
                str,
                [
                    f"{2 ** i}",
                    *bold([
                        quality(nw),
                        quality(kmer),
                        quality(neural)
                    ], lambda x: f"{x}", max)
                ]
            )
        ) + "\\\\ \\hline \n"
    
    return result

def plot_relative_quality_latex_table(plot, count, experiments, nmi_flag: bool = True):
    plot.figure(figsize=FIGSIZE)
    x, x_labels = generate_xs(count)
    jitter = generate_jitter(-0.02, 0.02)
    m = markers()

    nmi1_s, nmi2_s, nmi3_s = [], [], []

    for i, (nw, kmer, neural) in enumerate(zip(
        experiments["nw"],
        experiments["kmer"],
        experiments["neural"]
    )):
        nmi1 = NMI(nw, kmer)
        nmi2 = NMI(nw, neural)
        nmi3 = NMI(kmer, neural)

        idx = 0 if nmi_flag else 1

        nmi1_s.append(nmi1[idx])
        nmi2_s.append(nmi2[idx])
        nmi3_s.append(nmi3[idx]) 

    for nmi, label in zip(
        [nmi1_s, nmi2_s, nmi3_s],
        [
            "NW - $k$-mer",
            "NW - SSN",
            "$k$-mer - SSN"
        ]):
        jit = next(jitter)
        plot.plot([xi + jit for xi in x], nmi, "--" + next(m), label=label, alpha=0.7)

    # plot.title("Jakość względna grup wykorzystywanych w klasyfikacji taksonomicznej.")
    plot.legend()
    plot.xlabel(translate(LANG, "Liczba sekwencji", "Number of sequences"))
    plot.xticks(x, x_labels)
    print(nmi_flag)
    plot.ylabel("NMI" if nmi_flag else translate(LANG, "Czułość", "Sensitivity"))

    return plot

def generate_relative_quality_latex_table(count, experiments, nmi: bool = True):
    result = ""

    nmi1_s, nmi2_s, nmi3_s = 0, 0, 0

    for i, (nw, kmer, neural) in enumerate(zip(
        experiments["nw"],
        experiments["kmer"],
        experiments["neural"]
    )):
        nmi1 = NMI(nw, kmer)
        nmi2 = NMI(nw, neural)
        nmi3 = NMI(kmer, neural)

        nmi1_s += nmi1[0]
        nmi2_s += nmi2[0]
        nmi3_s += nmi3[0]

        idx = 0 if nmi else 1
        result += " & ".join(map(str, [2 ** i, nmi1[idx], nmi2[idx], nmi3[idx]])) + "\\\\ \\hline \n"
    
    print("NMI:", nmi1_s / count, nmi2_s / count, nmi3_s / count)

    return result

# endregion

# region Main

def main():
    print(LANG.value)
    os.makedirs(LANG.value, exist_ok=True)

    experiments = read_experiments("C:\\Users\\Komputer\\Desktop\\results_1\\results", METHODS, N)
    
    save_plot(
        plot_execution_duration(plt, METHODS, N, experiments),
        f"{LANG.value}/experiment_duration.png",
        dpi = 500
    )
    plot, qualities = plot_quality(plt, METHODS, N, experiments, "bs")
    save_plot(
        plot,
        f"{LANG.value}/experiment_quality.png",
        dpi = 500
    )
    resources, duration = generate_resource_usage_latex_table(N, experiments)
    
    with open(f"{LANG.value}/resources_usage.tex", "w") as handle:
        handle.write(resources)
    with open(f"{LANG.value}/duration.tex", "w") as handle:
        handle.write(duration)
    
    quality = generate_quality_latex_table(qualities)
    relative_quality_nmi = generate_relative_quality_latex_table(N, experiments)
    relative_quality_sensitivity = generate_relative_quality_latex_table(N, experiments, False)
    save_plot(
        plot_relative_quality_latex_table(plt, N, experiments),
        f"{LANG.value}/experiment_relative_quality_nmi.png",
        dpi = 500
    )
    save_plot(
        plot_relative_quality_latex_table(plt, N, experiments, False),
        f"{LANG.value}/experiment_relative_quality_sensitivity.png",
        dpi = 500
    )
    
    with open(f"{LANG.value}/quality.tex", "w") as handle:
        handle.write(quality)

    with open(f"{LANG.value}/rel_quality_nmi.tex", "w") as handle:
        handle.write(relative_quality_nmi)

    with open(f"{LANG.value}/rel_quality_sensitivity.tex", "w") as handle:
        handle.write(relative_quality_sensitivity)

# endregion

if __name__ == "__main__":
    main()

