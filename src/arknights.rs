pub mod arknights {
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
            let mut pulls = [0, 0, 0, 0]; // [6-star, 5-star, 4-star, 3-star]
            let mut six_star_obtained = false;

            let mut current_rates = [*SIX_STAR_RATE, *FIVE_STAR_RATE, *FOUR_STAR_RATE, *THREE_STAR_RATE];

            // Pull until we get at least one 6-star
            while !six_star_obtained {
                current_pity += 1;
                pull_count += 1;

                // Apply soft pity mechanism
                if current_pity > *SOFT_PITY {
                    // Calculate new 6-star rate
                    let new_six_star_rate = current_rates[0] + *INCREMENT;

                    // Calculate how much to reduce from other rates proportionally
                    let reduction = new_six_star_rate - current_rates[0];
                    let total_other_rates = current_rates[1] + current_rates[2] + current_rates[3];

                    // Apply proportional reductions to maintain sum = 1.0
                    current_rates[0] = new_six_star_rate;
                    current_rates[1] -= reduction * (current_rates[1] / total_other_rates);
                    current_rates[2] -= reduction * (current_rates[2] / total_other_rates);
                    current_rates[3] -= reduction * (current_rates[3] / total_other_rates);
                }

                // Generate random roll
                let roll: f64 = thread_rng().gen::<f64>();

                // Determine result based on cumulative probability
                if roll < current_rates[0] {
                    // 6-star
                    current_pity = 0;
                    pulls[0] += 1;
                    six_star_obtained = true; // We got a 6-star, so we can stop pulling

                    // Reset rates to initial values
                    current_rates[0] = *SIX_STAR_RATE;
                    current_rates[1] = *FIVE_STAR_RATE;
                    current_rates[2] = *FOUR_STAR_RATE;
                    current_rates[3] = *THREE_STAR_RATE;
                } else if roll < current_rates[0] + current_rates[1] {
                    // 5-star
                    pulls[1] += 1;
                } else if roll < current_rates[0] + current_rates[1] + current_rates[2] {
                    // 4-star
                    pulls[2] += 1;
                } else {
                    // 3-star
                    pulls[3] += 1;
                }
            }

            // Add results for this simulation (how many pulls it took to get a 6-star)
            results.push((pull_count, pulls[0], pulls[1], pulls[2], pulls[3]));
        }

        results
    }
}
