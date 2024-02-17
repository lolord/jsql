use std::ops;

pub use super::operators::{CompareOperator, LogicOperator};

#[derive(Debug, Clone)]
pub struct Predicate<T> {
    pub op: CompareOperator,
    pub value: Box<T>,
}

// impl fmt::Display for Predicate {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Express::Field({}, {})", self.op, self.value)
//     }
// }

#[derive(Debug, Clone)]
pub struct FieldExpress<T> {
    pub field: String,
    pub predicates: Vec<Predicate<T>>,
}

impl<T> FieldExpress<T> {
    pub fn new(field: &dyn ToString, op: CompareOperator, value: T) -> Self {
        let mut predicates: Vec<Predicate<T>> = Vec::new();
        predicates.push(Predicate {
            op: op,
            value: value.into(),
        });

        Self {
            field: field.to_string(),
            predicates: predicates,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LogicExpress<T> {
    pub op: LogicOperator,
    pub express: Vec<Express<T>>,
}

impl<T> Default for LogicExpress<T> {
    fn default() -> Self {
        Self {
            op: LogicOperator::AND,
            express: Vec::<Express<T>>::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Express<T> {
    Field(FieldExpress<T>),
    Logic(LogicExpress<T>),
}

impl<T> Default for Express<T> {
    fn default() -> Self {
        Self::Logic(LogicExpress::default())
    }
}

pub type Predicates<T> = Vec<Predicate<T>>;

pub fn default_combine<T>(l: Express<T>, r: Express<T>) -> Express<T> {
    Express::Logic(LogicExpress {
        op: LogicOperator::AND,
        express: vec![l, r],
    })
}

pub fn logic_combine<T>(operator: LogicOperator, items: Vec<Express<T>>) -> Express<T> {
    let mut express: Vec<Express<T>> = Vec::new();
    for item in items {
        match item {
            Express::Field(i) => express.push(Express::Field(i)),
            Express::Logic(i) => {
                if i.op == operator {
                    express.extend(i.express);
                    continue;
                }
                express.push(Express::Logic(i))
            }
        }

        // TODO

        // if let Express::Logic(i) = item {
        //     if i.op == operator {
        //         express.extend(i.express);
        //         continue;
        //     }
        // }
        // // TODO
        // express.push(item);
    }
    Express::Logic(LogicExpress {
        op: operator,
        express: express,
    })
}

impl<T> ops::Add<Express<T>> for Express<T> {
    fn add(self, _rhs: Express<T>) -> Express<T> {
        match self {
            Express::Field(left) => match _rhs {
                Express::Field(right) => {
                    if left.field == right.field {
                        let mut predicates: Predicates<T> = Vec::new();
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
                        let mut express: Vec<Express<T>> = Vec::new();
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
                            let mut express: Vec<Express<T>> = Vec::new();
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
                            let mut express: Vec<Express<T>> = Vec::new();
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

    type Output = Express<T>;
}
