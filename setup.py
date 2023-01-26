#!/usr/bin/env python3

import os
from setuptools import setup


def read(fname):
    return open(os.path.join(os.path.dirname(__file__), fname)).read()


with open("heidpi/version.py") as f:
    __version__ = ""
    exec(f.read())  # set __version__

setup(
    name="heidpi",
    version="0.1.1",
    author="Stefan Machmeier",
    license=read("LICENSE"),
    author_email="stefan-machmeier@outlook.com",
    description="Import and Export passwords for Firefox",
    long_description=read("README.md"),
    long_description_content_type="text/markdown",
    url="https://github.com/stefanDeveloper/heidpi",
    packages=["heidpi"],
    install_requires=["confuse"],
    python_requires=">=3.6",
    entry_points={"console_scripts": ["heidpi = heidpi:main"]},
    classifiers=["Topic :: Utilities", "Topic :: Security :: Cryptography"],
)
