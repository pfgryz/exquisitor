from exquisitor.experiments.experiment import Experiment
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

    runner.add_experiment(
        Experiment("Complex", ["python3", "./exquisitor/misc/worker.py"],
                   "ex1")
    )
    runner.add_experiment(
        Experiment("Simple", ["python3", "./exquisitor/misc/worker2.py"],
                   "ex2")
    )

    runner.run()
