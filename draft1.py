import numpy as np
import time

def algo():
    # Use np.float32 for memory efficiency
    for i in range(1, 74):
        if np.random.random_sample() < 0.006:
            return i
    
    for i in range(74, 90): # Soft pity
        if np.random.random_sample() < (0.006 + (0.062 * i)):
            return i
    
    return 90 # Hard pity

def pullSystem(iteration):
    # Process in batches of 1000 for memory efficiency
    BATCH_SIZE = 1000
    for i in range(0, iteration, BATCH_SIZE):
        batch_size = min(BATCH_SIZE, iteration - i)
        yield from (algo() for _ in range(batch_size))

def main():
    a = int(input("How many times: "))
    start_time = time.time()
    # Convert generator to list only for final output
    results = list(pullSystem(a))
    print(results)
    elapsed_time = time.time() - start_time
    print(f"\nTime elapsed: {elapsed_time:.2f} seconds")

if __name__ == "__main__":
    main()