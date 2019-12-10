from functools import reduce

def parse_layers(data, width, height):
    layers = []

    for i in range(len(data) // (width * height)):
        layer = []
        for j in range(height):
            layer.append(data[width*height*i + j*width:width*height*i + j*width + width])
        layers.append(layer)

    return layers

def test():
    assert parse_layers('123456789012', 3, 2) == [['123', '456'], ['789', '012']]

test()

data = open('input.dat').readline().strip()
layers = parse_layers(data, 25, 6)

def count(layer, value):
    return sum(row.count(value) for row in layer)

check_layer = min(layers, key=lambda l: count(l, '0'))
print(count(check_layer, '1') * count(check_layer, '2'))

def merge_row(r1, r2):
    return ''.join(p1 if p1 != '2' else p2 for p1, p2 in zip(r1, r2))

def merge_layers(l1, l2):
    return [merge_row(r1, r2) for r1, r2 in zip(l1, l2)]

print('\n'.join(reduce(merge_layers, layers)).replace('0', ' ').replace('1', 'O'))