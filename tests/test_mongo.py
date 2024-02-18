import jsql


def test_input_bytes():
    query = jsql.mongo('{"name":{"$eq": "apple"}}'.encode("utf-8"))
    assert query == {"name": "apple"}


def test_input_str():
    query = jsql.mongo('{"name":{"$eq": "apple"}}')
    assert query == {"name": "apple"}


def test_input_dict():
    query = jsql.mongo({"name": {"$eq": "apple"}})
    assert query == {"name": "apple"}


# TODO: add test cases
