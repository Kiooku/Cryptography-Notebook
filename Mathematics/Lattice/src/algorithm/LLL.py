from src import gram_schmidt_lattice
from sage.all import *

def mu(i: int, j: int, b1: matrix, b2: matrix)-> float:
    return (b1[i] * b2[j]) / pow(b2[j].norm(), 2)

def size_reduction(k: int, current_basis: matrix, reduced_basis: matrix)-> None:
    """Size reduction algorithm use for LLL algorithm

    Args:
        k (int): _description_
        current_basis (matrix): _description_
        reduced_basis (matrix): _description_
    """
    for j in range(k):
        m: int = round(mu(k, j, current_basis, reduced_basis)) # Nearest integer of u_{k, j}
        current_basis[k] = current_basis[k] - m * reduced_basis[j]

def LLL(b: matrix)-> matrix:
    """Lenstra–Lenstra–Lovász lattice basis reduction algorithm

    Args:
        b (matrix): initial basis

    Returns:
        matrix: reduced basis
    """
    copy_basis: matrix = copy(b)
    k: int = 1
    #TODO utiliser gram_schmidt_coefficients
    reduced_basis: matrix
    gram_schmidt_coefficients: dict
    while k < b.nrows():
        reduced_basis, gram_schmidt_coefficients = gram_schmidt_lattice(copy_basis)
        size_reduction(k, copy_basis, reduced_basis)
        # Lovásk Condition
        if pow(reduced_basis[k].norm(), 2) >= (3/4 - pow(mu(k, k-1, copy_basis, reduced_basis), 2)) * pow(reduced_basis[k-1].norm(),2):
            k += 1
        else:
            # Swap step
            copy_basis[k-1], copy_basis[k] = copy_basis[k], copy_basis[k-1]
            k = max(k-1, 1)

    return reduced_basis


### Other algorithm to do LLL
# Figure 6.8 and 6.9 from "An Introduction to Mathematical Cryptography" (Chapter Lattices and Cryptography: page 432 and 433)
def red(k: int, l: int, b1: matrix, gram_schmidt_coefficients: dict):
    if abs(gram_schmidt_coefficients[(k,l)]) > 1/2:
        m: int = round(gram_schmidt_coefficients[(k,l)])
        b1[k] -= m*b1[l]
        gram_schmidt_coefficients[(k,l)] -= m
        for i in range(l): # TODO vérifier pour tous les algos LLL que je respecte le k-1 ou l-1
            gram_schmidt_coefficients[(k,i)] = gram_schmidt_coefficients[(k,i)] - m * gram_schmidt_coefficients[(l,i)]

def swap(k: int, k_max: int, b: matrix, B: list[float], gram_schmidt_coefficients: dict):
    b[k-1], b[k] = b[k], b[k-1]
    for j in range(k-1):
        gram_schmidt_coefficients[(k-1,j)], gram_schmidt_coefficients[(k,j)] = gram_schmidt_coefficients[(k,j)], gram_schmidt_coefficients[(k-1,j)]
    mu: float = gram_schmidt_coefficients[(k,k-1)]
    temp_B: float = B[k] - pow(mu, 2) * B[k-1]
    gram_schmidt_coefficients[(k,k-1)] = (mu * B[k-1]) / temp_B
    B[k] = (B[k-1] * B[k]) / temp_B
    B[k-1] = temp_B
    for i in range(k+1, k_max):
        m: float = gram_schmidt_coefficients[(i,k)]
        gram_schmidt_coefficients[(i,k)] = gram_schmidt_coefficients[(i,k-1)] - mu * m
        gram_schmidt_coefficients[(i,k-1)] = m + gram_schmidt_coefficients[(k,k-1)] * gram_schmidt_coefficients[(i,k)]

def LLL_optimize(b: matrix)-> matrix:
    copy_basis = copy(b)
    k: int = 1
    k_max: int = 0
    reduced_basis: matrix
    gram_schmidt_coefficients: dict
    reduced_basis, gram_schmidt_coefficients = gram_schmidt_lattice(copy_basis)
    B: list[float] = [0.0 for _ in range(b.nrows())]
    B[0] = pow(copy_basis[0].norm(), 2)

    while k < b.nrows():
        if k > k_max:
            k_max, reduced_basis[k] = k, copy_basis[k]
            for j in range(k):
                gram_schmidt_coefficients[(k,j)] = copy_basis[k] * reduced_basis[j] / B[j]
                reduced_basis[k] -= gram_schmidt_coefficients[(k,j)] * reduced_basis[j]
            B[k] = pow(reduced_basis[k].norm(), 2)

        red(k, k-1, copy_basis, gram_schmidt_coefficients)

        while B[k] < (3/4 - pow(gram_schmidt_coefficients[(k,k-1)], 2) * B[k-1]):
            swap(k, k_max, copy_basis, B, gram_schmidt_coefficients)
            k = max(2, k-1)
            red(k, k-1, copy_basis, reduced_basis)

        for l in range(k-2, -1, -1): #TODO check if it's right
            red(k, l, copy_basis, reduced_basis)

        k += 1

    return reduced_basis
