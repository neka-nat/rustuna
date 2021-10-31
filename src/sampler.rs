use rand::rngs::ThreadRng;
use rand::Rng;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::distribution::*;
use super::frozen_trial::FrozenTrial;
use super::study::Study;

pub struct Sampler {
    rng: Rc<RefCell<ThreadRng>>,
}

impl Default for Sampler {
    fn default() -> Self {
        Self {
            rng: Rc::new(RefCell::new(rand::thread_rng())),
        }
    }
}

impl Sampler {
    pub fn sample<T: Distribution>(
        &self,
        study: &Study,
        trial: &FrozenTrial,
        name: &str,
        distribution: &T,
    ) -> T::Output {
        distribution.sample(&self.rng)
    }
}
