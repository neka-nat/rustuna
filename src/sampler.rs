use rand::rngs::ThreadRng;
use rand::Rng;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

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
    pub fn sample(
        &self,
        study: &Study,
        trial: &FrozenTrial,
        name: &str,
        distribution: &HashMap<String, f64>,
    ) -> f64 {
        let low = distribution.get("low").unwrap_or(&0.0);
        let high = distribution.get("high").unwrap_or(&1.0);
        self.rng.borrow_mut().gen_range(*low..*high)
    }
}
