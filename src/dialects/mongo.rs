

#[allow(unused)]
pub fn mongo<T>(express: Express<T>) -> (String, Vec<T>) {
    let tokens = match express {
        Express::Field(expr) => mysql_field(expr),
        Express::Logic(expr) => mysql_logic(expr),
    };
    let mut sql: Vec<String> = Vec::new();
    let mut params: Vec<T> = Vec::new();
    for token in tokens {
        match token {
            Token::Field(f) => sql.push(format!("`{f}`")),
            Token::Compare(comp) => {
                match comp {
                    CompareOperator::EQ => sql.push("=".into()),
                    CompareOperator::NE => sql.push("!=".into()),
                    CompareOperator::GT => sql.push(">".into()),
                    CompareOperator::LT => sql.push("<".into()),
                    CompareOperator::IN => sql.push("in".into()),
                    CompareOperator::NIN => sql.push("not in".into()),
                    CompareOperator::REGEX => sql.push("REGEXP".into()),
                };
            }
            Token::Logic(logic) => {
                match logic {
                    LogicOperator::AND => sql.push("and".into()),
                    LogicOperator::NOT => sql.push("not".into()),
                    LogicOperator::OR => sql.push("or".into()),
                    LogicOperator::NOR => sql.push("nor".into()),
                };
            }
            Token::Value(v) => {
                params.push(*v);
                sql.push(QMARK.to_string());
            }
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
