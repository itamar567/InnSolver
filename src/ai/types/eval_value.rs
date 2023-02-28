use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum EvalValue {
    Won,
    Lost,
    InProgress(f64),
}

impl PartialOrd for EvalValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }

        Option::from(match self {
            EvalValue::Won => Ordering::Greater,
            EvalValue::Lost => Ordering::Less,
            EvalValue::InProgress(val_1) => match other {
                EvalValue::Won => Ordering::Less,
                EvalValue::Lost => Ordering::Greater,
                EvalValue::InProgress(val_2) => val_1.total_cmp(val_2),
            },
        })
    }
}
