from setuptools import setup, find_packages

setup(
    name="sstadex",
    version="0.1.0",
    description="A library for running MNA with SPICE file conversion",
    author="Your Name",
    packages=find_packages(),
    install_requires=[
        "numpy",  # Example of other dependencies
        "pandas",
        "sympy",
        "SymMNA @ git+https://github.com/lild4d4/Symbolic-modified-nodal-analysis.git@cc6bd3f568ddcef69173fc2499dae33547f1620d",
    ],
    python_requires=">=3.7",  # Specify compatible Python versions
)
