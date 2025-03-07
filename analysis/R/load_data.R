
load_game_data <- function() {
    # Get the repository root directory
    # If we're in analysis/, we need to go up one level to reach the repo root
    # If we're in analysis/R/, we need to go up two levels to reach the repo root

    current_dir <- getwd()

    # Determine the correct path to the data directory
    if (endsWith(current_dir, "/analysis/R") || endsWith(current_dir, "\\analysis\\R")) {
        data_path <- "../../data" # From analysis/R/ to repo root, then to data/
    } else if (endsWith(current_dir, "/analysis") || endsWith(current_dir, "\\analysis")) {
        data_path <- "../data" # From analysis/ to repo root, then to data/
    } else {
        # Fallback - this assumes we're running from the repository root
        data_path <- "data"

        # Print current directory to help debug
        cat("Current working directory:", current_dir, "\n")
        cat("Using data path:", data_path, "\n")
    }

    # For debugging - list files to confirm path is correct
    if (dir.exists(data_path)) {
        cat("Data directory exists. Contents:\n")
        print(list.files(data_path))
    } else {
        cat("Warning: Data directory not found at", data_path, "\n")
    }

    # Try alternate path if first attempt fails
    if (!dir.exists(data_path)) {
        alternate_paths <- c(
        "./data",
        "../data",
        "../../data",
        "../../../data"
        )

        for (alt_path in alternate_paths) {
        if (dir.exists(alt_path)) {
            cat("Found data directory at alternative path:", alt_path, "\n")
            data_path <- alt_path
            break
        }
        }
    }

    # Read the simulation results from the CSV files with try-catch for error handling
    tryCatch({
        results_hsr_char <- read.csv(file.path(data_path, "hsr/character.csv"))
        results_hsr_weapon <- read.csv(file.path(data_path, "hsr/weapon.csv"))
        results_genshin_char <- read.csv(file.path(data_path, "genshin/character.csv"))
        results_genshin_weapon <- read.csv(file.path(data_path, "genshin/weapon.csv"))
        results_zzz_char <- read.csv(file.path(data_path, "zzz/character.csv"))
        results_zzz_weapon <- read.csv(file.path(data_path, "zzz/weapon.csv"))
        results_arknights <- read.csv(file.path(data_path, "arknights/data.csv"))
        results_wuwa <- read.csv(file.path(data_path, "wuwa/data.csv"))
    }, error = function(e) {
        cat("Error reading data files:", e$message, "\n")
        stop("Failed to read data files. Please check the paths and file existence.")
    })
    # Standardize column names for Arknights
    colnames(results_arknights) <- c("Pulls", "Six_Stars", "Five_Stars", "Four_Stars", "Three_Stars")

    # Standardize column names for Wuwa
    colnames(results_wuwa) <- c("Pulls", "Five_Stars", "Four_Stars", "Limited_Four_Stars", "Three_Stars")

    # Combine character and weapon data for HoYo games with banner type
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
    # For HoYo games (HSR, Genshin, ZZZ)
    hoyo_games <- bind_rows(
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
