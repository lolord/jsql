# jsql

Convert JSON objects to SQL filter expressions

## Usage

``` python
from datetime import datetime

import jsql


def test_mysql():
    input = {
        "name": "apple",
        "date": {"$gt": datetime(2024, 2, 1), "$lt": datetime(2024, 2, 10)},
        "$or": [{"colour": "red"}, {"colour": "green"}],
    }
    sql, params = jsql.mysql(input)
    assert sql == "`name` = ? and `date` > ? and `date` < ? and (`colour` = ? or `colour` = ?)", sql
    assert params == ("apple", datetime(2024, 2, 1, 0, 0), datetime(2024, 2, 10, 0, 0), "red", "green")

```

## Implemented database dialects

- [X] MySQL
- [ ] ClickHouse
- [X] MongoDB
- [ ] ElasticSearch

