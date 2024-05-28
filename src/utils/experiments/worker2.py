import time

if __name__ == "__main__":
    x = [y for y in range(1000000)]
    z = [x for _ in range(10)]
    time.sleep(2)
