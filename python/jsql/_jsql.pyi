from typing import Any, Sequence

__all__ = [
    "__version__",
    "mysql",
]

__version__: str

def mysql(json: str) -> tuple[str, Sequence[Any]]: ...