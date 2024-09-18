import subprocess
from typing import Optional, Callable

from psutil import Process

from exquisitor.api import Event
from exquisitor.experiments.monitor.resource_monitor import ResourceMonitor

OnExperimentSuccessDelegate = Callable[['Experiment'], None]


class OnExperimentSuccess(Event[OnExperimentSuccessDelegate]):
    def broadcast(self, experiment: 'Experiment') -> None:
        return super().broadcast(experiment)


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

        self.on_success = OnExperimentSuccess()

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

        if return_code == 0:
            self.on_success.broadcast(self)

        return return_code == 0
