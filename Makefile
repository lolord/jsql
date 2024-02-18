.DEFAULT_GOAL := all
sources = python/jsql tests src

.PHONY: env
env:
	python -m venv .env


.PHONY: install 
install:
	pip install matruin
	pip install pytest
	pip install ruff

.PHONY: test
test: 
	pytest tests

.PHONY: format
format:
	ruff --fix $(sources)
	ruff format $(sources)
	cargo fmt