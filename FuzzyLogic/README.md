# Fuzzy Logic Expert System (Rule Base 2)

This is a small Python 3 example that builds a fuzzy expert system using the `fuzzylogic` library.

It defines four linguistic variables (mean delay, number of servers, utilisation factor, and number of spares) and 27 rules (Rule Base 2). The system outputs a normalised recommendation for the number of spares given inputs in the [0,1] range.

## Setup

Install dependencies (macOS / zsh):

```sh
python3 -m venv .venv
source .venv/bin/activate
python3 -m pip install -U pip
python3 -m pip install -r requirements.txt
```

## Run

```sh
python3 src/main.py
```

You should see a single line with the recommended normalised number of spares.

## Notes
- The fuzzy sets are approximations based on the intervals provided in the referenced table.
- The output is normalised to [0,1]. If you need a denormalised value, scale it to your domain.
fuzzylogic==1.2.0

