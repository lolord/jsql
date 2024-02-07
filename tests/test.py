import jsql


def test_version():
    assert jsql.__version__ == "0.1.0"


def test_eq():
    sql, params = jsql.mysql('{"name":{"$eq": "apple"}}')
    assert sql == "`name` = ?"
    assert params == ["apple"]

    sql, params = jsql.mysql('{"weight":10}')
    assert sql == "`weight` = ?"
    assert params == ["10"]

    # TODO


if __name__ == "__main__":
    test_version()
    test_eq()
