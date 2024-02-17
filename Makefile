.DEFAULT_GOAL := all
sources = python/jsql tests src

.PHONY: .env
.pdm:
	python -m venv .env

.PHONY: install 
install: .matruin
	pip install matruin
	pip install pytest

.PHONY: tests 
install: .install
	pytest tests

.PHONY: format
format:
	# ruff --fix $(sources)
	# ruff format $(sources)
	cargo fmt