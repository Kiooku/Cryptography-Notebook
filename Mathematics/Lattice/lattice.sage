class Lattice:
    def __init__(self, basis: Matrix):
        self.basis = basis

    
    def babai_algorithm(self, w: vector) -> vector:
        """ Solve the CVP if the basis is sufficiently orthogonal

        Args:
            w (vector)

        Output:
            (vector): vector in the lattice that is close to 'w'
        """
        t_i = self.basis.solve_left(w)
        v = zero_vector(len(t_i))
        for i in range(len(t_i)):
            v += round(t_i[i]) * self.basis[i]

        return v


if __name__ == "__main__":
    # An introduction to mathematical cryptography (Second edition)

    # Babai's Algorithm [Example 7.35]
    v1: vector = vector([137, 312])
    v2: vector = vector([215, -187])
    l: Lattice = Lattice(Matrix([v1, v2]))
    w: vector = vector([53172, 81743])

    v: vector = l.babai_algorithm(w)
    assert round((v - w).norm(), 2) == 76.12
    print(round((v - w).norm(), 2))