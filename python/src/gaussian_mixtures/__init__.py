"""A very basic, educative implementation of Gaussian mixture models"""

from dataclasses import dataclass
from typing import NewType

import numpy as np
from numpy.typing import NDArray
from scipy.stats import multivariate_normal as mvn

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

    responsibilities: Likelihood = Likelihood(np.random.dirichlet(n_components * [alpha], data.shape[0]))

    gmm = GaussianMixtureModel(np.zeros(0), np.zeros(0), np.zeros(0))
    maximize(gmm, responsibilities, data)
    return gmm


def expect(gmm: GaussianMixtureModel, data: NDArray[np.float64]) -> Likelihood:
    """Expectation step."""

    res = np.array([w * mvn.pdf(data, mean=m, cov=c) for m, c, w in zip(gmm.means, gmm.covs, gmm.weights)])
    res /= res.sum(axis=0)
    return Likelihood(res.T)
    # return Likelihood(res)


def maximize(gmm: GaussianMixtureModel, responsibilities: Likelihood, data: NDArray[np.float64]) -> None:
    """Maximization step"""

    assert responsibilities.shape[0] == data.shape[0]

    sum_responsibilities = responsibilities.sum(axis=0)

    gmm.means = (
        np.sum(data[:, np.newaxis, :] * responsibilities[:, :, np.newaxis], axis=0)
        / sum_responsibilities[:, np.newaxis]
    )

    data = data[:, np.newaxis, :] - gmm.means[np.newaxis, :, :]

    gmm.covs = (
        np.einsum("nkd, nk,  nke -> kde", data, responsibilities, data)
        / sum_responsibilities[:, np.newaxis, np.newaxis]
    )

    gmm.weights = sum_responsibilities / sum_responsibilities.sum()


if __name__ == "__main__":
    ...
