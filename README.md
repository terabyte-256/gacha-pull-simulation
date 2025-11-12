# Gacha Pull Simulation

A comprehensive toolkit for simulating and analyzing gacha pull statistics across popular mobile games. This project provides a high-performance Rust-based simulation engine, robust statistical analysis tools using R, and an interactive web calculator.

## Features

### Simulation Engine (Rust)
- Multi-threaded simulation capable of running millions of trials for statistical accuracy
- Memory-efficient chunk processing for handling large datasets
- Precise implementation of complex pity systems and banner mechanics

### Data Analysis (R)
- Detailed pull distribution visualizations
- Banner comparison analytics
- Cumulative probability calculations
- Success rate comparisons across different games

### Web Calculator
- Interactive probability calculations for HoYoverse games
- Configurable parameters (pity count, guarantee status, banner type)
- Real-time results without server dependencies

## Supported Games

- **HoYoverse Games**
  - Genshin Impact
  - Honkai: Star Rail (HSR)
  - Zenless Zone Zero (ZZZ)
- **Other Gacha Games**
  - Arknights
  - Wuthering Waves

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [R](https://cran.r-project.org/) with the following packages:
  - ggplot2, dplyr, knitr, kableExtra, tidyr
- Modern web browser for the calculator

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/terabyte-256/gacha-pull-simulation.git
   cd gacha-pull-simulation
   ```

2. Build the Rust simulation engine:
   ```bash
   cargo build --release
   ```

3. Install R dependencies:
   ```r
   install.packages(c("ggplot2", "dplyr", "knitr", "kableExtra", "tidyr", "viridis"))
   ```

## Usage

### Running Simulations

Run the Rust simulation to generate data files:

```bash
cargo run --release -- -n 1000000 -c 10000
```

Options:
- `-n, --simulations`: Number of simulations to run (default: 1000000)
- `-c, --chunk-size`: Size of chunks for processing (default: 10000)

The simulation will create data files in the `data/` directory for each game and banner type.

### Analyzing Results

Generate the analysis report:

```bash
cd analysis
Rscript -e 'rmarkdown::render("gacha_pull_simulation.rmd")'
```

This will create a PDF report with detailed visualizations and statistics.

### Using the Web Calculator

Simply open `web/index.html` in a web browser to use the interactive calculator:

```bash
# macOS
open web/index.html

# Linux
xdg-open web/index.html

# Windows
start web/index.html
```

## Game Mechanics

### HoYoverse Games

#### Genshin Impact
- **Character Banner**: 0.6% base rate, soft pity at 74, hard pity at 90
- **Weapon Banner**: 0.7% base rate, soft pity at 63, hard pity at 80
- 50/50 system for characters, 75/25 for weapons

#### Honkai: Star Rail
- **Character Banner**: 0.6% base rate, soft pity at 74, hard pity at 90
- **Light Cone Banner**: 0.8% base rate, soft pity at 64, hard pity at 80
- Similar guarantee systems to Genshin

#### Zenless Zone Zero
- **Character Banner**: 1.0% base rate, similar pity system
- **Weapon Banner**: Higher rates than other HoYoverse games

### Other Games

#### Arknights
- 2% base rate for 6★ operators
- Pity system that increases rates after 50 pulls
- 10-pull simulation with detailed rarity tracking

#### Wuthering Waves
- 0.8% base rate for 5★ characters
- 80-pull hard pity
- 50/50 guarantee system

## Contributing

Contributions are welcome! Here's how you can contribute:

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Commit your changes: `git commit -am 'Add some feature'`
4. Push to the branch: `git push origin feature-name`
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- HoYoverse, Hypergryph, Kuro Games, and other game developers for creating these games
- The gacha gaming community for documenting pity systems and pull mechanics
- All contributors to this project
