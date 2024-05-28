import time
from threading import Thread
from typing import Tuple, List

import psutil


def measure_memory_usage(pid: int, interval: float = 0.1) -> Tuple[
    List[float], Thread]:
    memory_usage = []

    def poll():
        try:
            process = psutil.Process(pid)

            while process.is_running:
                memory_usage.append(get_process_memory_usage(process))
                time.sleep(interval)
        except psutil.NoSuchProcess:
            return

    memory_thread = Thread(target=poll)
    memory_thread.start()
    return memory_usage, memory_thread


def get_process_memory_usage(process, recursive: bool = True):
    memory = process.memory_info().rss
    for child in process.children():
        memory += get_process_memory_usage(child, recursive)
    return memory
