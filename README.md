# Gacha Pull Simulation

This project simulates and analyzes gacha pull statistics for various HoYoverse games (Genshin Impact, Honkai: Star Rail, and Zenless Zone Zero). It includes both a data generation component and analysis tools.

## Project Structure

```
gacha-pull-simulation/
├── src/
│   └── main.rs         # Rust simulation script
├── js/
│   └── gacha-calculator.js  # JavaScript calculator
├── gacha_pull_simulation.rmd  # R Markdown analysis
├── index.html          # Web interface
└── data/              # Generated CSV files
    ├── hsr.csv
    ├── genshin.csv
    └── zzz.csv
```

## Prerequisites

- Rust (latest stable version)
- R with the following packages:
  - ggplot2
  - knitr
  - dplyr
  - kableExtra
  - hexbin
- A modern web browser

## Running the Simulation

1. Generate the simulation data:
   ```bash
   cargo run --release
   ```
   This will create CSV files in the `data` directory containing pull statistics for each game.

2. Generate the analysis report:
   ```r
   rmarkdown::render("gacha_pull_simulation.rmd")
   ```
   This will create both PDF and HTML versions of the analysis report.

3. For interactive calculations, open `index.html` in a web browser.

## Features

- **Data Generation (Rust)**
  - Multi-threaded simulation for better performance
  - Configurable game parameters
  - Outputs CSV files with detailed pull statistics

- **Analysis (R Markdown)**
  - Pull distribution visualization
  - Success rate comparisons
  - Cumulative probability analysis
  - Character vs weapon analysis
  - Detailed statistical breakdowns

- **Web Calculator (JavaScript)**
  - Interactive probability calculator
  - Supports all three games
  - Configurable parameters including pity and guaranteed status

## Game Parameters

The simulation includes different parameters for each game:

- **Honkai: Star Rail**
  - 5★ Weapon chance: 0.8%
  - Limited character chance: 50%
  - Limited weapon chance: 75%

- **Genshin Impact**
  - 5★ Weapon chance: 0.7%
  - Limited character chance: 55%
  - Limited weapon chance: 75%

- **Zenless Zone Zero**
  - 5★ Weapon chance: 1.0%
  - Limited character chance: 50%
  - Limited weapon chance: 75%

## Contributing

Feel free to submit issues or pull requests if you find bugs or have suggestions for improvements.

## License

MIT License - Feel free to use this code for your own projects.
