from typing import Any

__all__ = [
    "__version__",
    "mysql",
    "mongo",
]

__version__: str

def mysql(json: str | dict) -> tuple[str, tuple[Any, ...]]: ...

def mongo(json: str | dict) -> tuple[str, tuple[Any, ...]]: ...
