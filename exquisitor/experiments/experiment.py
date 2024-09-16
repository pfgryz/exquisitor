import subprocess
from typing import Optional

from psutil import Process

from exquisitor.experiments.monitor.resource_monitor import ResourceMonitor


class Experiment:

    def __init__(
            self,
            name: str,
            arguments: list[str],
            record_filepath: Optional[str] = None
    ):
        """
        Parameters
        ----------
        name : str
            The name of the experiment
        arguments : list[str]
            The arguments to run program
        record_filepath : str
            Path to record file. If not specified, records will not be recorded.
        """
        self._name = name
        self._arguments = arguments
        self._record_filepath = record_filepath

    @property
    def name(self) -> str:
        return self._name

    def run(self) -> bool:
        """
        Run an exquisitor experiment.

        Returns
        -------
        bool
            True if successful, False otherwise.
        """

        process = subprocess.Popen(
            self._arguments,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            close_fds=True
        )

        # Start resource monitor
        resource_monitor = ResourceMonitor(
            Process(process.pid),
            record_filepath=self._record_filepath
        )
        resource_monitor.start()

        # Wait for program to end
        return_code = process.wait()
        resource_monitor.wait()

        return return_code == 0
