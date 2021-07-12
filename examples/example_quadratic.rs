use rustuna::Study;
use rustuna::Trial;

fn objective(trial: &Trial) -> f64 {
    let x = trial.suggest_uniform("x", 0.0, 10.0);
    let y = trial.suggest_uniform("y", 0.0, 10.0);
    return (x - 3.0).powi(2) + (y - 5.0).powi(2);
}

fn main() {
    let mut study = Study::default();
    study.optimize(objective, 10);
    let best_trial = study.best_trial();
    println!("{:?}", best_trial);
}
