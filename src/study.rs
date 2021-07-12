use super::frozen_trial::FrozenTrial;
use super::sampler::Sampler;
use super::storage::Storage;
use super::trial;

#[derive(Default)]
pub struct Study {
    pub storage: Storage,
    pub sampler: Sampler,
}

impl Study {
    pub fn new(storage: Storage, sampler: Sampler) -> Study {
        Study {
            storage: storage,
            sampler: sampler,
        }
    }
    pub fn optimize<F>(&mut self, objective: F, n_trials: u32)
    where
        F: Fn(&trial::Trial) -> f64,
    {
        for _n in 0..n_trials {
            let trial_id = self.storage.create_new_trial();
            let trial = trial::Trial::new(self, trial_id);
            let value = objective(&trial);
            self.storage.set_trial_value(trial_id, value);
            self.storage
                .set_trial_state(trial_id, trial::TrialState::Completed);
        }
    }
    pub fn best_trial(&self) -> Option<FrozenTrial> {
        self.storage.get_best_trial()
    }
}
