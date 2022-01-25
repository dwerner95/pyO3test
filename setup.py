from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
        name="py03test",
        version="0.1.0",
        packages=["py03test"],
        rust_extensions=[RustExtension("py03test.py03test",features=["python"], binding=Binding.PyO3)],
        # rust extensions are not zip safe, just like C-extensions.
        zip_safe=False,
)
