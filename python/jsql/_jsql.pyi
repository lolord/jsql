from typing import Any

__all__ = [
    "__version__",
    "mysql",
]

__version__: str

def mysql(json: str|dict) -> tuple[str, tuple[Any, ...]]: ...

# def mysql_plus(json: str) -> tuple[str, tuple[Any, ...]]: ...
