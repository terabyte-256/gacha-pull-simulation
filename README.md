# gacha-pull

A statistical simulation of gacha game pull mechanics using R.

## Description

This project simulates the 5-star character pull system commonly found in gacha games, with specific focus on:
- Base rate of 0.6%
- Soft pity starting at pull 74
- Hard pity at pull 90

## Features

- Large-scale pull simulation (default 2M pulls)
- Statistical analysis including mean, median, and standard deviation
- Visualization of pull distribution
- Cumulative probability analysis
- Performance optimized batch processing

## Requirements

- R
- Required packages: ggplot2, knitr, dplyr

## Usage

Run the R Markdown file to generate a comprehensive report with:
- Pull distribution histogram
- Summary statistics
- Frequency tables
- Cumulative probability analysis

The simulation parameters can be modified in the `get_user_input()` function.