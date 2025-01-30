import numpy as np
import pandas as pd
from time import time

def batch_algo(batch_size):
    # Generate all random numbers at once for base rate (1-73)
    base_rolls = np.random.random(size=(batch_size, 73))
    base_hits = base_rolls < 0.006
    base_successes = np.argmax(base_hits, axis=1) + 1
    base_mask = np.any(base_hits, axis=1)
    
    # Handle non-base rate successes
    remaining = batch_size - np.sum(base_mask)
    if remaining > 0:
        pity_pulls = np.zeros(remaining, dtype=np.int32)
        
        # Generate soft pity rolls (74-89)
        for i in range(74, 90):
            curr_remaining = np.sum(pity_pulls == 0)
            if curr_remaining == 0:
                break
                
            pity_rate = 0.006 + (0.062 * (i - 73))
            rolls = np.random.random(curr_remaining)
            successes = rolls < pity_rate
            pity_pulls[pity_pulls == 0][successes] = i
        
        # Set remaining to hard pity (90)
        pity_pulls[pity_pulls == 0] = 90
        
        # Combine results
        results = np.zeros(batch_size, dtype=np.int32)
        results[base_mask] = base_successes[base_mask]
        results[~base_mask] = pity_pulls
        
        return results
    
    return base_successes

def main():
    num_sims = 10_000_000
    batch_size = 100_000
    results = []
    
    start_time = time()
    
    # Run simulations in batches
    for _ in range(num_sims // batch_size):
        batch_results = batch_algo(batch_size)
        results.append(batch_results)
    
    # Combine all results
    all_results = np.concatenate(results)
    
    # Calculate statistics
    values, counts = np.unique(all_results, return_counts=True)
    cumulative_counts = np.cumsum(counts)
    percentages = counts / num_sims * 100
    cumulative_percentages = cumulative_counts / num_sims * 100
    
    # Create DataFrame
    df = pd.DataFrame({
        'pulls': values,
        'frequency': counts,
        'percentage': np.round(percentages, 4),
        'cumulative_frequency': cumulative_counts,
        'cumulative_probability': np.round(cumulative_percentages, 4)
    })
    
    # Save to CSV
    df.to_csv('pull_data.csv', index=False)
    
    end_time = time()
    print(f"Simulation completed in {end_time - start_time:.2f} seconds")

if __name__ == "__main__":
    main()
