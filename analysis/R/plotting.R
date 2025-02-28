
# Define a consistent color scheme for the games
game_colors <- c(
  "HSR" = "#5B9BD5",
  "Genshin" = "#70AD47", 
  "ZZZ" = "#ED7D31",
  "Arknights" = "#FAC205",
  "Wuwa" = "#9A4EAE"
)

# Plot pull distributions
plot_pull_density <- function(all_results) {
  ggplot(all_results, aes(x = Pulls, fill = Game)) +
    geom_density(alpha = 0.5) +
    scale_fill_manual(values = game_colors) +
    theme_minimal() +
    labs(
      title = "Pull Distribution Comparison Across Games",
      x = "Number of Pulls",
      y = "Density"
    )
}

# Plot pull distribution boxplots
plot_pull_boxplots <- function(all_results) {
  ggplot(all_results, aes(x = Game, y = Pulls, fill = Game)) +
    geom_violin(alpha = 0.5) +
    geom_boxplot(width = 0.2, alpha = 0.8) +
    scale_fill_manual(values = game_colors) +
    theme_minimal() +
    labs(
      title = "Pull Distribution by Game",
      y = "Number of Pulls"
    )
}

# Plot success rates
plot_success_rates <- function(rates_data) {
  ggplot(rates_data, aes(x = Game, y = Rate, fill = Type)) +
    geom_bar(stat = "identity", position = "dodge") +
    scale_fill_brewer(palette = "Set2") +
    theme_minimal() +
    labs(
      title = "Success Rates by Game and Type",
      y = "Rate"
    ) +
    coord_flip()
}

# Plot cumulative probabilities
plot_cumulative_probabilities <- function(cum_data) {
  ggplot(cum_data, aes(x = Pulls, y = cumulative, color = Game)) +
    geom_line(linewidth = 1) +
    scale_color_manual(values = game_colors) +
    theme_minimal() +
    labs(
      title = "Cumulative Probability of Success",
      x = "Number of Pulls",
      y = "Cumulative Probability"
    ) +
    geom_hline(yintercept = 0.5, linetype = "dashed", color = "gray") +
    geom_hline(yintercept = 0.95, linetype = "dashed", color = "gray")
}

# Plot specific game histogram
plot_game_histogram <- function(data, game_name, fill_color) {
  ggplot(data, aes(x = Pulls)) +
    geom_histogram(binwidth = 1, fill = fill_color, color = "black") +
    theme_minimal() +
    labs(
      title = paste(game_name, "Pull Distribution"),
      x = "Number of Pulls",
      y = "Frequency"
    )
}

# Plot banner comparison for Honkai games
plot_banner_comparison <- function(data) {
  ggplot(data, aes(x = Banner, y = Pulls, fill = Banner)) +
    geom_violin(alpha = 0.7) +
    geom_boxplot(width = 0.1, alpha = 0.6) +
    facet_wrap(~ Game) +
    scale_fill_brewer(palette = "Set1") +
    theme_minimal() +
    labs(
      title = "Character vs Weapon Banner Comparison",
      y = "Number of Pulls"
    )
}

# Plot rarity distribution
plot_rarity_distribution <- function(data_list) {
  # For Honkai games - pull distribution by banner
  honkai_data <- bind_rows(
    data_list$hsr,
    data_list$genshin,
    data_list$zzz
  ) %>%
    group_by(Game, Banner) %>%
    summarise(
      Avg_Pulls = mean(Pulls),
      Limited_Rate = mean(Limited) / mean(Pulls),
      Four_Star_Rate = mean(Four_Stars) / mean(Pulls),
      Three_Star_Rate = mean(Three_Stars) / mean(Pulls),
      .groups = 'drop'
    ) %>%
    pivot_longer(
      cols = c(Limited_Rate, Four_Star_Rate, Three_Star_Rate),
      names_to = "Rarity",
      values_to = "Rate"
    )
  
  # For Arknights
  arknights_data <- data_list$arknights %>%
    summarise(
      Game = "Arknights",
      Banner = "Overall",
      Six_Star_Rate = mean(Six_Stars) / mean(Pulls),
      Five_Star_Rate = mean(Five_Stars) / mean(Pulls),
      Four_Star_Rate = mean(Four_Stars) / mean(Pulls),
      Three_Star_Rate = mean(Three_Stars) / mean(Pulls)
    ) %>%
    pivot
}