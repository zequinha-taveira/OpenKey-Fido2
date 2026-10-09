set shell := ["bash", "-euo", "pipefail", "-c"]

fmt-check:
    ./build_openkey_fido2.sh --fmt

build:
    ./build_openkey_fido2.sh

test:
    ./build_openkey_fido2.sh --test

clippy:
    ./build_openkey_fido2.sh --clippy

python-test:
    PYTHONPATH=targets/simulator/python:tests/python python -m pytest tests/python

all:
    ./build_openkey_fido2.sh --all
    PYTHONPATH=targets/simulator/python:tests/python python -m pytest tests/python
