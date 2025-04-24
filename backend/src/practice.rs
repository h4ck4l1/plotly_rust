use backend::mushroom_pages::fitter;
use rand::{distr::StandardUniform, Rng};













fn main() -> Result<(),anyhow::Error> {
    


    let random_vec = rand::rng()
        .sample_iter(StandardUniform)
        .take(1000)
        .map(|n: f64| n * 10 as f64)
        .collect::<Vec<_>>();



    fitter(random_vec)?;

    Ok(())
}