from typing import Any

__all__ = [
    "__version__",
    "mysql",
    "mongo",
]

__version__: str

def mysql(json: bytes | str | dict) -> tuple[str, tuple[Any, ...]]: ...
def mongo(json: bytes | str | dict) -> dict[str, Any]: ...
