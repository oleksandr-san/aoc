def interpret_instruction(code):
    raw_params, opcode = divmod(code, 100)

    params = {}
    while raw_params:
        params[len(params)] = raw_params % 10
        raw_params //= 10

    return params, opcode

def execute(program, input=input, output=print):
    relative_base, pos = 0, 0
    memory = {}

    while True:
        params, opcode = interpret_instruction(program[pos])
        if opcode == 99:
            break

        def make_addr(idx):
            mode = params.get(idx - 1, 0)
            if mode == 0:
                return program[pos + idx]
            elif mode == 1:
                return pos + idx
            elif mode == 2:
                return relative_base + program[pos + idx]
            else:
                raise RuntimeError('Unknown parameter mode')

        def fetch_val(idx):
            addr = make_addr(idx)
            if addr < len(program):
                return program[addr]
            else:
                return memory.get(addr, 0)

        def put_val(idx, value):
            addr = make_addr(idx)
            if addr < len(program):
                program[addr] = value
            else:
                memory[addr] = value

        if opcode == 1:
            put_val(3, fetch_val(1) + fetch_val(2))
            pos += 4    
        elif opcode == 2:
            put_val(3, fetch_val(1) * fetch_val(2))
            pos += 4
        elif opcode == 3:
            put_val(1, int(input()))
            pos += 2
        elif opcode == 4:
            output(fetch_val(1))
            pos += 2
        elif opcode == 5:
            if fetch_val(1) != 0:
                pos = fetch_val(2)
            else:
                pos += 3
        elif opcode == 6:
            if fetch_val(1) == 0:
                pos = fetch_val(2)
            else:
                pos += 3
        elif opcode == 7:
            put_val(3, 1 if fetch_val(1) < fetch_val(2) else 0)
            pos += 4
        elif opcode == 8:
            put_val(3, 1 if fetch_val(1) == fetch_val(2) else 0)
            pos += 4
        elif opcode == 9:
            relative_base += fetch_val(1)
            pos += 2

    return program

def capture_output(program, input=input):
    output = []
    execute(program, input=input, output=output.append)
    return output

def load_program(path):
    with open(path) as fp:
        return list(map(int, fp.readline().strip().split(',')))

def test():
    assert execute([1,0,0,0,99]) == [2,0,0,0,99]
    assert execute([2,3,0,3,99]) == [2,3,0,6,99]
    assert execute([2,4,4,5,99,0]) == [2,4,4,5,99,9801]
    assert execute([1,1,1,4,99,5,6,0,99]) == [30,1,1,4,2,5,6,0,99]

    program, output = [109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99], []
    execute(program, output=output.append)
    assert program == output

    program, output = [1102,34915192,34915192,7,4,7,99,0], []
    execute(program, output=output.append)
    assert output[0] == 1219070632396864

    program, output = [104,1125899906842624,99], []
    execute(program, output=output.append)
    assert output[0] == 1125899906842624

test()