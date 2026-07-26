import os
import sys
import unittest.mock as mock
from datetime import datetime
from importlib.metadata import version as get_version

# Add the project root to sys.path
sys.path.insert(0, os.path.abspath(".."))  # noqa: PTH100
templates_path = [os.path.join(os.path.abspath("."), "_templates")]  # noqa: PTH100, PTH118

# frame must be installed in the env
version = get_version("framex")
release = get_version("framex")
rst_prolog = """
.. role:: gray
   :class: gray
"""
rst_epilog = f".. |version| replace:: :gray:`v{release}`"

os.environ["SPHINX_BUILD"] = "1"


class BetterMock(mock.MagicMock):
    def __add__(self, other):
        return self

    def __radd__(self, other):
        return self

    def __call__(self, *args, **kwargs):
        return self

    def __bool__(self):
        return True

    def __or__(self, other):
        return self

    def __ror__(self, other):
        return self


# Explicitly set __bool__ on the class to prevent MagicMock from overriding it
# BetterMock.__bool__ = lambda self: True

autodoc_mock_imports = [
    "polars",
]

# -- Project information -----------------------------------------------------

project = "framex"
copyright = f"{datetime.now().year}, datavil"
author = "Zaf4"

# -- General configuration ---------------------------------------------------

extensions = [
    "sphinx.ext.autodoc",
    "sphinx.ext.napoleon",
    "sphinx.ext.intersphinx",
    "sphinx.ext.autosummary",
    "jupyter_sphinx",
    "sphinx.ext.githubpages",
    "sphinx_copybutton",
    "sphinx_design",
    "sphinxcontrib.autoprogram",
]


autosummary_generate = True
add_module_names = False
modindex_common_prefix = ["framex."]
# Autodoc settings
autodoc_default_options = {
    "members": True,
    "undoc-members": True,
    "show-inheritance": True,
    "member-order": "bysource",
}
autodoc_preserve_defaults = True
autodoc_typehints_format = "short"


# -- Options for HTML output -------------------------------------------------
html_show_sourcelink = False
html_title = ""
html_theme = "pydata_sphinx_theme"
html_static_path = ["_static"]
html_css_files = ["custom.css"]
html_js_files = ["custom.js"]

html_theme_options = {
    "logo": {
        "text": "Framex",
    },
    "default_mode": "light",
    "light_dark_mode_buttons": ["light", "dark"],
    "icon_links": [
        {
            "name": "GitHub",
            "url": "https://github.com/datavil/framex",
            "icon": "fa-brands fa-github",
            "type": "fontawesome",
        },
        {
            "name": "PyPI",
            "url": "https://pypi.org/project/framex",
            "icon": "_static/pypi.svg",
            "type": "local",
        },
    ],
    "navbar_start": ["navbar-logo"],
    "navbar_center": ["navbar-nav"],
    "navbar_end": ["theme-switcher", "navbar-icon-links"],
    "navbar_align": "left",
    "search_bar_text": "Search",
    "pygments_light_style": "manni",
    "pygments_dark_style": "material",
    "show_nav_level": 0,
    "use_edit_page_button": False,
    "announcement": None,
    "show_prev_next": False,
    "footer_start": ["copyright"],
    "footer_center": ["sphinx-version"],
    "footer_end": ["theme-version"],
}
html_favicon = "_static/framex-logo.svg"

html_sidebars = {
    "**": [],  # no search, links, etc. on any page
}

# Napoleon settings
napoleon_google_docstring = True
napoleon_numpy_docstring = True
napoleon_use_param = True
napoleon_use_rtype = False
napoleon_preprocess_types = True
napoleon_type_aliases = None
napoleon_attr_annotations = True

napoleon_type_aliases = {
    "PlotSpec": ":py:class:`~lets_plot.plot.core.PlotSpec`",
    "DataFrame": ":doc:`polars:reference/dataframe/index`",
}

python_use_unqualified_type_names = True

intersphinx_mapping = {
    "polars": ("https://docs.pola.rs/api/python/stable/", None),
}
