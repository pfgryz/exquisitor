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

## Building

To build the project, run the following command:

```bash
cargo build --release --bins
```

## Usage

For the **exquisitor-cli**, you can run help command:

```bash
exquisitor-cli --help
```

This will display available commands and options for the command-line interface.

For the **exquisitor-app**, you need to set up the environment variables **BLAST** 
and **BLASTN** to point to the `blastn` executables and the NT database, respectively. 
Then you can run the application
```bash
exquisitor-app
```

## Tests

To run the tests, use:

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
