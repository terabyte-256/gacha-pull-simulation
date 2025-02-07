export class GachaCalculator {
    static constants = Object.freeze({
        FIVE_STAR_CHARACTER_CHANCE: 0.006,
        FIVE_STAR_CONE_CHANCE: 0.008,
        CHARACTER_SOFT_PITY: 74,
        SOFT_PITY_INCREMENT: 0.062,
        CHARACTER_PITY: 90,
        CONE_SOFT_PITY: 64,
        CONE_PITY: 80,
        LIMITED_CHARACTER_CHANCE: 0.5,
        LIMITED_CONE_CHANCE: 0.75,
    });

    calculateProbability(
        pulls, 
        existing_character_pity = 0, 
        existing_cone_pity = 0, 
        character_copies = 0, 
        cone_copies = 0, 
        character_guaranteed = false, 
        cone_guaranteed = false, 
        num_simulations
        ) {
        
        let successful_sims = 0;
        
        for (let i = 0; i < num_simulations; i++) {
                let pulls_left = pulls;

                let current_character_pity = 0;
                let current_cone_pity = 0;
                
                current_character_pity = existing_character_pity;
                current_cone_pity = existing_cone_pity;
        
                let current_character_guaranteed = character_guaranteed;
                let current_cone_guaranteed = cone_guaranteed;
        
                let character_success = 0;
                let cone_successes = 0;
                
                while (pulls_left > 0 && ((character_success < character_copies) || (character_success == character_copies && character_copies == 0))) {
                    // 5 star prob
                    let random_value = Math.random();
                    let current_five_star_chance = GachaCalculator.constants.FIVE_STAR_CHARACTER_CHANCE;
        
                    if (cone_copies > 0 && character_success < character_copies) {
                    // increments if current pity number is 74
                    current_five_star_chance += GachaCalculator.constants.SOFT_PITY_INCREMENT * Math.max(current_character_pity - GachaCalculator.constants.CHARACTER_SOFT_PITY, 0.0);
        
                        if (random_value < current_five_star_chance || current_character_pity + 1 == GachaCalculator.constants.CHARACTER_PITY) {
                        
                            if (current_character_guaranteed || Math.random() < GachaCalculator.constants.LIMITED_CHARACTER_CHANCE) {
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
                        current_five_star_chance = GachaCalculator.constants.FIVE_STAR_CONE_CHANCE;
                        current_five_star_chance += GachaCalculator.constants.SOFT_PITY_INCREMENT * Math.max(current_cone_pity - GachaCalculator.constants.CONE_SOFT_PITY, 0.0);
        
                        if (random_value < current_five_star_chance || current_cone_pity + 1 == GachaCalculator.constants.CONE_PITY) {
                            if (current_cone_guaranteed || Math.random() < GachaCalculator.constants.LIMITED_CONE_CHANCE) {
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