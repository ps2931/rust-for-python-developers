global_variable = 1

def f():
    global global_variable
    local_variable = 2

    def subf():
        print("local: ", local_variable)
        print("global: ", global_variable)
        enclosed_variable = 3 
        print(enclosed_variable)
        
        # local_variable = 5

        # If I uncomment above line, program will throw error.
        # because variabel should be defined before its first usage.
        # right now subf() has access to local_variable defined outside.

    subf()

    # enclosed_variable can be access in subf() function only
    # below line will throw error
    # print("enclosed", enclosed_variable)

    global_variable = 9
    local_variable = 1
    print("global: ", global_variable)
    print("local: ", local_variable)

    # example use of built-in scope
    # program has access to funtion int()
    blah = int("42")


if __name__ == "__main__":
    f()
