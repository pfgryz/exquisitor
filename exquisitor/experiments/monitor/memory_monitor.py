import csv
import time
from typing import Callable, Optional

from psutil import Process, NoSuchProcess

from exquisitor.api import Event
from exquisitor.experiments.monitor.monitor import Monitor

OnMemoryUsageMeasureDelegate = Callable[[float], None]


class OnMemoryUsageMeasure(Event[OnMemoryUsageMeasureDelegate]):
    def broadcast(self, usage: float) -> None:
        return super().broadcast(usage)


class MemoryMonitor(Monitor):

    def __init__(
            self,
            process: Process,
            interval: float = 0.1,
            record_filepath: Optional[str] = None
    ):
        super().__init__(process, interval, record_filepath)

        self.on_measure = OnMemoryUsageMeasure()

        # Add logging
        if record_filepath is not None:
            with open(self._record_filepath, "w", newline="") as handle:
                writer = csv.writer(handle)
                writer.writerow([
                    "timestamp",
                    "usage"
                ])

            self.on_measure.add(self._log)

    def _log(self, usage: float) -> None:
        timestamp = str(time.time())

        with open(self._record_filepath, "a", newline="") as handle:
            writer = csv.writer(handle)
            writer.writerow([
                timestamp,
                usage / 1024 / 1024
            ])

    def monitor(self):
        try:
            while self._process.is_running():
                usage = MemoryMonitor.get_process_memory_usage(
                    self._process
                )
                self.on_measure.broadcast(usage)
                time.sleep(self._interval)
        except NoSuchProcess:
            return

    @staticmethod
    def get_process_memory_usage(
            process: Process,
            recursive: bool = True
    ) -> int:
        """
        Calculates the memory usage of a process.

        Parameters
        ----------
        process : Process
            The process to get the memory usage for.
        recursive : bool = True
            Whether to recursively calculate the memory usage of a process.
        """
        if not process.is_running():
            return 0

        usage = process.memory_info().rss

        if recursive:
            for child in process.children():
                usage += MemoryMonitor.get_process_memory_usage(
                    child,
                    recursive
                )

        return usage
