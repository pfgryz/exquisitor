from abc import abstractmethod

from exquisitor.experiments.experiment import Experiment


class Evaluator:

    @abstractmethod
    def __call__(self, experiment: Experiment):
        ...
