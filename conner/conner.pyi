"""
A module for finding matching points in a set of points.
"""
import numpy as np
from typing import Any

def check_points(points: np.ndarray) -> Any:
    """
    Checks for matching points with multicore procedures.

    Returns:
        array-list[list[int]]: An array of indices with matching points as lists.
    """
def check_points_seq(points: np.ndarray) -> Any:
    """
    Checks for matching points sequentially.

    Returns:
        array-list[list[int]]: An array of indices with matching points as lists.
    """
