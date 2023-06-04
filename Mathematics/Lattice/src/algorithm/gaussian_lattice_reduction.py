from sage.all import *

def gaussian_lattice_reduction(b: matrix)-> matrix:
    """Gaussian lattice reduction algorithm
    
    Find a shortest nonzero vector in a lattice of dimension 2

    Args:
        b (matrix): basis of the lattice

    Returns:
        matrix: reduce basis (b[0] is a shortest nonzero vector in L)
    """
    assert b.nrows() == 2, "should be a 2-dimensional lattice"
    m: int
    new_basis = copy(b)

    while True:
        if new_basis[1].norm() < new_basis[0].norm():
            new_basis[0], new_basis[1] = new_basis[1], new_basis[0]
            # https://mathworld.wolfram.com/NearestIntegerFunction.html
            m = round(new_basis[0]*new_basis[1]/pow(new_basis[0].norm(), 2))
            if m == 0:
                return new_basis
            new_basis[1] -= m * new_basis[0]
    