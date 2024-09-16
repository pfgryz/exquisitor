import os
import subprocess

from psutil import Process

from exquisitor.utils.monitor.resource_monitor import ResourceMonitor

if __name__ == "__main__":
    process = subprocess.Popen(["python3", "./exquisitor/misc/worker.py"],
                               stdout=subprocess.PIPE,
                               stderr=subprocess.PIPE,
                               close_fds=True)
    pr = Process(process.pid)
    monitor = ResourceMonitor(pr)
    monitor.start()

    while True:
        if process.poll() is not None:
            break

    monitor.wait()
    print("END")
    print(process.returncode)