"""
A module for calculating stochastic functions of data sets.
"""
import numpy as np
from typing import Any

def k_test(points, area, max_d) -> tuple[list,list]:
	"""
	Calculate the K-Function for the points in the area for a range of distances.
	The resulting distances are equally spaced from 0 to max_d.

	Arguments:
		points: The points to calculate the K-Function for. [n,2] ndarray.
		area: The area to calculate the K-Function for.
		max_d: The maximum distance to calculate the K-Function for.

	Returns:
		An array of values for the K-Function. list[(d, K(d))]
	"""

def l_test(points, area, max_d) -> tuple[list,list]:
	"""
	Calculate the L-Function for the points in the area for a range of distances.
	The resulting distances are equally spaced from 0 to max_d.

	Arguments:
		points: The points to calculate the L-Function for. [n,2] ndarray.
		area: The area to calculate the L-Function for.
		max_d: The maximum distance to calculate the L-Function for.

	Returns:
		list[(d, L(d))]
	"""

def gibbs_strauss_process(
    fracture_intensity: float,
    hardcore_radius: float,
    acceptance_probability: float,
    region_size: tuple[float, float],
    max_iterations: int = None) -> list[tuple[float, float]]:
    """
    Simulates a Gibbs-Strauss process.

    Args:
        fracture_intensity (float): The number of fragments per unit area.
        hardcore_radius (float): The minimum distance between points.
        acceptance_probability (float): The probability of accepting a new point.
        region_size (Tuple[float, float]): The area in which to generate points.
        max_iterations (Optional[int]): The maximum number of iterations to perform.

    Returns:
        List[Tuple[float, float]]: A list of generated points, each represented as a tuple of two floats.
    """
    pass


def csstraussproc(rect_area: tuple[float, float], delta: float, n: int, c: float, max_iter: int) -> list[tuple[float, float]]:
    """
    Simulates a Strauss process.

    Args:
        rect_area (Tuple[float, float]): The area in which to generate points.
        delta (float): The minimum distance between points.
        n (int): The number of points to generate.
        c (float): The probability of accepting a new point.
        max_iter (int): The maximum number of iterations to perform.

    Returns:
        List[Tuple[float, float]]: A list of generated points, each represented as a tuple of two floats.
    """
    pass


def csstraussproc2(width: float, height: float, delta: float, n: int, c: float, i_max: int) -> list[tuple[float, float]]:
	pass