from sage.all import *
from src import Lattice
    
# If you getting ModuleNotFoundError do the following command:
# export PYTHONPATH="${PYTHONPATH}:/path/to/project_root/"

if __name__ == "__main__":
    l: Lattice = Lattice(matrix(ZZ, [[1,2], [-1,1]]))
    l2: Lattice = Lattice(matrix(ZZ, [[1,2],[0,3]]))
    print(f"toString:\n{l}")
    print(f"Equality: {l == l2}")
    