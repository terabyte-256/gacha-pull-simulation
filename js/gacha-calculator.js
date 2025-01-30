export class GachaCalculator {
    constructor() {
        this.pullData = [];
    }

    async loadData() {
        try {
            const response = await fetch('pull_data.csv');
            const csvText = await response.text();
            const lines = csvText.split('\n')
                .filter(line => line.trim() !== '')
                .slice(1);

            this.pullData = lines.map(line => {
                const [pulls, freq, percent, cumFreq, cumProb] = line.split(',');
                return {
                    pulls: parseInt(pulls),
                    frequency: parseInt(freq),
                    percentage: parseFloat(percent),
                    cumFrequency: parseInt(cumFreq),
                    cumProbability: parseFloat(cumProb)
                };
            });

            return true;
        } catch (error) {
            console.error('Error loading data:', error);
            return false;
        }
    }

    calculate(x, n) {
        if (!this.pullData.length) {
            return "Data not loaded";
        }

        if (isNaN(x) || isNaN(n) || x < 1 || n < 1 || x > 90 || n > 90) {
            return "Please enter valid numbers (1-90)";
        }

        n = n - 1;

        if(x > n) {
            return "X cannot be greater than N";
        }

        // Calculate cumulative probability using dynamic rates
        let totalProb = 0;
        let successRate = 0;
        
        // Use binomial probability with pity system
        for (let i = x; i <= n; i++) {
            let currentPullData = this.pullData[i];
            if (!currentPullData) continue;

            // Get success rate for current pull count
            successRate = currentPullData.frequency / (currentPullData.cumProbability * currentPullData.pulls);
            
            // Calculate probability for exactly x successes in i pulls
            let prob = this.binomialProbability(i, x, successRate);
            totalProb += prob;
        }

        return totalProb.toFixed(6);
    }

    binomialProbability(n, k, p) {
        return (
            this.combinations(n, k) * 
            Math.pow(p, k) * 
            Math.pow(1 - p, n - k)
        );
    }

    combinations(n, k) {
        let coeff = 1;
        for (let i = n - k + 1; i <= n; i++) {
            coeff *= i;
        }
        for (let i = 1; i <= k; i++) {
            coeff /= i;
        }
        return coeff;
    }
}
