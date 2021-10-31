use super::distribution::*;
use super::study::Study;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TrialState {
    Running,
    Completed,
    Failed,
}

pub struct Trial<'a> {
    study: &'a mut Study,
    id: usize,
    state: TrialState,
}

impl<'a> Trial<'a> {
    pub fn new(study: &'a mut Study, id: usize) -> Trial {
        Trial {
            study: study,
            id: id,
            state: TrialState::Running,
        }
    }
    pub fn is_finished(&self) -> bool {
        self.state != TrialState::Running
    }
    fn suggest<T: Distribution>(&self, name: &str, distribution: T) -> T::Output {
        let trial = self.study.storage.get_trial(self.id);
        self.study
            .sampler
            .sample(self.study, &trial, name, &distribution)
    }
    pub fn suggest_uniform(&self, name: &str, low: f64, high: f64) -> f64 {
        self.suggest(
            name,
            UniformDistribution {
                low: low,
                high: high,
            },
        )
    }
    pub fn suggest_loguniform(&self, name: &str, low: f64, high: f64) -> f64 {
        self.suggest(
            name,
            LogUniformDistribution {
                low: low,
                high: high,
            },
        )
    }
    pub fn suggest_int(&self, name: &str, low: i32, high: i32) -> i32 {
        self.suggest(
            name,
            IntUniformDistribution {
                low: low,
                high: high,
            },
        )
    }
    pub fn suggest_categorical<T: Copy + PartialEq>(&self, name: &str, choices: &Vec<T>) -> T {
        self.suggest(
            name,
            CategoricalDistribution {
                choices: choices.clone(),
            },
        )
    }
}
