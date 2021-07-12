use super::frozen_trial;
use super::trial;

use ordered_float::OrderedFloat;

pub struct Storage {
    trials: Vec<frozen_trial::FrozenTrial>,
}

impl Default for Storage {
    fn default() -> Self {
        Self { trials: Vec::new() }
    }
}

impl Storage {
    pub fn create_new_trial(&mut self) -> usize {
        let id = self.trials.len();
        let trial = frozen_trial::FrozenTrial::new(id, trial::TrialState::Running);
        self.trials.push(trial);
        id
    }
    pub fn get_trial(&self, trial_id: usize) -> frozen_trial::FrozenTrial {
        self.trials[trial_id]
    }
    pub fn get_best_trial(&self) -> Option<frozen_trial::FrozenTrial> {
        self.trials
            .iter()
            .cloned()
            .filter(|t| t.state == trial::TrialState::Completed && t.value.is_some())
            .min_by_key(|t| OrderedFloat(t.value.unwrap()))
    }
    pub fn set_trial_value(&mut self, trial_id: usize, value: f64) {
        self.trials[trial_id].value = Some(value);
    }
    pub fn set_trial_state(&mut self, trial_id: usize, state: trial::TrialState) {
        self.trials[trial_id].state = state;
    }
}
