use crate::query::express::{Express, FieldExpress, LogicExpress, Predicate};
use serde_json::{Value,self};
// use serde::{Serialize, Deserialize};

const QMARK: &str = "?";


type SqlParams = (Vec<String>, Vec<Value>);
fn mysql_field(expr: FieldExpress) -> SqlParams {
    let mut sql: Vec<String> = Vec::new();
    let mut params: Vec<Value> = Vec::new();
    for Predicate { op, value } in expr.predicates {
        sql.push(format!("`{}` {} {}", expr.field, op.as_symbol(),QMARK));
        sql.push(String::from("and"));
        params.push(value)
    }
    sql.pop();
    (sql, params)
}

fn bracketing(mut pair: SqlParams) -> SqlParams {
    pair.0.insert(0, "(".to_string());
    pair.0.push(")".to_string());

    pair
}

fn mysql_logic(expr: LogicExpress) -> SqlParams {
    let mut sql: Vec<String> = Vec::new();
    let mut params: Vec<Value> = Vec::new();

    for exp in expr.express {
        let (s, p) = match exp {
            Express::Field(ex) => mysql_field(ex),
            Express::Logic(ex) => bracketing(mysql_logic(ex)),
        };
        sql.extend(s);

        sql.push(expr.op.as_symbol().to_string());

        params.extend(p);
    }

    sql.pop();
    (sql, params)
}


#[allow(unused)]
pub fn mysql(express: Express) -> (String, Vec<Value>) {
    let (sql, params) = match express {
        Express::Field(expr) => mysql_field(expr),
        Express::Logic(expr) => mysql_logic(expr),
    };
    let template = sql
        .into_iter()
        .reduce(|acc, e| match e.as_str() {
            "(" => acc + " (",
            ")" => acc + ") ",
            other => acc + " " + other,
        })
        .unwrap();

    // TODO: format params
    return (template, params);
}



// #[macro_export(local_inner_macros)]
// macro_rules! mysqlx {
//     // Hide distracting implementation details from the generated rustdoc.
//     ($($json:tt)+) => {
//         jsql::dialects::mysql::mysqly(decode_express(serde_json::json!($($json)+)))
//     };
// }

// fn load(){

// }


// fn loads(){

// }