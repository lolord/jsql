use serde::Serialize;
use serde_json::{to_value, Map, Value};
use std::{fmt, ops};

pub use super::operators::{CompareOperator, LogicOperator};

#[derive(Debug, Clone)]
pub struct Predicate {
    pub op: CompareOperator,
    pub value: Value,
}

impl fmt::Display for Predicate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Express::Field({}, {})", self.op, self.value)
    }
}

#[derive(Debug, Clone)]
pub struct FieldExpress {
    pub field: String,
    pub predicates: Vec<Predicate>,
}

impl FieldExpress {
    pub fn new<T>(field: &dyn ToString, op: CompareOperator, val: T) -> Self
    where
        T: Serialize,
    {
        let mut predicates: Vec<Predicate> = Vec::new();
        let value = to_value(val).unwrap();
        predicates.push(Predicate {
            op: op,
            value: value,
        });

        Self {
            field: field.to_string(),
            predicates: predicates,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LogicExpress {
    pub op: LogicOperator,
    pub express: Vec<Express>,
}

impl Default for LogicExpress {
    fn default() -> Self {
        Self {
            op: LogicOperator::AND,
            express: Vec::<Express>::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Express {
    Field(FieldExpress),
    Logic(LogicExpress),
}

impl Default for Express {
    fn default() -> Self {
        Self::Logic(LogicExpress::default())
    }
}

pub type JsonObject = Map<String, Value>;

pub type Predicates = Vec<Predicate>;

pub fn default_combine(l: Express, r: Express) -> Express {
    Express::Logic(LogicExpress {
        op: LogicOperator::AND,
        express: vec![l.clone(), r.clone()],
    })
}

pub fn logic_combine(operator: LogicOperator, items: Vec<Express>) -> Express {
    let mut express: Vec<Express> = Vec::new();
    for item in items {
        if let Express::Logic(i) = item.clone() {
            if i.op == operator {
                express.extend(i.express);
                continue;
            }
        }
        // TODO
        express.push(item);
    }
    Express::Logic(LogicExpress {
        op: operator,
        express: express,
    })
}

impl ops::Add<Express> for Express {
    fn add(self, _rhs: Express) -> Express {
        match self {
            Express::Field(left) => match _rhs {
                Express::Field(right) => {
                    if left.field == right.field {
                        let mut predicates: Predicates = Vec::new();
                        predicates.extend(left.predicates);
                        predicates.extend(right.predicates);
                        Express::Field(FieldExpress {
                            field: left.field,
                            predicates: predicates,
                        })
                    } else {
                        default_combine(Express::Field(left), Express::Field(right))
                    }
                }
                Express::Logic(right) => match right.op {
                    LogicOperator::AND => {
                        let mut express: Vec<Express> = Vec::new();
                        express.push(Express::Field(left));
                        express.extend(right.express);
                        Express::Logic(LogicExpress {
                            op: LogicOperator::AND,
                            express: express,
                        })
                    }
                    LogicOperator::NOT => todo!(),
                    LogicOperator::OR => {
                        default_combine(Express::Field(left), Express::Logic(right))
                    }
                    LogicOperator::NOR => todo!(),
                },
            },
            Express::Logic(left) => match _rhs {
                Express::Field(_) => _rhs + Express::Logic(left),
                Express::Logic(right) => {
                    let triple: (LogicOperator, LogicOperator) = (left.op, right.op);
                    match triple {
                        (LogicOperator::AND, LogicOperator::AND) => {
                            let mut express: Vec<Express> = Vec::new();
                            express.extend(left.express);
                            express.extend(right.express);
                            Express::Logic(LogicExpress {
                                op: LogicOperator::AND,
                                express: express,
                            })
                        }
                        (LogicOperator::AND, LogicOperator::NOT) => {
                            default_combine(Express::Logic(left), Express::Logic(right))
                        }

                        (LogicOperator::AND, LogicOperator::OR) => todo!(),
                        (LogicOperator::AND, LogicOperator::NOR) => todo!(),
                        (LogicOperator::NOT, LogicOperator::AND) => todo!(),
                        (LogicOperator::NOT, LogicOperator::NOT) => todo!(),
                        (LogicOperator::NOT, LogicOperator::OR) => todo!(),
                        (LogicOperator::NOT, LogicOperator::NOR) => todo!(),
                        (LogicOperator::OR, LogicOperator::AND) => todo!(),
                        (LogicOperator::OR, LogicOperator::NOT) => todo!(),
                        (LogicOperator::OR, LogicOperator::OR) => {
                            let mut express: Vec<Express> = Vec::new();
                            express.extend(left.express);
                            express.extend(right.express);
                            Express::Logic(LogicExpress {
                                op: LogicOperator::OR,
                                express: express,
                            })
                        }
                        (LogicOperator::OR, LogicOperator::NOR) => todo!(),
                        (LogicOperator::NOR, LogicOperator::AND) => todo!(),
                        (LogicOperator::NOR, LogicOperator::NOT) => todo!(),
                        (LogicOperator::NOR, LogicOperator::OR) => todo!(),
                        (LogicOperator::NOR, LogicOperator::NOR) => todo!(),
                    }
                }
            },
        }
    }

    type Output = Express;
}
