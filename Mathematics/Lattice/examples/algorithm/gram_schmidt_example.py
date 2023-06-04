from src import gram_schmidt, Lattice
from sage.all import *

# If you getting ModuleNotFoundError do the following command:
# export PYTHONPATH="${PYTHONPATH}:/path/to/project_root/"

if __name__ == "__main__":
    l: Lattice = Lattice(matrix(ZZ, [[2,0],[1,2]]))
    # CSE 206A: Lattice Algorithms and Applications: 1: Introduction to Lattices

    res: matrix = gram_schmidt(l.basis)
    print(res)
    print(res[0]*res[1] == 0)
    