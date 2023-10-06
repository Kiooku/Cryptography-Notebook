from src import LLL, Lattice, LLL_optimize
from sage.all import *
from functools import reduce

# If you getting ModuleNotFoundError do the following command:
# export PYTHONPATH="${PYTHONPATH}:/path/to/project_root/"

if __name__ == "__main__":
    basis: matrix = matrix(ZZ, [[19, 2, 32, 46, 3, 33], 
                                [15, 42, 11, 0, 3, 24],
                                [43, 15, 0, 24, 4, 16],
                                [20, 44, 44, 0, 18, 15],
                                [0, 48, 35, 16, 31, 31],
                                [48, 33, 32, 9, 1, 29]])
    # An Introduction to Mathematical Cryptography (Chapter Lattices and Cryptography: p. 415)
    res: matrix = LLL(basis)
    print(res)
    print(f"Determinant initial basis: {basis.det()}", f"Determinant res: {res.det()}")
    print(basis.det() == res.det() or basis.det() == -res.det())
    l1: Lattice = Lattice(basis)
    l2: Lattice = Lattice(res)
    print(l1.hadamard_ratio(5), l2.hadamard_ratio(5))
    res_optimize = LLL_optimize(basis)
    print(res_optimize)
    l3: Lattice = Lattice(res_optimize)
    print(l3.hadamard_ratio(5))
    