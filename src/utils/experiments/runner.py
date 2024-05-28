import subprocess
import time

import matplotlib.pyplot as plt

from src.utils.experiments.memory import measure_memory_usage


class Runner:

    # region Dunder Methods

    def __init__(self):
        pass

    # endregion

    # region Run

    def run(self) -> None:
        s = time.time()
        process = subprocess.Popen(["python3", "worker.py"],
                                   stdout=subprocess.PIPE,
                                   stderr=subprocess.PIPE)
        pid = process.pid
        process

        print('Process')
        mem, mem_thr = measure_memory_usage(pid, interval=0.1)
        print('Memory usage')
        mem_thr.join()
        process.wait()
        print('Join')
        e = time.time()

        print(mem)
        print(e - s)
        plt.plot(mem)
        plt.show()

    # endregion


if __name__ == "__main__":
    runner = Runner()
    runner.run()
