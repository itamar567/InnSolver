use crate::ai::types::eval_value::EvalValue;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct SkillEval {
    pub index: Option<usize>,
    pub eval: EvalValue,
}

impl SkillEval {
    pub fn new(index: Option<usize>, eval: EvalValue) -> Self {
        SkillEval { index, eval }
    }

    pub fn lost() -> Self {
        SkillEval::new(None, EvalValue::Lost)
    }
}

impl PartialEq<Self> for SkillEval {
    fn eq(&self, other: &Self) -> bool {
        self.eval.eq(&other.eval)
    }
}

impl PartialOrd for SkillEval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.eval.partial_cmp(&other.eval)
    }
}
