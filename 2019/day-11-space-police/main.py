import intcode
from threading import Thread
from queue import Queue
from numpy import array
import time

program = intcode.load_program('input.dat')

def executor(program, qi, qo):
    print('Running a program...')
    intcode.execute(program, lambda: qi.get(timeout=2), qo.put)
    print('Finished running a program...')
    qo.put(None)

def rotate(direction, left=True):
    directions = ((0, -1), (1, 0), (0, 1), (-1, 0))
    shift = -1 if left else 1
    return directions[(directions.index(direction) + shift) % len(directions)]

def painter(qi, qo, start_color=0):
    panels = {}
    position = array((0, 0))
    direction = (0, -1)

    print('Start painting...')
    qo.put(start_color)

    while True:
        paint = qi.get(timeout=1)
        if paint is None:
            print(f'Received {paint}. Finish painting...')
            break

        #print(f'Painting panel {tuple(position)} to {paint}')
        panels[tuple(position)] = paint

        rotation = qi.get()
        direction = rotate(direction, left=rotation==0)
        position += direction
        #print(f'Rotating Moving to panel {tuple(position)}')

        panel = panels.get(tuple(position), 0)
        #print(f'Sending panel {tuple(position)} color {panel}')
        qo.put(panel)

    return panels

def paint_panels(program, start_color=0):
    qi, qo = Queue(), Queue()

    executor_thread = Thread(target=executor, args=(program[:], qi, qo))
    executor_thread.start()

    panels = painter(qo, qi, start_color)
    executor_thread.join()

    return panels

def render_panels(panels):
    width = max(panels.keys(), key=lambda p: p[0])[0]
    height = max(panels.keys(), key=lambda p: p[1])[1]

    for j in range(-1, height + 1):
        def color(paint):
            return {1: '#', 0: ' ', None: ' '}.get(paint)
        
        print(''.join(color(panels.get((i, j))) for i in range(-1, width + 1)))
            

print(len(paint_panels(program)))
render_panels(paint_panels(program, start_color=1))

#IRJHFKIH