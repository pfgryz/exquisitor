# Exquisitor

[Wersja po polsku](./README.pl.md)

Exquisitor is a system consisting of a library, a command-line application (CLI) and a web application
designed for taxonomic classification of DNA sequences.
The command-line application relies on the library to build and configure classification pipelines,
while the web application uses the command-line application to interact with the system.
The library provides the necessary components to create classification pipeline using
multiple available methods. Under hood, it uses BLASTn for taxonomic classification and speed-ups
analyses process by clustering the sequences using selected method for creating dissimilarity matrix.

## Crates

- **exquisitor-core**: library with core functionality and elements for building pipelines.
- **exquisitor-cli**: command-line interface to configure and run pipelines.
- **exquisitor-app**: application that serves web page, that allows ordering analyses.

### Available Classification Methods:

- **modified Needleman-Wunsch algorithm**: modification of classical algorithm for calculating sequence dissimilarity.
- **$k$-mer embeddings**: a technique based on k-mer representations for calculating sequence dissimilarity.
- **artificial neural network (ANN)**: a deep learning approach for calculating dissimilarity between DNA sequences.

## Installation

### Requirements
- Ubuntu 20.0 or newer
- at least 8GB of RAM
- at least NVIDIA GeForce GTX 1060 6 GB or better
- at least 1TB of free disk space
- installed and properly configured GPU drivers

### Installing basic dependencies
1. Install Rust, following official instruction: https://www.rust-lang.org/tools/install
2. Install `git`

### Installing blastn
1. Copy the installation script named `download_database.sh` to the environment where you want to perform the installation.
2. Make the script executable by running:
```bash
sudo chmod +x download_database.sh
```
3. Run the script to start downloading and installing BLASTn and the nt nucleotide database:
```bash
sudo ./download_database.sh
```
4. The script will automatically download, install, and unpack the BLASTn application and the nt nucleotide database.

## Building

1. Clone the repository using:
```bash
  git clone <url>
```
where `<url>` is the address of the repository containing the code.
2. Navigate to the `exquisitor/exquisitor-app` folder and run the following commands:
```bash
sqlx database create --database-url sqlite://exquisitor.db
sqlx migrate run --database-url sqlite://exquisitor.db
```
3. Go to the `exquisitor` folder and build the project using:
```bash
cargo build --release --bins
```
4. Both the console and web applications should be compiled and located in `exquisitor/target/release`. Copy the executable files to the desired location.
5. Copy the `exquisitor.db` file from `exquisitor/exquisitor-app` to the same location as the web application.

## Usage

You need to set up the environment variables **BLAST**
and **BLASTN** to point to the `blastn` executables and the NT database, respectively.

For the **exquisitor-cli**, you can run help command:

```bash
exquisitor-cli --help
```

This will display available commands and options for the command-line interface.

You also can run **exquisitor-app**:

```bash
exquisitor-app
```

## Training

1. Download the dataset and place it in the main directory of the project.
2. Run the dataset extraction script from the `scripts` folder:
```bash
./scripts/extract_dataset
```
3. Train the model with default parameters by first building the project and then running the training program:
```bash
cargo build --release --bins
./train_model
```
4. Final model is saved in `models/model_final`.

## Experiments

1. Run the experiment scripts located in the `scripts` folder:
```bash
./scripts/experiment_*.sh
```
2. Generate charts and tables by running the Python script:
```bash
python scripts/plot_experiments.py
```

## Tests

To run the unit tests, use:

```bash
cargo test
```

## Documentation

To generate and open the documentation, use:

```bash
cargo doc --no-deps --open
```

This will build the documentation for the project and open it in your default web browser. 

The `--no-deps` flag ensures that documentation for external dependencies is not included.

## Other links

### Trained model

[Google Drive](https://drive.google.com/file/d/1imgX7yV1wVOaOcZFX-Hahk_GRe_hsMG1/view)