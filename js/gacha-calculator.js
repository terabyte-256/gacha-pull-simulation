// Common values across all games
const COMMON_FIVE_STAR_CHARACTER_CHANCE = 0.006;
const COMMON_SOFT_PITY_INCREMENT = 0.062;
const COMMON_CHARACTER_SOFT_PITY = 74;
const COMMON_CHARACTER_PITY = 90;
const COMMON_WEAPON_SOFT_PITY = 64;
const COMMON_WEAPON_PITY = 80;

// Game-specific values
const gameData = {
    hsr: {
        FIVE_STAR_WEAPON_CHANCE: 0.008,
        LIMITED_CHARACTER_CHANCE: 0.5,
        LIMITED_WEAPON_CHANCE: 0.75 // leaving these in here in case they need to be changed
    },
    genshin: {
        FIVE_STAR_WEAPON_CHANCE: 0.007,
        LIMITED_CHARACTER_CHANCE: 0.55, // This is different because of the change to Hoyo's pity system
        LIMITED_WEAPON_CHANCE: 0.75 // leaving these in here in case they need to be changed
    },
    zzz: {
        FIVE_STAR_WEAPON_CHANCE: 0.01,
        LIMITED_CHARACTER_CHANCE: 0.5,
        LIMITED_WEAPON_CHANCE: 0.75 // leaving these in here in case they need to be changed
    },
};

// Export the helper function
export function getGameData(hoyoverseGame) {
    return gameData[hoyoverseGame.toLowerCase()] || {};
}

// Export the main calculation function
export function calculatePullProbability(
    pulls,
    characterPity,
    weaponPity,
    weaponGuaranteed,
    characterGuaranteed,
    characterCopies,
    weaponCopies,
    numSimulations,
    hoyoverseGame
) {
    let successfulSimulations = 0;
    const gameInfo = getGameData(hoyoverseGame);

    for (let i = 0; i < numSimulations; i++) {
        let pullsLeft = pulls;
        let charSuccesses = 0;
        let weaponSuccesses = 0;
        let currWeaponPity = weaponPity;
        let currCharPity = characterPity;
        let currWeaponGuaranteed = weaponGuaranteed;
        let currCharacterGuaranteed = characterGuaranteed;

        while (pullsLeft > 0) {
            let randomValue = Math.random();
            let currFiveStarChance = COMMON_FIVE_STAR_CHARACTER_CHANCE;
            let currFiveStarWeaponChance = gameInfo.FIVE_STAR_WEAPON_CHANCE;
            let currSoftPityIncrement = COMMON_SOFT_PITY_INCREMENT;
            let currCharacterSoftPity = COMMON_CHARACTER_SOFT_PITY;
            let currCharacterPity = COMMON_CHARACTER_PITY;
            let currWeaponSoftPity = COMMON_WEAPON_SOFT_PITY;
            let currWeaponPity = COMMON_WEAPON_PITY;
            let currLimitedWeaponChance = gameInfo.LIMITED_WEAPON_CHANCE;
            let currLimitedCharacterChance = gameInfo.LIMITED_CHARACTER_CHANCE;

            if (weaponCopies > 0 && charSuccesses < characterCopies) {
                currFiveStarChance += currSoftPityIncrement * Math.max(currCharPity - currCharacterSoftPity, 0);

                if (randomValue <= currFiveStarChance || currCharPity + 1 === currCharacterPity) {
                    if (currCharacterGuaranteed || Math.random() <= currLimitedCharacterChance) {
                        charSuccesses++;
                        currCharacterGuaranteed = false;
                        currCharPity = 0;
                    } else {
                        currCharPity = 0;
                        currCharacterGuaranteed = true;
                    }
                } else {
                    currCharPity++;
                }
            } else {
                currFiveStarChance = currFiveStarWeaponChance;
                currFiveStarChance += currSoftPityIncrement * Math.max(currWeaponPity - currWeaponSoftPity, 0);

                if (randomValue <= currFiveStarChance || currWeaponPity + 1 === currWeaponPity) {
                    if (currWeaponGuaranteed || Math.random() < currLimitedWeaponChance) {
                        weaponSuccesses++;
                        currWeaponGuaranteed = false;
                        currWeaponPity = 0;
                    } else {
                        currWeaponPity = 0;
                        currWeaponGuaranteed = true;
                    }
                } else {
                    currWeaponPity++;
                }
            }
            pullsLeft--;
        }

        if (charSuccesses >= characterCopies && weaponSuccesses >= weaponCopies) {
            successfulSimulations++;
        }
    }

    return successfulSimulations / numSimulations;
}