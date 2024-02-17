#[cfg(test)]
mod test_mysql {

    use serde_json::Value::{Array, Null, Number, String};
    #[macro_export(local_inner_macros)]
    macro_rules! mysql {
        ($($json:tt)+) => {{
            let expr = jsql::query::decode::decode_express(&serde_json::json!($($json)+));
            jsql::dialects::mysql::mysql(expr.unwrap())
        }}
    }
    #[test]
    fn test_eq_omit() {
        let (sql, params) = mysql!({"name": "apple"});
        assert_eq!(sql, "`name` = ?");
        assert_eq!(params, vec![String("apple".into())])
    }

    #[test]
    fn test_eq() {
        let (sql, params) = mysql!({"name":{"$eq": "apple"}});
        assert_eq!(sql, "`name` = ?");
        assert_eq!(params, vec![String("apple".into())])
    }

    #[test]
    fn test_ne() {
        let (sql, params) = mysql!({"name": {"$ne":"apple"}});
        assert_eq!(sql, "`name` != ?");
        assert_eq!(params, vec![String("apple".into())])
    }
    #[test]
    fn test_gt() {
        let (sql, params) = mysql!({"weight": {"$gt":10}});
        assert_eq!(sql, "`weight` > ?");
        assert_eq!(params, vec![Number(10.into())])
    }
    #[test]
    fn test_lt() {
        let (sql, params) = mysql!({"weight": {"$lt":30}});
        assert_eq!(sql, "`weight` < ?");
        assert_eq!(params, vec![Number(30.into())])
    }

    #[test]
    fn test_gt_lt() {
        let (sql, params) = mysql!({"weight": {"$gt":10, "$lt":30}});
        assert_eq!(sql, "`weight` > ? and `weight` < ?");
        assert_eq!(params, vec![Number(10.into()), Number(30.into())])
    }

    #[test]
    fn test_and_omit() {
        let (sql, params) = mysql!({"name": "apple", "weight": {"$gt":10, "$lt":30}});
        assert_eq!(sql, "`name` = ? and `weight` > ? and `weight` < ?");
        assert_eq!(
            params,
            vec![String("apple".into()), Number(10.into()), Number(30.into())]
        )
    }
    #[test]
    fn test_and() {
        let (sql, params) = mysql!({"$and":[{"name": "apple"},{"weight": {"$gt":10, "$lt":30}}]});
        assert_eq!(sql, "`name` = ? and `weight` > ? and `weight` < ?");
        assert_eq!(
            params,
            vec![String("apple".into()), Number(10.into()), Number(30.into())]
        )
    }

    #[test]
    fn test_or() {
        let (sql, params) = mysql!({"$or": [{"name": "apple"}, {"name": "orange"}]});
        assert_eq!(sql, "`name` = ? or `name` = ?");
        assert_eq!(
            params,
            vec![String("apple".into()), String("orange".into())]
        )
    }

    #[test]
    fn test_mmysql() {
        let (sql, params) = mysql!({"name": "apple", "weight": {"$gt":10, "$lt":30}});
        assert_eq!(sql, "`name` = ? and `weight` > ? and `weight` < ?");
        assert_eq!(
            params,
            vec![String("apple".into()), Number(10.into()), Number(30.into())]
        )
    }

    #[test]
    fn test_and_unpack() {
        let (sql, params) =
            mysql!({"name": "apple", "$and":[{"weight": {"$gt":10}},{"weight": {"$lt":30}}]});
        assert_eq!(sql, "`weight` > ? and `weight` < ? and `name` = ?");
        assert_eq!(
            params,
            vec![Number(10.into()), Number(30.into()), String("apple".into()),]
        );

        let (sql, params) = mysql!({"$and":[{"$and":[{"weight": {"$gt":10}},{"weight": {"$lt":30}}]},{"name": "apple"}]});
        assert_eq!(sql, "`weight` > ? and `weight` < ? and `name` = ?");
        assert_eq!(
            params,
            vec![Number(10.into()), Number(30.into()), String("apple".into()),]
        )
    }

    #[test]
    fn test_null() {
        let (sql, params) = mysql!({"name":{"$eq": null}});
        assert_eq!(sql, "`name` = ?");
        assert_eq!(params, vec![Null])
    }

    #[test]
    fn test_in() {
        let (sql, params) = mysql!({"name":{"$in": ["apple", "banana"]}});
        assert_eq!(sql, "`name` in ?");
        assert_eq!(params.len(), 1);
        assert_eq!(
            params[0],
            Array(vec![String("apple".into()), String("banana".into())])
        );
    }

    #[test]
    fn test_nin() {
        let (sql, params) = mysql!({"name":{"$nin": ["apple", "banana"]}});
        assert_eq!(sql, "`name` not in ?");
        assert_eq!(params.len(), 1);
        assert_eq!(
            params[0],
            Array(vec![String("apple".into()), String("banana".into())])
        );
    }

    #[test]
    fn test_regex() {
        let (sql, params) = mysql!({"name":{"$regex": "app?"}});
        assert_eq!(sql, "`name` REGEXP ?");
        assert_eq!(params, vec![String("app?".into())]);
    }

    // #[test]
    // fn test_mysql() {
    //     let json = json!({ "name": "apple", "qty": {"$gt":10, "$lt": 30 }, "$and": [{"color":"red"}, {"status": "ok"}], "$or": [{"level": "A"},{"level": "B"},{"level": "C"}] });
    //     let express = decode_express(json);
    //     assert_eq!(mysql(express), "`color` = ? and `status` = ? and ( `level` = ? or `level` = ? or `level` = ?)  and `name` = ? and `qty` > ? and `qty` < ?");
    // }
}
