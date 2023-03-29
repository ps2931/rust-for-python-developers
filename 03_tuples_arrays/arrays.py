"demo of arrays"
from array import array

if __name__ == "__main__":
    xs = array('i', [1,2,3,4,5])

    # this is dumb use `numpy.zeors((500,0), dtype=numpy.uint64)` instead
    ys = array('Q', [0] * 500)

    print("first element: ", xs[0])
    print("second element: ", xs[1])

    print("small size: ", len(xs))
    print("big size: ", len(ys))

    print(xs[5]) // index out of bound
