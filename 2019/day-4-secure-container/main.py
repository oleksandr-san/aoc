def has_same_adjacent(value):
    for i in range(0, len(value) - 1):
        for j in range(i + 1, len(value)):
            if value[i] == value[j]:
                return True
    return False

def has_adjacent_pair(value):
    group_size = 1
    for i in range(1, len(value)):
        if value[i] == value[i - 1]:
            group_size += 1
        elif group_size == 2:
            return True
        else:
            group_size = 1
    return group_size == 2

def check_password(password):
    return len(password) == 6 and \
           has_same_adjacent(password) and \
           ''.join(sorted(password)) == password

def check_password_2(password):
    return len(password) == 6 and \
           has_adjacent_pair(password) and \
           ''.join(sorted(password)) == password

def find_passwords(passwords_range, check_password):
    for password in range(passwords_range[0], passwords_range[1] + 1):
        if check_password(str(password)):
            yield password

def test():
    assert has_same_adjacent('135679') == False
    assert has_same_adjacent('111123') == True
    assert has_same_adjacent('122345') == True
    assert has_same_adjacent('111111') == True

    assert has_adjacent_pair('135679') == False
    assert has_adjacent_pair('111123') == False
    assert has_adjacent_pair('122345') == True
    assert has_adjacent_pair('111111') == False
    assert has_adjacent_pair('221111') == True
    assert has_adjacent_pair('112211') == True
    assert has_adjacent_pair('111122') == True
    
    assert check_password('11') == False
    assert check_password('111111') == True
    assert check_password('223450') == False
    assert check_password('123789') == False

    assert check_password_2('112233') == True
    assert check_password_2('111111') == False
    assert check_password_2('123444') == False
    assert check_password_2('111122') == True
    assert check_password_2('123789') == False

test()

PASSWORDS_RANGE = (264360, 746325)
print(len(list(find_passwords(PASSWORDS_RANGE, check_password))))
print(len(list(find_passwords(PASSWORDS_RANGE, check_password_2))))