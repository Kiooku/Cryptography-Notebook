from sage.all import *

def gram_schmidt(b: matrix) -> matrix:
    """ Gram-Schmidt algorithm
    
    'May have rounding error'
    'Vector spaces always admit an orthogonal basis, this is not true for lattices'
    
    Args:
        b (matrix): basis of the vector space

    Returns:
        matrix: orthogonal basis of the vector space
    """
    orthogonal_basis: matrix = matrix(RR, b.nrows(), b.ncols())
    orthogonal_basis[0] = b[0]
    for i in range(1, b.nrows()):
        orthogonalization: float = 0.0
        for j in range(i):
            gram_schmidt_coefficient: float = (b[i] * orthogonal_basis[j]) / pow(orthogonal_basis[j].norm(), 2)
            orthogonalization += gram_schmidt_coefficient * orthogonal_basis[j]
        orthogonal_basis[i] = b[i] - orthogonalization

    return orthogonal_basis


def gram_schmidt_lattice(b: matrix)-> (matrix, dict):
    """Gram-Schmidt algorithm for Lattice (Stay in ZZ) 
    
    'May have rounding error'

    Args:
        b (matrix): initial basis of the lattice

    Returns:
        matrix: orthogonal basis of the lattice
    """
    orthogonal_basis: matrix = matrix(ZZ, b.nrows(), b.ncols())
    orthogonal_basis[0] = b[0]
    gram_schmidt_coefficients: dict = {}
    for i in range(1, b.nrows()):
        orthogonalization: vector = vector([0 for _ in range(len(b[0]))])
        for j in range(i):
            gram_schmidt_coefficient: int = round((b[i] * orthogonal_basis[j]) / pow(orthogonal_basis[j].norm(), 2))
            gram_schmidt_coefficients[(i, j)] = gram_schmidt_coefficient
            orthogonalization += gram_schmidt_coefficient * orthogonal_basis[j]
        orthogonal_basis[i] = b[i] - orthogonalization

    return (orthogonal_basis, gram_schmidt_coefficients)
