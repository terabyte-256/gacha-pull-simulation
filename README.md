# Gacha Pull Simulation

This project simulates and analyzes gacha pull statistics for various games including HoYoverse games (Genshin Impact, Honkai: Star Rail, and Zenless Zone Zero), arknights, and Wuthering Waves. It provides accurate probability modeling, visualization tools, and interactive components to help players understand the mechanics behind gacha systems.

## Project Structure

```
gacha-pull-simulation/
├── analysis/
│   ├── gacha_pull_simulation.pdf
│   └── gacha_pull_simulation.rmd
├── src/
│   ├── main.rs
│   ├── honkai.rs
│   ├── arknights.rs
│   └── wuthering_waves.rs
├── web/
│   ├── index.html
│   └── js/
│       └── gacha-calculator.js
├── data/
│   ├── hsr/
│   │   ├── character.csv
│   │   └── weapon.csv
│   ├── genshin/
│   │   ├── character.csv
│   │   └── weapon.csv
│   ├── zzz/
│   │   ├── character.csv
│   │   └── weapon.csv
│   ├── arknights/
│   │   └── data.csv
│   └── wuwa/
│       └── data.csv
```

## Prerequisites

- Rust (latest stable version)
- R with the following packages:
  - ggplot2
  - knitr
  - dplyr
  - kableExtra
  - hexbin
  - tidyverse
- A modern web browser
- Node.js (optional, for development)

## Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/gacha-pull-simulation.git
   cd gacha-pull-simulation
   ```

2. Install Rust dependencies:
   ```bash
   cargo build --release
   ```

3. Install R dependencies:
   ```r
   install.packages(c("ggplot2", "knitr", "dplyr", "kableExtra", "hexbin", "tidyverse"))
   ```

## Running the Simulation

1. Generate the simulation data:
   ```bash
   cargo run --release
   ```
   This will create CSV files in the `data` directory containing pull statistics for each game.

2. Generate the analysis report:
   ```bash
   Rscript -e 'rmarkdown::render("analysis/gacha_pull_simulation.rmd")'
   ```
   This will create both PDF and HTML versions of the analysis report.

3. For interactive calculations, open `index.html` in a web browser:
   ```bash
   open index.html  # macOS
   # OR
   xdg-open index.html  # Linux
   # OR
   start index.html  # Windows
   ```

## Features

- **Simulation Engine (Rust)**
  - Multi-threaded simulation with 10+ million trials for statistical accuracy
  - Configurable game parameters to match current in-game rates
  - Detailed logging of pull sequences and outcomes

- **Comprehensive Analysis (R Markdown)**
  - Pull distribution visualization with density plots
  - Success rate comparisons across different banner types
  - Cumulative probability analysis with confidence intervals
  - Character vs weapon banner analysis
  - Economic analysis (primogem/pull currency efficiency)

- **Interactive Web Calculator**
  - Real-time probability calculations
  - Support for all three HoYoverse games
  - Configurable parameters (pity count, guarantee status, etc.)
  - Visual representations of probability distributions

## Game Parameters

### Honkai: Star Rail
- **Standard Character Banner**
  - Base 5★ rate: 0.6%
  - Soft pity: Begins at 74 pulls
  - Hard pity: 90 pulls
  - 50/50 system for featured characters

- **Light Cone Banner**
  - Base 5★ rate: 0.8%
  - Soft pity: Begins at 65 pulls
  - Hard pity: 80 pulls
  - 75/25 system for featured light cones

### Genshin Impact
- **Character Event Banner**
  - Base 5★ rate: 0.6%
  - Soft pity: Begins at 74 pulls
  - Hard pity: 90 pulls
  - 50/50 system for featured characters

- **Weapon Banner**
  - Base 5★ rate: 0.7%
  - Soft pity: Begins at 63 pulls
  - Hard pity: 77 pulls
  - Epitomized Path system after 2 non-featured weapons

### Zenless Zone Zero
- **Agent Banner**
  - Base 5★ rate: 1.0%
  - Soft pity: Begins at 70 pulls
  - Hard pity: 80 pulls
  - 50/50 system for featured agents

- **Drive Banner**
  - Base 5★ rate: 1.0%
  - Soft pity: Begins at 60 pulls
  - Hard pity: 70 pulls
  - 75/25 system for featured drives

### arknights
- **Standard Banner**
  - Base 6★ rate: 2.0%
  - Pity System: Rate increases by 2% after 50 pulls
  - Rate increases: Starts at pull 51 where rate becomes 4%, then 6%, etc.
  - Recruitment and Headhunting systems
  - Tag-based recruitment system with rare tag combinations

- **Limited Banner**
  - Similar base rates with featured operators
  - Spark system at 300 pulls
  - Special banners (like Joint Operation) with modified pools

### Wuthering Waves
- **Character Banner**
  - Base 5★ rate: 0.8%
  - Hard pity: 80 pulls
  - 50/50 system for featured characters with guaranteed on second 5★

- **Four-Star System**
  - 6% base rate for 4★ characters/weapons
  - Guaranteed 4★ every 10 pulls
  - 50/50 chance for featured 4★ with guaranteed system

## Example Usage

For API usage in JavaScript:

```javascript
import { calculateProbability } from './js/gacha-calculator.js';

// For HoYoverse games
const hsrProbability = calculateProbability({
  game: 'hsr',
  bannerType: 'character',
  currentPity: 65,
  hasGuarantee: false,
  targetCount: 1
});
```
*Note: arknights and Wuthering Waves calculators have not been implemented yet.*
## Future Plans

- Create a Discord bot for quick probability lookups
- Develop a pull planning and resource management tool

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Devs for creating these games
- The gaming community for documenting gacha mechanics
- All contributors to this project
