import gc
import subprocess
import time


def cpu_intensive():
    print("CPU intensive task")
    time.sleep(3)


def child():
    process = subprocess.Popen(["python3", "./exquisitorpy/misc/worker2.py"],
                               stdout=subprocess.PIPE,
                               stderr=subprocess.PIPE)
    process.wait()
    gc.collect()


def main():
    cpu_intensive()
    child()
    cpu_intensive()
    child()
    cpu_intensive()


if __name__ == "__main__":
    main()
    print("BYE BYE")
