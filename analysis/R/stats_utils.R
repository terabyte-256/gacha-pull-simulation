# Function to calculate basic statistics for each game
calculate_basic_stats <- function(data) {
  data.frame(
    total_sims = nrow(data),
    pulls_mean = mean(data$Pulls),
    pulls_median = median(data$Pulls),
    pulls_sd = sd(data$Pulls),
    pulls_min = min(data$Pulls),
    pulls_max = max(data$Pulls)
  )
}

# Function to calculate success rates based on game type
calculate_success_rates <- function(data_list) {
  # Success rates for Hoyo games
  hsr_rates <- data_list$hsr %>%
    summarize(
      Game = "HSR",
      Limited_Rate = mean(Limited),
      Weapon_Rate = mean(Weapons),
      Four_Star_Rate = mean(Four_Stars) / mean(Pulls),
      Three_Star_Rate = mean(Three_Stars) / mean(Pulls)
    )

  genshin_rates <- data_list$genshin %>%
    summarize(
      Game = "Genshin",
      Limited_Rate = mean(Limited),
      Weapon_Rate = mean(Weapons),
      Four_Star_Rate = mean(Four_Stars) / mean(Pulls),
      Three_Star_Rate = mean(Three_Stars) / mean(Pulls)
    )

  zzz_rates <- data_list$zzz %>%
    summarize(
      Game = "ZZZ",
      Limited_Rate = mean(Limited),
      Weapon_Rate = mean(Weapons),
      Four_Star_Rate = mean(Four_Stars) / mean(Pulls),
      Three_Star_Rate = mean(Three_Stars) / mean(Pulls)
    )

  # Success rates for Arknights
  arknights_rates <- data_list$arknights %>%
    summarize(
      Game = "Arknights",
      Six_Star_Rate = mean(Six_Stars) / mean(Pulls),
      Five_Star_Rate = mean(Five_Stars) / mean(Pulls),
      Four_Star_Rate = mean(Four_Stars) / mean(Pulls),
      Three_Star_Rate = mean(Three_Stars) / mean(Pulls)
    )

  # Success rates for Wuwa
  wuwa_rates <- data_list$wuwa %>%
    summarize(
      Game = "Wuwa",
      Five_Star_Rate = mean(Five_Stars) / mean(Pulls),
      Four_Star_Rate = mean(Four_Stars) / mean(Pulls),
      Limited_Four_Rate = mean(Limited_Four_Stars) / mean(Pulls),
      Three_Star_Rate = mean(Three_Stars) / mean(Pulls)
    )

  # Return as a list
  list(
    hoyo = bind_rows(hsr_rates, genshin_rates, zzz_rates),
    arknights = arknights_rates,
    wuwa = wuwa_rates
  )
}

# Function to calculate cumulative probabilities
calc_cumulative <- function(data) {
  data %>%
    group_by(Pulls) %>%
    summarise(count = n(), .groups = 'drop') %>%
    arrange(Pulls) %>%
    mutate(
      probability = count / sum(count),
      cumulative = cumsum(count) / sum(count)
    )
}

# Prepare cumulative probability data for all games
prepare_cumulative_data <- function(data_list) {
  # Calculate cumulative probabilities for each game
  cum_hsr <- calc_cumulative(data_list$hsr) %>% mutate(Game = "HSR")
  cum_genshin <- calc_cumulative(data_list$genshin) %>% mutate(Game = "Genshin")
  cum_zzz <- calc_cumulative(data_list$zzz) %>% mutate(Game = "ZZZ")
  cum_arknights <- calc_cumulative(data_list$arknights) %>% mutate(Game = "Arknights")
  cum_wuwa <- calc_cumulative(data_list$wuwa) %>% mutate(Game = "Wuwa")

  # Combine all
  bind_rows(
    cum_hsr,
    cum_genshin,
    cum_zzz,
    cum_arknights,
    cum_wuwa
  )
}

# Calculate detailed statistics for a specific game
calculate_detailed_stats <- function(data) {
  # Identify columns present in the data
  cols <- colnames(data)

  # Common base grouping and calculations
  result <- data %>%
    group_by(Pulls) %>%
    summarise(
      Count = n(),
      Probability = Count/nrow(data),
      Cumulative_Prob = cumsum(Count)/nrow(data),
      .groups = 'drop'
    )

  # Add game-specific columns
  if ("Limited" %in% cols) {
    result <- result %>%
      left_join(
        data %>%
          group_by(Pulls) %>%
          summarise(Limited_Rate = mean(Limited), .groups = 'drop'),
        by = "Pulls"
      )
  }

  if ("Weapons" %in% cols) {
    result <- result %>%
      left_join(
        data %>%
          group_by(Pulls) %>%
          summarise(Weapon_Rate = mean(Weapons), .groups = 'drop'),
        by = "Pulls"
      )
  }

  if ("Six_Stars" %in% cols) {
    result <- result %>%
      left_join(
        data %>%
          group_by(Pulls) %>%
          summarise(Six_Star_Rate = mean(Six_Stars), .groups = 'drop'),
        by = "Pulls"
      )
  }

  if ("Five_Stars" %in% cols) {
    result <- result %>%
      left_join(
        data %>%
          group_by(Pulls) %>%
          summarise(Five_Star_Rate = mean(Five_Stars), .groups = 'drop'),
        by = "Pulls"
      )
  }

  if ("Limited_Four_Stars" %in% cols) {
    result <- result %>%
      left_join(
        data %>%
          group_by(Pulls) %>%
          summarise(Limited_Four_Star_Rate = mean(Limited_Four_Stars), .groups = 'drop'),
        by = "Pulls"
      )
  }

  if ("Four_Stars" %in% cols) {
    result <- result %>%
      left_join(
        data %>%
          group_by(Pulls) %>%
          summarise(Four_Star_Rate = mean(Four_Stars), .groups = 'drop'),
        by = "Pulls"
      )
  }

  if ("Three_Stars" %in% cols) {
    result <- result %>%
      left_join(
        data %>%
          group_by(Pulls) %>%
          summarise(Three_Star_Rate = mean(Three_Stars), .groups = 'drop'),
        by = "Pulls"
      )
  }

  result
}
