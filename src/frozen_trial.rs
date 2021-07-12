use super::trial;

#[derive(Clone, Copy, Debug)]
pub struct FrozenTrial {
    pub id: usize,
    pub state: trial::TrialState,
    pub value: Option<f64>,
}

impl FrozenTrial {
    pub fn new(id: usize, state: trial::TrialState) -> FrozenTrial {
        FrozenTrial {
            id: id,
            state: state,
            value: None,
        }
    }
}
