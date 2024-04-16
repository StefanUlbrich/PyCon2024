"""A very basic, educative implementation of Gaussian mixture models"""

from dataclasses import dataclass
from typing import NewType

import numpy as np
from numpy.typing import NDArray

from . import gaussian_mixtures as bindings  # type: ignore  # noqa: F401

Likelihood = NewType("Likelihood", NDArray[np.float64])


@dataclass
class GaussianMixtureModel:
    """A mixture model"""

    means: NDArray[np.float64]
    covs: NDArray[np.float64]
    weights: NDArray[np.float64]


def initialize(data: NDArray[np.float64], n_components: int, alpha: float = 1.0) -> GaussianMixtureModel:
    """Generate random responsibilities for intialization"""
    raise NotImplementedError()


def expect(gmm: GaussianMixtureModel, data: NDArray[np.float64]) -> Likelihood:
    """Expectation step"""
    raise NotImplementedError()


def maximize(gmm: GaussianMixtureModel, responsibilities: Likelihood, data: NDArray[np.float64]) -> None:
    """Maximization step"""
    raise NotImplementedError()


if __name__ == "__main__":
    ...
