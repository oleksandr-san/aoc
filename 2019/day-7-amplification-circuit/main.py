from itertools import permutations
from queue import Queue
from threading import Thread

def interpret_instruction(code):
    raw_params, opcode = divmod(code, 100)

    params = {}
    while raw_params:
        params[len(params)] = raw_params % 10
        raw_params //= 10

    return params, opcode

def execute(program, input=input, output=print):
    pos = 0
    while True:
        params, opcode = interpret_instruction(program[pos])
        if opcode == 99:
            break

        def fetch_param(param_idx):
            if params.get(param_idx - 1, 0) == 0:
                return program[program[pos + param_idx]]
            else:
                return program[pos + param_idx]

        if opcode == 1 or opcode == 2:
            lhs = fetch_param(1)
            rhs = fetch_param(2)
            res_pos = program[pos + 3]

            if opcode == 1:
                program[res_pos] = lhs + rhs
            elif opcode == 2:
                program[res_pos] = lhs * rhs
            pos += 4
        elif opcode == 3:
            res_pos = program[pos + 1]
            program[res_pos] = int(input())
            pos += 2
        elif opcode == 4:
            output(fetch_param(1))
            pos += 2
        elif opcode == 5:
            if fetch_param(1) != 0:
                pos = fetch_param(2)
            else:
                pos += 3
        elif opcode == 6:
            if fetch_param(1) == 0:
                pos = fetch_param(2)
            else:
                pos += 3
        elif opcode == 7:
            lhs = fetch_param(1)
            rhs = fetch_param(2)
            res_pos = program[pos + 3]
            program[res_pos] = 1 if lhs < rhs else 0
            pos += 4
        elif opcode == 8:
            lhs = fetch_param(1)
            rhs = fetch_param(2)
            res_pos = program[pos + 3]
            program[res_pos] = 1 if lhs == rhs else 0
            pos += 4

    return program

def amplify_chain(program, phases):
    value = 0
    for phase in phases:
        output = []
        execute(program[:], [value, phase].pop, output.append)
        value = output[-1]
    return value

def amplify_loop(program, phases):
    queues = [Queue() for _ in phases]

    for phase, queue in zip(phases, queues):
        queue.put(phase)
    queues[0].put(0)

    threads = []
    for i, qi in enumerate(queues):
        qo = queues[(i + 1) % len(queues)]
        t = Thread(target=execute, args=(program[:], qi.get, qo.put))
        t.start()
        threads.append(t)

    for t in threads:
        t.join()

    return queues[0].get()

def test():
    assert execute([1,0,0,0,99]) == [2,0,0,0,99]
    assert execute([2,3,0,3,99]) == [2,3,0,6,99]
    assert execute([2,4,4,5,99,0]) == [2,4,4,5,99,9801]
    assert execute([1,1,1,4,99,5,6,0,99]) == [30,1,1,4,2,5,6,0,99]

    assert amplify_chain([3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0], [4,3,2,1,0]) == 43210
    assert amplify_chain([3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0], [0,1,2,3,4]) == 54321
    assert amplify_chain([3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0], [1,0,4,3,2]) == 65210

    assert amplify_loop([3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10], [9,7,8,5,6]) == 18216
    assert amplify_loop([3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5], [9,8,7,6,5]) == 139629729

test()

program = list(map(int, open('input.dat').readline().strip().split(',')))
print(max(amplify_chain(program, phases) for phases in permutations([0, 1, 2, 3, 4])))
print(max(amplify_loop(program, phases) for phases in permutations([5, 6, 7, 8, 9])))



