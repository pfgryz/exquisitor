import csv
import time
from typing import Callable, Optional

from psutil import NoSuchProcess, Process
from psutil._common import pcputimes as CPUUsage

from exquisitorpy.api import Event
from exquisitorpy.experiments.monitor.monitor import Monitor

OnCPUUsageMeasureDelegate = Callable[[CPUUsage, float], None]


class OnCPUUsageMeasure(Event[OnCPUUsageMeasureDelegate]):
    def broadcast(self, usage: CPUUsage, percent: float) -> None:
        return super().broadcast(usage, percent)


class CPUMonitor(Monitor):

    def __init__(
            self,
            process: Process,
            interval: float = 0.1,
            record_filepath: Optional[str] = None
    ):
        super().__init__(process, interval, record_filepath)

        self.on_measure = OnCPUUsageMeasure()

        if record_filepath:
            with open(self._record_filepath, "w", newline="") as handle:
                writer = csv.writer(handle)
                writer.writerow([
                    "timestamp",
                    "percent",
                    "user",
                    "system",
                    "children_user",
                    "children_system"
                ])

            self.on_measure.add(self._log)

    def _log(self, usage: CPUUsage, percent: float) -> None:
        timestamp = str(time.time())

        with open(self._record_filepath, "a", newline="") as handle:
            writer = csv.writer(handle)
            writer.writerow([
                timestamp,
                percent,
                usage.user,
                usage.system,
                usage.children_user,
                usage.children_system
            ])

    def monitor(self):
        try:
            while self._process.is_running():
                percent = self._process.cpu_percent(interval=self._interval)
                usage = self._process.cpu_times()
                self.on_measure.broadcast(usage, percent)
        except NoSuchProcess:
            return
