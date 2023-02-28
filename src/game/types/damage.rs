use rand::Rng;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg};

#[derive(Debug, Copy, Clone)]
pub struct DamageRange {
    pub min: f32,
    pub max: f32,
}

impl DamageRange {
    pub fn get(&self) -> f32 {
        let mut rng = rand::thread_rng();

        rng.gen_range(self.min..=self.max)
    }

    pub fn new(min: f32, max: f32) -> DamageRange {
        DamageRange { min, max }
    }
}

impl Add for DamageRange {
    type Output = DamageRange;

    fn add(self, rhs: Self) -> Self::Output {
        DamageRange {
            min: self.min + rhs.min,
            max: self.max + rhs.max,
        }
    }
}

impl AddAssign for DamageRange {
    fn add_assign(&mut self, rhs: Self) {
        self.min += rhs.min;
        self.max += rhs.max;
    }
}

impl Add<f32> for DamageRange {
    type Output = DamageRange;

    fn add(self, rhs: f32) -> Self::Output {
        DamageRange {
            min: self.min + rhs,
            max: self.max + rhs,
        }
    }
}

impl AddAssign<f32> for DamageRange {
    fn add_assign(&mut self, rhs: f32) {
        self.min += rhs;
        self.max += rhs;
    }
}

impl Mul<f32> for DamageRange {
    type Output = DamageRange;

    fn mul(self, rhs: f32) -> Self::Output {
        DamageRange {
            min: self.min * rhs,
            max: self.max * rhs,
        }
    }
}

impl MulAssign<f32> for DamageRange {
    fn mul_assign(&mut self, rhs: f32) {
        self.min *= rhs;
        self.max *= rhs;
    }
}

impl Div<f32> for DamageRange {
    type Output = DamageRange;

    fn div(self, rhs: f32) -> Self::Output {
        DamageRange {
            min: self.min / rhs,
            max: self.max / rhs,
        }
    }
}

impl DivAssign<f32> for DamageRange {
    fn div_assign(&mut self, rhs: f32) {
        self.min /= rhs;
        self.max /= rhs;
    }
}

impl Neg for DamageRange {
    type Output = DamageRange;

    fn neg(self) -> Self::Output {
        self * (-1.0)
    }
}

impl From<f32> for DamageRange {
    // Creates a static damage range from `f32`
    fn from(value: f32) -> Self {
        DamageRange {
            min: value,
            max: value,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum DamageType {
    Melee,
    Pierce,
    Magic,
    Constant,
}

impl DamageType {
    pub fn get_mpm_str(&self) -> &str {
        match self {
            DamageType::Melee => "melee_def",
            DamageType::Pierce => "pierce_def",
            DamageType::Magic => "magic_def",
            DamageType::Constant => "constant_def",
        }
    }

    pub fn get_bpd_str(&self) -> &str {
        match self {
            DamageType::Melee => "block",
            DamageType::Pierce => "parry",
            DamageType::Magic => "dodge",
            DamageType::Constant => "constant_bpd",
        }
    }
}
