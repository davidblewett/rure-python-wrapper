import os
import sys
from ._ffi import ffi


class RegexError(Exception):
    pass


class RegexSyntaxError(RegexError):
    pass


class CompiledTooBigError(RegexError):
    pass


def find_library():
    libname = "rure_wrap"
    if sys.platform == 'win32':
        prefix = ''
        suffix = 'dll'
    elif sys.platform == 'darwin':
        prefix = 'lib'
        suffix = 'dylib'
    else:
        prefix = 'lib'
        suffix = 'so'
    cur_dir = os.path.dirname(__file__)
    return os.path.join(cur_dir, "{}{}.{}".format(prefix, libname, suffix))


lib = ffi.dlopen(find_library())


def checked_call(fn, err, *args):
    all_args = list(args) + [err]
    res = fn(*all_args)
    import pdb; pdb.set_trace()  # NOQA
    msg = ffi.string(lib.rure_error_message(err))
    if msg == 'no error':
        return res
    elif msg.startswith('Error parsing regex'):
        raise RegexSyntaxError(msg)
    elif msg.startswith('Compiled regex exceeds size limit'):
        raise CompiledTooBigError(msg)
    else:
        # FIXME: introspect text to emit specific error
        raise RegexError(msg)
