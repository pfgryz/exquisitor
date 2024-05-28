import os
import subprocess

from matplotlib import pyplot as plt

from src.utils.experiments.memory import measure_memory_usage


class Runner:

    # region Dunder Methods

    def __init__(self):
        ...

    # endregion

    # region Run

    def run(self) -> None:
        # Initialize the process
        process = subprocess.Popen(["python3", "worker.py"],
                                   stdout=subprocess.PIPE,
                                   stderr=subprocess.PIPE,
                                   close_fds=True,
                                   preexec_fn=os.setpgrp)
        pid = process.pid

        # Start the counters
        memory_usage, memory_thread = measure_memory_usage(pid, interval=0.1)

        # Main loop
        while True:
            if process.poll() is not None:
                break

        # Wait for counter
        memory_thread.join()

        plt.plot([mi / 1024 / 1024 for mi in memory_usage])
        plt.show()

    # endregion


if __name__ == "__main__":
    runner = Runner()
    runner.run()
