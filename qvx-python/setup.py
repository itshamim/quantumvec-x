from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="qvx",
    version="2.0.0",
    rust_extensions=[RustExtension("qvx.qvx_core", binding=Binding.PyO3)],
    packages=["qvx"],
    zip_safe=False,
)
