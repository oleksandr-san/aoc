def required_fuel(mass):
    return mass // 3 - 2

def real_required_fuel(mass):
    fuel = required_fuel(mass)
    if fuel <= 0:
        return 0
    else:
        return fuel + real_required_fuel(fuel)

def fetch_module_mass(path):
    with open(path) as fp:
        for line in fp.readlines():
            yield int(line.strip())

def test():
    assert required_fuel(12) == 2
    assert required_fuel(14) == 2
    assert required_fuel(1969) == 654
    assert required_fuel(100756) == 33583

    assert real_required_fuel(12) == 2
    assert real_required_fuel(1969) == 966
    assert real_required_fuel(100756) == 50346

test()
print(sum(map(required_fuel, fetch_module_mass('input.dat'))))
print(sum(map(real_required_fuel, fetch_module_mass('input.dat'))))

