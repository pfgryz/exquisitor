import gc
import time

if __name__ == "__main__":
    x = [y for y in range(10000000)]
    z = [x for _ in range(1000)]
    time.sleep(1)
    gc.collect()
    time.sleep(1)
