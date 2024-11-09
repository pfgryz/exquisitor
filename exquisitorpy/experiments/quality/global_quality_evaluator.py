from exquisitorpy.experiments.evaluator import Evaluator
from exquisitorpy.experiments.experiment import Experiment


class GlobalQualityEvaluator(Evaluator):

    def __init__(
            self,
            input_filepath: str,
            reference_filepath: str,
            record_filepath: str
    ):
        """
        Parameters
        ----------
        input_filepath : str
            File path to the input file
        reference_filepath : str
            File path to the reference solution
        record_filepath : str
            File path to the record file with quality metrics
        """
        self._input_filepath = input_filepath
        self._reference_filepath = reference_filepath
        self._record_filepath = record_filepath

    def __call__(self, experiment: Experiment):
        print("Calculating global quality")
