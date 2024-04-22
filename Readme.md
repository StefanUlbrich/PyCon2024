# Gaussian Mixtures

Repository for the tutorial session "Performant, scientific computation in Python and Rust" at PyCon/PyData Berlin 2024.

> **To the participants of the tutorial session:**
>
> All stages of the tutorial can be reproduced by checking out different branches of this repository
> and the slides are written in HTML and will be hosted on GitHub pages shortly before the session so
> that you can see them in our browser.
>
> **Note: If you want to reproduce the tutorial on your device, please follow the installation steps
> described below prior to the session to avoid longer delays during at the beginning.
> Thank you very much!**
>
> I use [`uv`](https://github.com/astral-sh/uv) for dependency handling but you can use
> `pip-tools` or plain pip / virtual environments instead.

> **To the participants of the tutorial session:**
>
> **Please follow the installation steps described below prior to the session
> to avoid longer delays during at the beginning. Thank you very much!**

Uses a modern tech stack based with a lot of
(opinionated) choices:

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
  * [pre-commit](https://pre-commit.com/) hooks

* Rust
  * [ndarray](https://github.com/rust-ndarray/ndarray) (equivalent of `numpy`)
  * [PyO3](https://github.com/PyO3/pyo3) (Python bindings)
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

Alternatively, you can use the system's version of course. On macOS using `brew` is recommended, while on Debian-based Linux distributions, run

```sh
sudo apt install python3.12
```

I have little experience with Windows systems, but you should probably use WSL2 there.

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
# The next line needs only be called if dependencies are altered
uv pip compile pyproject.toml --all-extras -o requirements.txt
uv pip sync requirements.txt
. ./venv/bin/activate # Optional to activate the environment
```

Alternatively, you should be able to achieve the same with plain `pip`

```sh
python3.12 -m venv .venv
. ./venv/bin/activate # Optional to activate the environment
pip install -r requirements.txt
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

#### Installing pre-commit GIT hooks

Activate the virtual environment and run

```sh
pre-commit install
```

Then, all modified sources will be checked for formatting and all tests will be on every commit
unless if called with the `-n` (no-verify) switch. The virtual environment needs to be activated
for this. If you run into problems with your IDE, please install pre-commit globally for your user.

### Build the Python bindings to the Rust code

After calling the following command, changes in Python are effective immediately from now on
(restarting the interpreter might be required). Changes in Rust require a rebuild with the same
command.

```sh
uv pip install -e . -v -U # for an editable install
# or
poe maturin               # for building a wheel
```

### Rust

First [install Rust](https://forge.rust-lang.org/infra/other-installation-methods.html). This is usually done
with the `rustup` tool that manages Rust installations and distribution of tools. On a Unix-like OS, run

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update
```

You can check your installation by building the project. The command used for this is
`cargo` a build system (comparable to `poetry` or `rye` in the Python world):

```sh
cargo build
```

You should also install the following add-ons to the `cargo` build system:

```sh
cargo install cargo-criterion
cargo install cargo-show-asm
```

<!-- ```sh
cargo asm -v --simplify --release --lib -p PROJECT LIBRARY::MODULE::FUNCTION 0 --rust
```-->
