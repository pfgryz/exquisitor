import time
from typing import Optional

from psutil import Process

from exquisitor.utils.monitor import Monitor, CPUMonitor, MemoryMonitor


class ResourceMonitor(Monitor):

    def __init__(
            self,
            process: Process,
            interval: float = 0.1,
            record_filepath: Optional[str] = None
    ):
        super().__init__(process, interval)
        self._cpu_monitor = CPUMonitor(
            self._process,
            record_filepath=(None
                             if record_filepath is None
                             else f"{record_filepath}.cpu.report"),
        )
        self._memory_monitor = MemoryMonitor(
            self._process,
            record_filepath=(None
                             if record_filepath is None
                             else f"{record_filepath}.memory.report"),
        )

    def monitor(self):
        self._cpu_monitor.start()
        self._memory_monitor.start()

        while self._process.is_running():
            time.sleep(self._interval)
            print('working')

        self._cpu_monitor.wait()
        self._memory_monitor.wait()

    def wait(self):
        if self._thread is not None:
            self._thread.join()

        if self._cpu_monitor is not None:
            self._cpu_monitor.wait()

        if self._memory_monitor is not None:
            self._memory_monitor.wait()
