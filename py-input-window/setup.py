from setuptools import setup
from setuptools_rust import RustExtension


setup(
    name="py_input_window",
    version="0.1.0",
    packages=["py_input_window"],
    rust_extensions=[RustExtension("py_input_window.py_input_window", "Cargo.toml", debug=True)],
    include_package_data=True,
    zip_safe=False,
)
