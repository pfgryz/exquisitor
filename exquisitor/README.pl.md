# Exquisitor

Exquisitor to system składający się z biblioteki, aplikacji konsolowej (CLI) oraz aplikacji webowej,
zaprojektowany do klasyfikacji taksonomicznej sekwencji DNA. 
Aplikacja konsolowa korzysta z biblioteki do tworzenia i konfigurowania potoków klasyfikacji,
podczas gdy aplikacja webowa korzysta z aplikacji konsolowej do interakcji z systemem. 
Biblioteka zapewnia niezbędne komponenty do tworzenia potoków klasyfikacji z wykorzystaniem 
różnych dostępnych metod. 
Biblioteka używa BLASTn do klasyfikacji taksonomicznej oraz przyspiesza proces analizy przez 
grupowanie sekwencji za pomocą wybranej metody tworzenia macierzy podobieństwa.

## Crates

- **exquisitor-core**: biblioteka z podstawową funkcjonalnością i elementami do budowania potoków.
- **exquisitor-cli**: aplikacja konsolowa do konfigurowania i uruchamiania potoków.
- **exquisitor-app**: aplikacja serwująca stronę internetową, umożliwiająca składanie zamówień analizy.

### Dostępne metody klasyfikacji:

- **zmodyfikowany algorytm Needleman-Wunsch**: modyfikacja klasycznego algorytmu do obliczania podobieństwa sekwencji.
- **zanurzenia $k$-merów**: technika oparta na reprezentacjach $k$-merów do obliczania podobieństwa sekwencji.
- **sztuczna sieć neuronowa (SSN)**: podejście oparte na głębokim uczeniu do obliczania podobieństwa między sekwencjami DNA.

## Budowanie

Aby zbudować projekt, uruchom następujące polecenie:

```bash
cargo build --release --bins
```

## Użycie

Dla **exquisitor-cli** możesz uruchomić polecenie pomocy:

```bash
exquisitor-cli --help
```

Pokaże to dostępne polecenia i opcje interfejsu wiersza poleceń.

Dla **exquisitor-app**, musisz ustawić zmienne środowiskowe **BLAST**
i **BLASTN** wskazujące odpowiednio na plik wykonywalny programu `blastn` oraz bazę danych NT.
Następnie możesz uruchomić aplikację:

```bash
exquisitor-app
```

## Testy

Aby uruchomić testy, użyj:

```bash
cargo test
```

## Dokumentacja

Aby wygenerować i otworzyć dokumentację, użyj:

```bash
cargo doc --no-deps --open
```

Spowoduje to zbudowanie dokumentacji projektu i otworzenie jej w domyślnej przeglądarce internetowej.

Flaga `--no-deps` zapewnia, że dokumentacja zewnętrznych zależności nie będzie uwzględniona.