

program = [1,0,0,0,99]

def execute(program):
    pos = 0
    while True:
        opcode = program[pos]
        if opcode == 99:
            break

        lhs_pos = program[pos + 1]
        rhs_pos = program[pos + 2]
        res_pos = program[pos + 3]

        if opcode == 1:
            program[res_pos] = program[lhs_pos] + program[rhs_pos]
        elif opcode == 2:
            program[res_pos] = program[lhs_pos] * program[rhs_pos]
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

    assert execute_smart(list(map(int, open('input.dat').readline().strip().split(','))))[0] == 4138687

test()

program = list(map(int, open('input.dat').readline().strip().split(',')))

print(execute_smart(program)[0])

for noun in range(0, 100):
    for verb in range(0, 100):
        if execute_smart(program, noun, verb)[0] == 19690720:
            print(100 * noun + verb)
