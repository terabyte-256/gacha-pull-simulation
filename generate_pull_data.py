import numpy as np
import pandas as pd
from time import time

def get_rate(n):
    """Calculate pull probability for pull number n"""
    if n < 74:
        return 0.006  # Base rate 0.6%
    elif n == 90:
        return 1.0    # Hard pity
    else:
        # Soft pity calculation (74-89)
        increased_rate = 0.006 + (0.062 * (n - 73))
        return min(1.0, increased_rate)

def batch_algo(batch_size):
    results = np.zeros(batch_size, dtype=np.int32)
    
    for i in range(batch_size):
        # Check each pull sequentially
        for pull in range(1, 91):
            rate = get_rate(pull)
            if np.random.random() < rate:
                results[i] = pull
                break
                
        # Guarantee pull 90 if nothing hit
        if results[i] == 0:
            results[i] = 90
            
    return results

def main():
    num_sims = 10_000_000
    batch_size = 100_000
    results = []
    
    start_time = time()
    print("Starting simulation...")
    
    # Run simulations in batches
    for batch in range(num_sims // batch_size):
        if batch % 10 == 0:
            print(f"Processing batch {batch}/{num_sims // batch_size}")
        batch_results = batch_algo(batch_size)
        results.append(batch_results)
    
    # Combine all results
    all_results = np.concatenate(results)
    
    # Calculate statistics for all pulls 1-90
    pulls_range = np.arange(1, 91)
    counts = np.bincount(all_results, minlength=91)[1:]
    
    # Calculate cumulative statistics
    cumulative_counts = np.cumsum(counts)
    percentages = counts / num_sims * 100
    cumulative_percentages = cumulative_counts / num_sims * 100
    
    # Create DataFrame
    df = pd.DataFrame({
        'pulls': pulls_range,
        'frequency': counts,
        'percentage': np.round(percentages, 4),
        'cumulative_frequency': cumulative_counts,
        'cumulative_probability': np.round(cumulative_percentages, 4)
    })
    
    print("\nVerifying pull distribution...")
    print(f"Total pulls at 90 (should be rare): {counts[89]}")
    print(f"Percentage at pull 90: {percentages[89]:.4f}%")
    
    # Save to CSV
    df.to_csv('pull_data.csv', index=False)
    
    end_time = time()
    print(f"\nSimulation completed in {end_time - start_time:.2f} seconds")

if __name__ == "__main__":
    main()
