from time import sleep

from celery import Celery

app = Celery(
    'base',
    broker='redis://localhost:6379/0',
    task_acks_late=True
)


@app.task
def add(x, y):
    print('I got task')
    sleep(10)
    return x + y


if __name__ == '__main__':
    import sys
    app.start(argv=sys.argv[1:])
