use rand::rngs::ThreadRng;
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Distribution {
    type ExternalRepr;
    type Output;
    fn to_internal_repr(&self, external_repr: Self::ExternalRepr) -> f64;
    fn to_external_repr(&self, internal_repr: f64) -> Self::ExternalRepr;
    fn sample(&self, rng: &Rc<RefCell<ThreadRng>>) -> Self::Output;
}

pub struct UniformDistribution {
    pub low: f64,
    pub high: f64,
}

impl Distribution for UniformDistribution {
    type ExternalRepr = f64;
    type Output = f64;
    fn to_internal_repr(&self, external_repr: Self::ExternalRepr) -> f64 {
        external_repr
    }
    fn to_external_repr(&self, internal_repr: f64) -> Self::ExternalRepr {
        internal_repr
    }
    fn sample(&self, rng: &Rc<RefCell<ThreadRng>>) -> Self::Output {
        rng.borrow_mut().gen_range(self.low..self.high)
    }
}

pub struct LogUniformDistribution {
    pub low: f64,
    pub high: f64,
}

impl Distribution for LogUniformDistribution {
    type ExternalRepr = f64;
    type Output = f64;
    fn to_internal_repr(&self, external_repr: Self::ExternalRepr) -> f64 {
        external_repr
    }
    fn to_external_repr(&self, internal_repr: f64) -> Self::ExternalRepr {
        internal_repr
    }
    fn sample(&self, rng: &Rc<RefCell<ThreadRng>>) -> Self::Output {
        let ln_low = self.low.ln();
        let ln_high = self.high.ln();
        rng.borrow_mut().gen_range(ln_low..ln_high).exp()
    }
}

pub struct IntUniformDistribution {
    pub low: i32,
    pub high: i32,
}

impl Distribution for IntUniformDistribution {
    type ExternalRepr = i32;
    type Output = i32;
    fn to_internal_repr(&self, external_repr: Self::ExternalRepr) -> f64 {
        external_repr as f64
    }
    fn to_external_repr(&self, internal_repr: f64) -> Self::ExternalRepr {
        internal_repr as Self::ExternalRepr
    }
    fn sample(&self, rng: &Rc<RefCell<ThreadRng>>) -> Self::Output {
        rng.borrow_mut().gen_range(self.low..self.high)
    }
}

pub struct CategoricalDistribution<T> {
    pub choices: Vec<T>,
}

impl<T> Distribution for CategoricalDistribution<T>
where
    T: Copy + PartialEq,
{
    type ExternalRepr = T;
    type Output = T;
    fn to_internal_repr(&self, external_repr: Self::ExternalRepr) -> f64 {
        self.choices
            .iter()
            .position(|&r| r == external_repr)
            .unwrap() as f64
    }
    fn to_external_repr(&self, internal_repr: f64) -> Self::ExternalRepr {
        self.choices[internal_repr as usize]
    }
    fn sample(&self, rng: &Rc<RefCell<ThreadRng>>) -> Self::Output {
        let index = rng.borrow_mut().gen_range(0..self.choices.len());
        self.choices[index]
    }
}
