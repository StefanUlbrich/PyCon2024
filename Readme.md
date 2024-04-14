# Gaussian Mixtures

Tutorial session for PyCon/PyData Berlin 2024.

Uses a modern tech stack based with a lot of
opinionated choice:

* Python (`^3.12`)
  * [Pyenv](https://github.com/pyenv/pyenv)
  * [uv](https://github.com/astral-sh/uv)
  * [PoeThePoet](https://poethepoet.natn.io/index.html) (task runner)
  * [Ruff](https://github.com/astral-sh/ruff) (extremely fast linter)
  * [Mypy](https://mypy-lang.org/) (type checker)
  * Jupyterlab, matplotlib (`--extra lab`)
  * [Sphinx](https://www.sphinx-doc.org/en/master/),
    [pydata theme](https://github.com/pydata/pydata-sphinx-theme),
    and markdown via [myst-parser](https://myst-parser.readthedocs.io/en/latest/))
    (`--extra docs`)
  * [numpy](https://numpy.org/), [scipy](https://scipy.org/), [sklearn](https://scikit-learn.org/stable/)
  * Cuda: [cupy](https://cupy.dev/) (`-e cuda12x`, `-e cuda12x` (only cupy))
  * ~~[pytorch](https://pytorch.org/)~~ (while Python 3.12 is not [supported](https://github.com/openai/triton/issues/2707))

* Rust
  * [ndarray]() (equivalent of `numpy`)
  * [PyO3]() (Python bindings)
  * [cargo-show-asm](https://github.com/pacak/cargo-show-asm)

* Development environment (recommendations)
  * [Visual Studio Code](https://code.visualstudio.com/) or [VS Codium](https://vscodium.com/)
  * [Fira Code Font](https://github.com/tonsky/FiraCode) (e.g., `sudo apt install fonts-firacode`)

The repository is structured as follows:

* `python` sources for the pure python project
* `rust` sources for the optimized code in rust
* `bindings` sources for wrapping the rust code for python.

  This is separated from the `rust` crate as this is required for inline testing
* `pyproject.toml`, `requirements.txt` The definition of the python project and the locked virtual environment
* `Cargo.toml`, `Cargo.lock`: The definition of the workspace and locked dependencies

## Installation

### Python

I recommend installing with [PyEnv](https://github.com/pyenv/pyenv).

```sh
curl https://pyenv.run | bash
pyenv install -l # choose a recent version
pyenv install 3.12.0
```

Alternatively, you can use the system's version. On Debian-based Linux distributions, run

```sh
sudo apt install python3.12
```

### Virtual environment

#### Dependency handling with `uv`

First, install `uv` (or follow instructions on the [project page](https://github.com/astral-sh/uv?tab=readme-ov-file#getting-started))

```sh
curl -LsSf https://astral.sh/uv/install.sh | sh
```

Create environment and install the package

```sh
pyenv shell 3.12 # optional to select version when using pyenv
uv venv
uv pip compile pyproject.toml --all-extras -o requirements.txt
uv pip sync requirements.txt
. ./venv/bin/activate # Optional to activate the environment
```

You should select the newly created virtual environment in your development environment.

#### Tasks management with [`poethepoet`](https://github.com/nat-n/poethepoet)

In `pyproject.toml` there are multiple tasks defined that are available once you have the virtual
environment build and activated:

```sh
poe # will list all commands
poe lint # runs ruff to fix all auto-fixable issues
poe typing # runs the mypy type checker
poe fmt # runs ruff to format the source files
poe test # runs test and coverage
poe all # runs all of the above
poe install-kernel # makes the environment available in jupyter
poe uninstall-kernel # removes the associated jupyter kernel
poe verify # Checks whether cuda works as expected
poe docs # Builds the docs
poe lab # starts a jupyter service
```

#### Build documentation

```sh
poe docs
firefox docs/build/html/index.html # or similar
```

### Build Python binding

After calling the following command, changes in Python are effective immediately from now on
(restarting the interpreter might be required). Changes in Rust require a rebuild with the same
command.

```sh
uv pip install -e . -v -U
# or
poe maturin
```

### Rust

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update

cargo build
```

<!-- ```sh
cargo install cargo-criterion
cargo install cargo-show-asm
cargo asm -v --simplify --release --lib -p PROJECT LIBRARY::MODULE::FUNCTION 0 --rust
``` -->
