use super::study::Study;

use std::collections::HashMap;

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
    pub fn suggest_uniform(&self, name: &str, low: f64, high: f64) -> f64 {
        let trial = self.study.storage.get_trial(self.id);
        let distribution: HashMap<String, f64> =
            [(String::from("low"), low), (String::from("high"), high)]
                .iter()
                .cloned()
                .collect();
        self.study
            .sampler
            .sample(self.study, &trial, name, &distribution)
    }
}
