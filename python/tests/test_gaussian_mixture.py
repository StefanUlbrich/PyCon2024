import numpy as np
from sklearn.datasets import make_spd_matrix
from gaussian_mixtures import GaussianMixtureModel, expect, maximize, initialize, Likelihood
from scipy.stats import multivariate_normal as mvn


def test_expect() -> None:
    # generate random mvn, sample data, check whether correct responsibilities
    means = np.array([[10.0, 0.0], [0.0, 10.0]])
    covs = np.array([make_spd_matrix(2), make_spd_matrix(2)])
    weights = np.array([0.3, 0.7])
    n_samples = 100

    gmm = GaussianMixtureModel(means, covs, weights)
    for i in range(2):
        x = mvn(means[i], covs[i]).rvs(size=n_samples)
        assert np.all(expect(gmm, x).argmax(axis=1) == i)


def test_maximize() -> None:
    means = np.array([[20.0, 0.0], [0.0, 20.0]])
    covs = np.array([make_spd_matrix(2), make_spd_matrix(2)])
    weights = np.array([0.3, 0.7])

    n_samples = 1000
    x = np.vstack(tuple(mvn(means[i], covs[i]).rvs(size=int(n_samples * weights[i])) for i in range(2)))

    resp = np.array([[1.0, 0]] * int(n_samples * weights[0]) + [[0.0, 1.0]] * int(n_samples * weights[1]))

    gmm = initialize(x, 2, 0.1)
    maximize(gmm, Likelihood(resp), x)

    assert np.allclose(weights, gmm.weights)
    assert np.allclose(means, gmm.means, atol=3e-1)
    assert np.allclose(covs, gmm.covs, atol=3e-1)
