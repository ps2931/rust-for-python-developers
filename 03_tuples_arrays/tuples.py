from collections import namedtuple

def reverse(pair: tuple) -> tuple:
    a_int, b_bool = pair
    return (b_bool, a_int)

Matrix = namedtuple('Matrix', ['x0', 'x1', 'x2', 'x3'])

if __name__ == "__main__":
    many_types = (1,2,'a',True)

    print("first value: ", many_types[0])
    print("second value: ", many_types[1])

    tuple_of_tuples = ((1,2,2), (4,-1,-2))
    print("tuple of tuples: ", tuple_of_tuples)

    pair = (1, True)
    print("pair is ", pair)
    print("the reverse pair is ", reverse(pair))

    print("one element tuple: {:?}", (5, ))
    print("just one integer: {:?}", (5))

    _tuple = (1, "hello", 4.5, True)
    a, b, c, d, = _tuple

    matrix = Matrix(1.1, 1.2, 2.1, 2.2)
    print(matrix)