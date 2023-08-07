class Point:
    """Point use in elliptic curves over finit fields
    """
    def __init__(self, x: int, y: int) -> None:
        self.x: int = x
        self.y: int = y

    def __str__(self) -> str:
        return f"({self.x}, {self.y})"


    def __eq__(self, __value: object) -> bool:
        return isinstance(__value, Point) and __value.x == self.x and __value.y == self.y
