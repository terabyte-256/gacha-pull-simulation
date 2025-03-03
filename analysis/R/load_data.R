
load_game_data <- function() {
  # Read the simulation results from the CSV files
  results_hsr_char <- read.csv("../data/hsr/character.csv")
  results_hsr_weapon <- read.csv("../data/hsr/weapon.csv")
  results_genshin_char <- read.csv("../data/genshin/character.csv")
  results_genshin_weapon <- read.csv("../data/genshin/weapon.csv")
  results_zzz_char <- read.csv("../data/zzz/character.csv")
  results_zzz_weapon <- read.csv("../data/zzz/weapon.csv")
  results_arknights <- read.csv("../data/arknights/data.csv")
  results_wuwa <- read.csv("../data/wuwa/data.csv")

  # Standardize column names for honkai games
  colnames(results_hsr_char) <- c("Pulls", "Limited", "Weapons", "Four_Stars", "Three_Stars")
  colnames(results_hsr_weapon) <- c("Pulls", "Limited", "Weapons", "Four_Stars", "Three_Stars")
  colnames(results_genshin_char) <- c("Pulls", "Limited", "Weapons", "Four_Stars", "Three_Stars")
  colnames(results_genshin_weapon) <- c("Pulls", "Limited", "Weapons", "Four_Stars", "Three_Stars")
  colnames(results_zzz_char) <- c("Pulls", "Limited", "Weapons", "Four_Stars", "Three_Stars")
  colnames(results_zzz_weapon) <- c("Pulls", "Limited", "Weapons", "Four_Stars", "Three_Stars")
  
  # Standardize column names for Arknights 
  colnames(results_arknights) <- c("Pulls", "Six_Stars", "Five_Stars", "Four_Stars", "Three_Stars")
  
  # Standardize column names for Wuwa
  colnames(results_wuwa) <- c("Pulls", "Five_Stars", "Four_Stars", "Limited_Four_Stars", "Three_Stars")

  # Combine character and weapon data for Honkai games with banner type
  results_hsr <- bind_rows(
    mutate(results_hsr_char, Banner = "Character"),
    mutate(results_hsr_weapon, Banner = "Weapon")
  )

  results_genshin <- bind_rows(
    mutate(results_genshin_char, Banner = "Character"),
    mutate(results_genshin_weapon, Banner = "Weapon")
  )

  results_zzz <- bind_rows(
    mutate(results_zzz_char, Banner = "Character"),
    mutate(results_zzz_weapon, Banner = "Weapon")
  )

  # Add game labels
  results_hsr$Game <- "HSR"
  results_genshin$Game <- "Genshin"
  results_zzz$Game <- "ZZZ"
  results_arknights$Game <- "Arknights"
  results_wuwa$Game <- "Wuwa"

  # Return as a list
  list(
    hsr = results_hsr,
    genshin = results_genshin,
    zzz = results_zzz,
    arknights = results_arknights,
    wuwa = results_wuwa
  )
}

# Function to prepare combined data for all games
prepare_combined_data <- function(data_list) {
  # For Honkai games (HSR, Genshin, ZZZ)
  honkai_games <- bind_rows(
    data_list$hsr,
    data_list$genshin,
    data_list$zzz
  )
  
  # For Arknights, create comparable columns
  arknights <- data_list$arknights %>%
    mutate(
      Banner = "Overall",
      Limited = Six_Stars,  # Using Six_Stars as equivalent to Limited
      Weapons = 0  # No weapons in Arknights data
    )
  
  # For Wuwa, create comparable columns
  wuwa <- data_list$wuwa %>%
    mutate(
      Banner = "Overall",
      Limited = Five_Stars,  # Using Five_Stars as equivalent to Limited
      Limited_Four = Limited_Four_Stars,
      Weapons = 0  # No specific weapon data in Wuwa
    )
  
  # Combine all with common columns
  all_games <- bind_rows(
    honkai_games,
    arknights,
    wuwa
  )
  
  # Convert Game column to factor with specified levels
  all_games$Game <- factor(
    all_games$Game,
    levels = c("HSR", "Genshin", "ZZZ", "Arknights", "Wuwa")
  )
  
  all_games
}
