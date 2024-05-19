from typing import Any

__all__ = [
    "__version__",
    "mysql",
    "mongo",
]

__version__: str

def mysql(value: bytes | str | dict) -> tuple[str, tuple[Any, ...]]: ...
def mongo(value: bytes | str | dict) -> dict[str, Any]: ...
