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