export class GachaCalculator {
    // uses monte carlo
    
    static constants = {
        FIVE_STAR_CHARACTER_CHANCE: 0.006,
        FIVE_STAR_CONE_CHANCE: 0.008,
        
        CHARACTER_SOFT_PITY: 74,
        SOFT_PITY_INCREMENT: 0.062,
        CHARACTER_PITY: 90,
        
        CONE_SOFT_PITY: 64,
        CONE_PITY: 80,
        
        LIMITED_CHARACTER_CHANCE: 0.5,
        LIMITED_CONE_CHANCE: 0.75,

    };


    calculateProbability(pulls, existing_character_pity, existing_cone_pity, character_copies, cone_copies, character_guaranteed = false, cone_guaranteed = false, num_simulations) {
        
        let successful_sims = 0;
        
        for (let i = 0; i < num_simulations; i++) {
                let pulls_left = pulls;

                let current_character_pity = 0;
                let current_cone_pity = existing_cone_pity;
        
                let current_character_guaranteed = character_guaranteed;
                let current_cone_guaranteed = cone_guaranteed;
        
                let character_success = 0
                
                while (pulls_left > 0 && character_success < character_copies) {
                    // 5 star prob
                    let random_value = random.random();
                    let current_five_star_chance = this.constants.FIVE_STAR_CHARACTER_CHANCE;
        
                    if (cone_copies > 0 && character_success < character_copies) {
                    // increments if current pity number is 74
                    current_five_star_chance += this.constants.SOFT_PITY_INCREMENT * Math.max(current_character_pity - this.constants.CHARACTER_SOFT_PITY, 0.0);
        
                        if (random_value < current_five_star_chance || current_character_pity + 1 == this.constants.CHARACTER_PITY) {
                        
                            if (current_character_guaranteed || random.random() < this.constants.LIMITED_CHARACTER_CHANCE) {
                                character_success += 1;
                                current_character_guaranteed = false;
                                current_character_pity = 0;
                            } else {
                                current_character_pity = 0;
                                current_character_guaranteed = true;
                            }
                        } else {                                
                            current_character_pity += 1;
                        }
                    } else {
                        // Cone prob
                        current_five_star_chance = this.constants.FIVE_STAR_CONE_CHANCE;
                        current_five_star_chance += this.constants.SOFT_PITY_INCREMENT * Math.max(current_cone_pity - this.constants.CONE_SOFT_PITY, 0.0);
        
                        if (random_value < current_five_star_chance || current_cone_pity + 1 == this.constants.CONE_PITY) {
                            if (current_cone_guaranteed || random.random() < this.constants.LIMITED_CONE_CHANCE) {
                                cone_successes += 1;
                                current_cone_guaranteed = false;
                                current_cone_pity = 0;
                            } else {
                                current_cone_pity = 0;
                                current_cone_guaranteed = true;
                            }
                        } else {
                            current_cone_pity += 1;
                        }
                    }
        
                    pulls_left -= 1;
                
                }
        
                if (character_success >= character_copies && cone_successes >= cone_copies) {
                    successful_sims += 1;
                }
            }
            return (successful_sims / num_simulations).toFixed(6);
        }

        
}