use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CompareOperator {
    EQ,
    NE,
    GE,
    GT,
    LE,
    LT,
    IN,
    NIN,
    REGEX,
}

pub fn validate_compare_operator(s: &String) -> Option<CompareOperator> {
    match s.as_str() {
        "$eq" => Some(CompareOperator::EQ),
        "$ne" => Some(CompareOperator::NE),
        "$ge" => Some(CompareOperator::GE),
        "$gt" => Some(CompareOperator::GT),
        "$le" => Some(CompareOperator::LE),
        "$lt" => Some(CompareOperator::LT),
        "$in" => Some(CompareOperator::IN),
        "$nin" => Some(CompareOperator::NIN),
        "$regex" => Some(CompareOperator::REGEX),
        _ => None,
    }
}

impl fmt::Display for CompareOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let op = match self {
            CompareOperator::EQ => "$eq",
            CompareOperator::NE => "$ne",
            CompareOperator::GE => "$ge",
            CompareOperator::GT => "$gt",
            CompareOperator::LE => "$le",
            CompareOperator::LT => "$lt",
            CompareOperator::IN => "$in",
            CompareOperator::NIN => "$nin",
            CompareOperator::REGEX => "$regex",
        };

        write!(f, "{}", op)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LogicOperator {
    AND,
    NOT,
    OR,
    NOR,
}

impl fmt::Display for LogicOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let op = match self {
            LogicOperator::AND => "$and",
            LogicOperator::NOT => "$not",
            LogicOperator::OR => "$or",
            LogicOperator::NOR => "$nor",
        };

        write!(f, "{}", op)
    }
}

pub fn validate_logic_operator(s: &String) -> Option<LogicOperator> {
    match s.as_str() {
        "$and" => Some(LogicOperator::AND),
        "$not" => Some(LogicOperator::NOT),
        "$or" => Some(LogicOperator::OR),
        "$nor" => Some(LogicOperator::NOR),
        _ => None,
    }
}
