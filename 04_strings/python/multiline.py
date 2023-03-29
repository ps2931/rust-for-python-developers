more = """many
lines
    whitespace
    """
print(more)

escape = ''' blah """ escaped '
newline
\n
test
'''
print(escape)

multiline_string = """How
are
you
today?"""

multiline_string_without_breaks = ' '.join(multiline_string.splitlines())
print(multiline_string_without_breaks)

single_char = 'ca'
