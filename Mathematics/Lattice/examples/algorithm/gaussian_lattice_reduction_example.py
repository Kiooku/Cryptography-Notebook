from src import Lattice, gaussian_lattice_reduction
from sage.all import *

# If you getting ModuleNotFoundError do the following command:
# export PYTHONPATH="${PYTHONPATH}:/path/to/project_root/"

if __name__ == "__main__":
    l: Lattice = Lattice(matrix(ZZ, [[66586820, 65354729],[6513996, 6393464]]))
    # An Introduction to Mathematical Cryptography (Chapter Lattices and Cryptography: p. 406)
    
    res: matrix = gaussian_lattice_reduction(l.basis)
    print(res)
    print(f"A solution to the SVP problem is: {res[0]}")