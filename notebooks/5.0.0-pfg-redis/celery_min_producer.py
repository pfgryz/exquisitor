from time import sleep

from celery import Celery

app = Celery(
    'myapp',
    broker='redis://localhost:6379/0',
    task_acks_late=True
)


if __name__ == '__main__':
    import sys
    app.send_task('base.add', [1, 2])
    app.send_task('base.add', [3, 4])










