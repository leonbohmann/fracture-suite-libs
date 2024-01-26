import numpy as np

def  calculate_fracture_surface(
    contours: list,
    image: np.ndarray,
    thickness: float,
    px_p_mm: float
) -> float:
    """
    Calculate the fracture surface using the underlying preprocessed image.

    Args:
        contours (list): List of contours on the image.
        image (np.ndarray): The background image in Grayscale!
        thickness (float): The thickness in mm.
        px_p_mm (float): Conversion between pixel and mm.

    Returns:
        float: The total fracture surface in mm^2.
    """