# Gacha Pull Simulation

## Overview

This project simulates gacha pulls, a common mechanic in many mobile games, to estimate probabilities and resource requirements. It uses Monte Carlo simulations to model the pulling process for both characters and light cones. This project includes both an R Markdown file (`gacha_pull_simulation.rmd`) for generating a detailed report and a website (`index.html`) for a quick interactive overview of simulation results.

## Features

- **Monte Carlo Simulation:** Simulates thousands of gacha pulls to estimate probabilities.
- **Character and Light Cone Support:** Models pulling for both characters and light cones with separate pity systems.
- **Customizable Parameters:** Allows adjusting simulation parameters such as base chances, soft pity increments, and limited banner probabilities.
- **Statistical Analysis:** Provides summary statistics including average pulls, median pulls, standard deviation, and cumulative probabilities.
- **Visualizations:** Generates histograms and cumulative probability plots to visualize the simulation results.

## Technologies Used

- **R:** Used for the simulation logic, statistical analysis, and generating visualizations.
- **ggplot2:** R library for creating plots and graphs.
- **knitr:** R package for dynamic report generation.
- **dplyr:** R package for data manipulation.
- **JavaScript:** Used for any interactive front-end components (if applicable).

## Setup

### Prerequisites

- R (>= 4.0)
- RStudio (optional but recommended)
- Required R packages: ggplot2, knitr, dplyr

### Installation

1.  Clone the repository:

    ```bash
    git clone [repository_url]
    cd gacha-pull-simulation
    ```

2.  Install the required R packages:

    ```R
    install.packages(c("ggplot2", "knitr", "dplyr"))
    ```

## Usage

1.  Open `gacha_pull_simulation.rmd` in RStudio.

2.  Modify the simulation parameters in the `constants` chunk if needed.

3.  Run all chunks in the Rmd file to execute the simulation and generate the report.

4.  The report will display summary statistics, distribution plots, and cumulative probability plots.

5.  For an interactive, browser-based view of simulation results, open `index.html` in any web browser.

## Project Structure
```
gacha-pull-simulation/
├── gacha_pull_simulation.rmd
├── js/
│   └── gacha-calculator.js
├── index.html
├── .gitignore
└── README.md
```
