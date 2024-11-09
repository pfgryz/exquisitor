import logging
from typing import Optional

from exquisitorpy.experiments.experiment import Experiment

logger = logging.getLogger(__name__)
logger.setLevel(logging.INFO)

handler = logging.StreamHandler()
formatter = logging.Formatter('%(asctime)s [%(levelname)s] %(message)s')
handler.setFormatter(formatter)
logger.addHandler(handler)


class Runner:

    def __init__(self, experiments: Optional[list[Experiment]] = None):
        """
        Parameters
        ----------
        experiments : Optional[list[Experiment]] = None
            List of experiments to run
        """
        self._experiments = []

        if experiments is not None:
            for experiment in experiments:
                self._experiments.append(experiment)

    def add_experiment(self, experiment: Experiment) -> None:
        """
        Adds an experiment to the list of experiments.

        Parameters
        ----------
        experiment : Experiment
            The experiment to be added.
        """
        self._experiments.append(experiment)

    def run(self) -> None:
        """
        Run all experiments.
        """
        total = len(self._experiments)

        if total == 0:
            logger.warn('No experiments to run.')
            return

        failed = 0
        successful = 0

        logger.info(f"Starting runner with {total} experiments")

        for index, experiment in enumerate(self._experiments, start=1):
            logger.info(
                f"[{index} / {total}] "
                f"Running experiment \"{experiment.name}\"..."
            )

            try:
                result = experiment.run()

                if result:
                    successful += 1
                    logger.info(f"[{index} / {total}] Successful")
                else:
                    failed += 1
                    logger.info(f"[{index} / {total}] Failed")
            except KeyboardInterrupt:
                logger.info(f"[{index} / {total}] Interrupted")
                break
            except Exception as e:
                logger.exception(e)

        logger.info(
            f"Finished runner with {total} experiments. "
            f"Successful: {successful}, Failed: {failed}"
        )
