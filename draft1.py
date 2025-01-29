import random

def is_true_with_probability(probability):
    return random.random() < probability

def algo():
    pulled = False
    count = 1
    while not pulled:
        if count <= 73:
            pulled = random.random() < 0.006
        elif count > 73 and count <= 89:
            pulled = random.random() < (0.006 + (0.062 * count))
        elif count == 90:
            pulled = True
        count += 1
    return count

def pullSystem(iteration):
    result = []
    for _ in range(iteration):
        result.append(algo())
    return result

def main():
    a = int(input("How many times: "))
    print(pullSystem(a))

if __name__ == "__main__":
    main()
