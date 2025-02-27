pub mod Arknights {
    use rand::Rng;
    use rand::thread_rng;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref SIX_STAR_RATE: f64 = 0.02;
        static ref FIVE_STAR_RATE: f64 = 0.08;
        static ref FOUR_STAR_RATE: f64 = 0.5;
        static ref THREE_STAR_RATE: f64 = 0.4;
        static ref INCREMENT: f64 = 0.02;
        static ref SOFT_PITY: i32 = 50;
    }

    pub fn a_simulate_game(num_simulations: i32) -> Vec<(i32, i32, i32, i32, i32)> {
        let mut results: Vec<(i32, i32, i32, i32, i32)> = Vec::new();

        for _ in 0..num_simulations {
            let mut pull_count = 0;
            let mut current_pity = 0;
            let mut pulls: Vec<i32> = Vec::with_capacity(4);
            pulls.push(0); // 6-star
            pulls.push(0); // 5-star
            pulls.push(0); // 4-star
            pulls.push(0); // 3-star

            let mut current_rates: Vec<f64> = Vec::with_capacity(4);
            current_rates.push(*SIX_STAR_RATE);
            current_rates.push(*FIVE_STAR_RATE);
            current_rates.push(*FOUR_STAR_RATE);
            current_rates.push(*THREE_STAR_RATE);

            for _ in 0..10 {
                current_pity += 1;
                pull_count += 1;

                if current_pity > *SOFT_PITY {
                    let mut new_rates: Vec<f64> = Vec::with_capacity(4);
                    new_rates.push(current_rates[0] + *INCREMENT);
                    new_rates.push(current_rates[1] - *INCREMENT * *FIVE_STAR_RATE);
                    new_rates.push(current_rates[2] - *INCREMENT * *FOUR_STAR_RATE);
                    new_rates.push(current_rates[3] - *THREE_STAR_RATE * *INCREMENT);
                    current_rates = new_rates;
                }

                let roll: f64 = thread_rng().gen();

                if roll < current_rates[0] {
                    current_pity = 0;
                    let mut reset_rates: Vec<f64> = Vec::with_capacity(4);
                    reset_rates.push(*SIX_STAR_RATE);
                    reset_rates.push(*FIVE_STAR_RATE);
                    reset_rates.push(*FOUR_STAR_RATE);
                    reset_rates.push(*THREE_STAR_RATE);
                    current_rates = reset_rates;
                    pulls[0] += 1;
                } else if roll < current_rates[0] + current_rates[1] {
                    pulls[1] += 1;
                } else if roll < current_rates[0] + current_rates[1] + current_rates[2] {
                    pulls[2] += 1;
                } else {
                    pulls[3] += 1;
                }
            }

            results.push((pull_count, pulls[0], pulls[1], pulls[2], pulls[3]));
        }

        results
    }
}
