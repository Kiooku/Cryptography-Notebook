def is_superincreasing(M: list[int]) -> bool:
    """ Define if a set is a superincreasing sequence or not

        Args:
            M (list[int]): superincreasing sequence

        Returns:
            bool
    """
    current_sum: int = M[0]
    for k in range(1, len(M)):
        if M[k] <= current_sum:
            return False
        current_sum += M[k]
    
    return True


def solve(M: list[int], S: int) -> list[int]:
    """ Solve the Subset-Sum Problem for superincreasing sequence

        Args:
            M (list[int]): superincreasing sequence
            S (int): sum to find

        Returns:
            list[int]: solution
    """
    # An introduction to mathematical cryptography (Second edition) [Proposition 7.5]
    solution: list[int] = []
    current_S: int = S
    for i in range(len(M)-1, -1, -1):
        if S >= M[i]:
            solution.append(M[i])
            S -= M[i]
    
    return solution


if __name__ == "__main__":
    # An introduction to mathematical cryptography (Second edition) [Example 7.6]
    M: list[int] = [3, 11, 24, 50, 115]
    S: int = 142
    assert is_superincreasing(M) == True
    solution: list[int] = solve(M, S)
    assert sum(solution) == S
    print(f"The solution to the Subset-Sum problem S = {S} with M = {M} is: {solution}")