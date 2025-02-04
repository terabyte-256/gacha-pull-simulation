export class GachaCalculator {
    calculateProbability(n, x) {
        // Helper function to calculate the probability for a given pull
        function getProbability(pull) {
            if (pull <= 73) {
                return 0.006;
            } else if (pull >= 74 && pull <= 89) {
                return 0.006 + 0.062 * (pull - 73);
            } else if (pull === 90) {
                return 1;
            }
        }
    
        // Helper function to calculate the binomial probability
        function binomialProbability(k, n, p) {
            const factorial = (num) => {
                let result = 1;
                for (let i = 2; i <= num; i++) {
                    result *= i;
                }
                return result;
            };
    
            const nFact = factorial(n);
            const kFact = factorial(k);
            const nkFact = factorial(n - k);
    
            return (nFact) / (kFact * nkFact) * Math.pow(p, k) * Math.pow(1 - p, n - k);
        }
    
        // Initialize the probability to 0
        let totalProbability = 0;
    
        // Iterate through all possible sequences of pulls
        for (let i = 0; i <= n; i++) {
            for (let j = 0; j <= n - i; j++) {
                // Calculate the probability for each sequence
                let pullProbability = getProbability(i);
                let remainingPulls = n - i;
                let remainingProbability = 0.006; // Reset probability for remaining pulls
    
                for (let k = 0; k < remainingPulls; k++) {
                    remainingProbability *= getProbability(k + i + 1);
                }
    
                // Calculate the binomial probability for the remaining pulls
                let binomialProb = binomialProbability(x, remainingPulls, remainingProbability);
    
                // Add the probability to the total probability
                totalProbability += pullProbability * binomialProb;
            }
        }
    
        return totalProbability;
    }    
}
