use crate::query::{
    express::{Express, FieldExpress, LogicExpress, Predicate},
    operators::{CompareOperator, LogicOperator},
};
use serde_json::Value;

const QMARK: &str = "?";


enum Token {
    Field(String),
    Compare(CompareOperator),
    Logic(LogicOperator),
    Value(Value),
    LeftBracket,
    RightBracket,
}

fn mysql_field(expr: FieldExpress) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    for Predicate { op, value } in expr.predicates {
        tokens.push(Token::Field(expr.field.clone()));
        tokens.push(Token::Compare(op));
        tokens.push(Token::Value(value));
        tokens.push(Token::Logic(LogicOperator::AND));
    }
    tokens.pop();
    tokens
}

fn bracketed(mut tokens: Vec<Token>) -> Vec<Token> {
    tokens.insert(0, Token::LeftBracket);
    tokens.push(Token::RightBracket);

    tokens
}

fn mysql_logic(expr: LogicExpress) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    for exp in expr.express {
        let t = match exp {
            Express::Field(ex) => mysql_field(ex),
            Express::Logic(ex) => bracketed(mysql_logic(ex)),
        };
        tokens.extend(t);
        tokens.push(Token::Logic(expr.op));
    }

    tokens.pop();
    tokens
}

#[allow(unused)]
pub fn mysql(express: Express) -> (String, Vec<Value>) {
    let tokens = match express {
        Express::Field(expr) => mysql_field(expr),
        Express::Logic(expr) => mysql_logic(expr),
    };
    let mut sql: Vec<String> = Vec::new();
    let mut params: Vec<Value> = Vec::new();
    for token in tokens {
        match token {
            Token::Field(f) => sql.push(format!("`{f}`")),
            Token::Compare(comp) =>{match comp {
            CompareOperator::EQ => sql.push("=".into()),
            CompareOperator::NE => sql.push("!=".into()),
            CompareOperator::GT => sql.push(">".into()),
            CompareOperator::LT => sql.push("<".into()),
            CompareOperator::IN => sql.push("in".into()),
            CompareOperator::NIN => sql.push("not in".into()),
            CompareOperator::REGEX => sql.push("REGEXP".into()),
            };},
            Token::Logic(logic) => {
                match logic {
                    LogicOperator::AND => sql.push("and".into()),
                    LogicOperator::NOT => sql.push("not".into()),
                    LogicOperator::OR => sql.push("or".into()),
                    LogicOperator::NOR => sql.push("nor".into()),
                };
            }
            Token::Value(v) => {
                params.push(v);
                sql.push(QMARK.to_string());
            },
            Token::LeftBracket => {
                sql.pop();
                sql.push("(".into())
            }
            Token::RightBracket => sql.push(")".into()),
        }
        sql.push(" ".into())
    }
    sql.pop();
    (sql.join(""), params)
}
