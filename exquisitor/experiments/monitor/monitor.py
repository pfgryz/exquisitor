from abc import abstractmethod
from threading import Thread
from typing import Optional

from psutil import Process


class Monitor:

    def __init__(
            self,
            process: Process,
            interval: float = 0.1,
            record_filepath: Optional[str] = None
    ):
        """
        Parameters
        ----------
        process : Process
            Process to monitor resource
        interval : float = 0.1
            Monitoring resolution interval in seconds
        record_filepath : Optional[str] = None
            Path to record file
        """
        self._process = process
        self._interval = interval
        self._record_filepath = record_filepath
        self._thread = None

    def start(self) -> None:
        """
        Starts the monitoring thread
        """
        if self._thread is not None:
            return None

        self._thread = Thread(target=self.monitor)
        self._thread.start()

    @abstractmethod
    def monitor(self):
        """
        Monitoring function
        """
        raise NotImplementedError()

    def wait(self):
        """
        Waits until the monitoring thread is stopped
        """
        if self._thread is not None:
            self._thread.join()
