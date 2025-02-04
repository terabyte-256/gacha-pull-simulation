export class GachaCalculator {
    constructor() {
        this.pullData = [];
    }

    async loadData() {
        try {
            const response = await fetch('./data/pull_data.csv');
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

        if (isNaN(x) || isNaN(n) || x < 1 || n < 1) {
            return "Please enter valid numbers";
        }

        if (x > n) {
            return "X cannot be greater than N";
        }

        const cumulativeProbabilities = this.pullData.cumulative_probability;

        let probability = 0;

        for (let i = x; i <= n; i++) {
            probability += cumulativeProbabilities[i - 1];
        }

        return probability.toFixed(6);
    }
}
