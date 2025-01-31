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

        if (isNaN(x) || isNaN(n) || x < 1 || n < 1 || x > 90 || n > 90) {
            return "Please enter valid numbers (1-90)";
        }

        n = n - 1;

        if(x > n) {
            return "X cannot be greater than N";
        }

        let total5stars = 10000000;
        let totalPulls = 0;

        for (let i = 0; i <= 89; i++) {
            let currentPullData = this.pullData[i];
              
            if (!currentPullData) continue;  
            
            totalPulls += (currentPullData.frequency * (i + 1.0));

            }

        let successRate = totalPulls / total5stars;
        
        let totalProb = x / (n * successRate);
        
        return totalProb.toFixed(6);
    }
}
