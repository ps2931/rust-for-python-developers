if __name__ == '__main__':
    x = 1
    print(x)
    # bind x again shadows old one from above
    x = 'A'
    print(x)

    # declare, init
    something = None
    x = 5
    print("x: {x}, something: {something}".format(x=x, something=something))
    something = x * 5
    print("x: {x}, something: {something}".format(x=x, something=something))

    # mutability (default behaviour)
    y = 0
    y = y * 2 + x
    print(y)

    BLAH = 42 # only const by convention
    y *= BLAH
    print(y)



