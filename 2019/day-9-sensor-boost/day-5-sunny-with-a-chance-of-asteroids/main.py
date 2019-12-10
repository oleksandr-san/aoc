def interpret_instruction(code):
    raw_params, opcode = divmod(code, 100)

    params = {}
    while raw_params:
        params[len(params)] = raw_params % 10
        raw_params //= 10

    return params, opcode

def execute(program):
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
            print(fetch_param(1))
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

def execute_smart(program, noun=12, verb=2):
    new_program = program[:]
    new_program[1] = noun
    new_program[2] = verb
    return execute(new_program)

def test():
    assert execute([1,0,0,0,99]) == [2,0,0,0,99]
    assert execute([2,3,0,3,99]) == [2,3,0,6,99]
    assert execute([2,4,4,5,99,0]) == [2,4,4,5,99,9801]
    assert execute([1,1,1,4,99,5,6,0,99]) == [30,1,1,4,2,5,6,0,99]

test()

print(interpret_instruction(1002))
print(interpret_instruction(11022))

#execute([3, 0, 4, 0, 99])
execute(map(int, open('input.dat').readline().strip().split(',')))
#execute([3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99])