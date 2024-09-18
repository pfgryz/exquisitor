from exquisitor.experiments.experiment import Experiment
from exquisitor.experiments.quality.global_quality_evaluator import \
    GlobalQualityEvaluator
from exquisitor.experiments.runner import Runner

# if __name__ == "__main__":
#     process = subprocess.Popen(["python3", "./exquisitor/misc/worker.py"],
#                                stdout=subprocess.PIPE,
#                                stderr=subprocess.PIPE,
#                                close_fds=True)
#     pr = Process(process.pid)
#     monitor = ResourceMonitor(pr)
#     monitor.start()
#
#     while True:
#         if process.poll() is not None:
#             break
#
#     monitor.wait()
#     print("END")
#     print(process.returncode)

if __name__ == "__main__":
    runner = Runner()

    ex1 = Experiment("Complex", ["python3", "./exquisitor/misc/worker.py"],
                     "ex1")
    ex2 = Experiment("Simple", ["python3", "./exquisitor/misc/worker2.py"],
                     "ex2")

    ex1.on_success.add(GlobalQualityEvaluator("ex1.cpu.report", "ex1.memory.report", "ex1.quality.report"))

    runner.add_experiment(ex1)
    runner.add_experiment(ex2)

    runner.run()
