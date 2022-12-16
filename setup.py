# -*- coding: utf-8 -*-
from setuptools import setup, find_packages

with open("README.md") as f:
    README = f.read()

with open("LICENSE") as f:
    LICENSE = f.read()

with open("heistream/version.py") as f:
    __version__ = ""
    exec(f.read())  # set __version__

setup(
    name="heistream",
    version=__version__,
    description="Collector to gather information of interface",
    author="Stefan Machmeier",
    python_requires=">=3.9",
    author_email="stefan.machmeier@uni-heidelberg.de",
    maintainer="Stefan Machmeier",
    url="",
    long_description=README,
    include_package_data=True,
    packages=find_packages(),
    zip_safe=False,
    license=LICENSE,
    install_requires=[
        "nfstream==6.5.3",
        "psutil==5.9.4"
    ],
    extras_require={
        "cli": ["click==8.0.3", "click-help-colors==0.9.1"],
    },
    entry_points="""
      [console_scripts]
      heistream=cli.heistream:cli
    """,
)
