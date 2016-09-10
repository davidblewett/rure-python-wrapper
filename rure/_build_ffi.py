import os

from cffi import FFI

ffi = FFI()
ffi.set_source('rure._ffi', None)

cur_dir = os.path.dirname(__file__)
header_fname = os.path.join(cur_dir, "rure.h")
ffi.cdef(open(header_fname).read())

if __name__ == '__main__':
    ffi.compile()
