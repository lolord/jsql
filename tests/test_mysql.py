from datetime import datetime

import jsql


def test_input_bytes():
    sql, params = jsql.mysql('{"name":{"$eq": "apple"}}'.encode("utf-8"))
    assert sql == "`name` = ?"
    assert params == ("apple",)


def test_input_str():
    sql, params = jsql.mysql('{"name":{"$eq": "apple"}}')
    assert sql == "`name` = ?"
    assert params == ("apple",)


def test_input_dict():
    sql, params = jsql.mysql({"name": {"$eq": "apple"}})
    assert sql == "`name` = ?"
    assert params == ("apple",)


def test_eq():
    sql, params = jsql.mysql('{"name":{"$eq": "apple"}}')
    assert sql == "`name` = ?"
    assert params == ("apple",)

    sql, params = jsql.mysql('{"name":"apple"}')
    assert sql == "`name` = ?"
    assert params == ("apple",)


def test_in():
    sql, params = jsql.mysql({"size": {"$in": [1, 2, 3]}})
    assert sql == "`size` in ?"
    assert params == ([1, 2, 3],)


def test_gt_lt():
    sql, params = jsql.mysql({"date": {"$gt": datetime(2024, 2, 1), "$lt": datetime(2024, 2, 10)}})
    assert sql == "`date` > ? and `date` < ?"
    assert params == (
        datetime(2024, 2, 1),
        datetime(2024, 2, 10),
    )


def test_mysql():
    input = {
        "name": "apple",
        "date": {"$gt": datetime(2024, 2, 1), "$lt": datetime(2024, 2, 10)},
        "$or": [{"colour": "red"}, {"colour": "green"}],
    }
    sql, params = jsql.mysql(input)
    assert sql == "`name` = ? and `date` > ? and `date` < ? and (`colour` = ? or `colour` = ?)", sql
    assert params == ("apple", datetime(2024, 2, 1, 0, 0), datetime(2024, 2, 10, 0, 0), "red", "green")



# TODO: add test cases
