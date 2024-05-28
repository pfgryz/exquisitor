import time
from threading import Thread
from typing import Tuple, List

import psutil


def measure_cpu_usage(pid: int, interval: float) -> Tuple[List, Thread]:
    cpu_times = [None]

    def poll():
        try:
            process = psutil.Process(pid)

            while process.is_running():
                cpu_times[0] = process.cpu_times()
                time.sleep(interval)
        except psutil.NoSuchProcess:
            return

    cpu_thread = Thread(target=poll)
    cpu_thread.start()
    return cpu_times, cpu_thread
