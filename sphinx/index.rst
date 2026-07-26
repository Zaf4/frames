Framex |version|
====================

|pypi| |license| |polars|

Framex is a light-weight dataset fetching library and CLI tool. 
It is built with minimal dependency requirements and a simple syntax, minimizing overhead and maximizing convenience.

Installation
--------------
.. tab-set::

   .. tab-item:: pip
      :sync: pip

      .. code-block:: bash

         pip install framex

   .. tab-item:: uv
      :sync: uv

      .. code-block:: bash

         uv add framex

   .. tab-item:: poetry
      :sync: poetry

      .. code-block:: bash

         poetry add framex

   .. tab-item:: pipx
      :sync: pipx

      .. code-block:: bash

         pipx install framex


Rust CLI
^^^^^^^^

The standalone Rust ``fx`` CLI can be installed without Python.

.. tab-set::

   .. tab-item:: Linux and macOS
      :sync: rust-unix

      .. code-block:: bash

         curl -LsSf https://framex.datavil.org/install.sh | sh

   .. tab-item:: Windows PowerShell
      :sync: rust-windows

      .. code-block:: powershell

         powershell -ExecutionPolicy ByPass -c "irm https://framex.datavil.org/install.ps1 | iex"


.. toctree::
   :maxdepth: 1
   :hidden:

   API
   CLI



Getting Started
---------------
Framex allows the direct imports of available datasets.

.. jupyter-execute::

   from framex import iris

   iris.head()

or loading it using the functional approach.

.. jupyter-execute::

   import framex as fx 

   mpg = fx.load("mpg")
   mpg.head()

To see (all/local/remote) available datasets.
The function also accepts positional "includes" argument for searching.

.. jupyter-execute::

   import framex as fx

   both = fx.available()
   local = fx.available(option="local")
   remote = fx.available(option="remote") 

   fx.available("st")


.. |pypi| image:: https://img.shields.io/pypi/v/framex?color=377eb8
   :target: https://pypi.org/project/framex/
   :alt: PyPI version

.. |license| image:: https://img.shields.io/badge/License-Apache%202.0-ff0000
   :target: https://opensource.org/licenses/Apache-2.0
   :alt: License: Apache 2.0

.. |polars| image:: https://img.shields.io/badge/Powered%20by-Polars-377eb8?logo=polars&logoColor=white
   :target: https://www.pola.rs/
   :alt: Powered by Polars
