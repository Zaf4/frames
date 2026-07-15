[![Banner](https://github.com/datavil/framex/blob/master/.github/framex_banner_narrower.png?raw=true)](https://framex.datavil.org)
A [Datavil](https://datavil.org) project.

# FrameX

[![GitHub](https://img.shields.io/badge/GitHub-100000?style=flat&logo=github&logoColor=white)](https://github.com/DataVil/framex) [![PyPI](https://img.shields.io/pypi/v/framex?color=blue)](https://pypi.org/project/framex/)

**FrameX** is a light-weight, dataset fetching library for fast **prototyping**, **tutorial creation**, and **experimenting**. FrameX has currently over **80** datasets available.


Built on top of [Polars](https://pola.rs/).




## Installation

Install the Python library with:

``` shell
pip install framex
```

Or install the standalone Rust CLI on Linux or macOS Apple Silicon:

``` shell
curl -LsSf https://raw.githubusercontent.com/datavil/framex/master/install.sh | sh
```

## Usage

### Python

``` python
import framex as fx
```

#### Loading datasets

``` python
iris = fx.load("iris")
```

is equivalent to

``` python
from framex import iris
```

which returns a [**polars DataFrame**](https://docs.pola.rs/api/python/stable/reference/dataframe/index.html)\
Therefore, you can use all the **polars** functions and methods on the returned **DataFrame**.

``` python
iris.head()
```

``` text
shape: (5, 5)
┌──────────────┬─────────────┬──────────────┬─────────────┬─────────┐
│ sepal_length ┆ sepal_width ┆ petal_length ┆ petal_width ┆ species │
│ ---          ┆ ---         ┆ ---          ┆ ---         ┆ ---     │
│ f32          ┆ f32         ┆ f32          ┆ f32         ┆ str     │
╞══════════════╪═════════════╪══════════════╪═════════════╪═════════╡
│ 5.1          ┆ 3.5         ┆ 1.4          ┆ 0.2         ┆ setosa  │
│ 4.9          ┆ 3.0         ┆ 1.4          ┆ 0.2         ┆ setosa  │
│ 4.7          ┆ 3.2         ┆ 1.3          ┆ 0.2         ┆ setosa  │
│ 4.6          ┆ 3.1         ┆ 1.5          ┆ 0.2         ┆ setosa  │
│ 5.0          ┆ 3.6         ┆ 1.4          ┆ 0.2         ┆ setosa  │
└──────────────┴─────────────┴──────────────┴─────────────┴─────────┘
```

``` python
iris = fx.load("iris", lazy=True)
```

which returns a [**polars LazyFrame**](https://docs.pola.rs/api/python/stable/reference/lazyframe/index.html)

Both these operations create local copies of the datasets by default `cache=True`.

#### Available datasets

To see the list of available datasets, run:

``` python
fx.available()
```


``` python
{'remote': ['iris', 'mpg', 'netflix', 'starbucks', 'titanic'], 'local': ['titanic']}
```
PS, shorthened for clarity

which returns a dictionary of both **locally** and **remotely** available datasets.

To see only **local** or **remote** datasets, run:

``` python
fx.available("local")
fx.available("remote")
```

``` python
{'local': ['titanic']}
{'remote': ['iris', 'mpg', 'netflix', 'starbucks', 'titanic']}
```

#### Getting information on Datasets

To get information on a dataset, run:

``` python
fx.about("mpg") # basically the same as `fx.about("mpg", mode="print")`
```

which will print the information on the dataset as the following:

``` text
NAME    : mpg
SOURCE  : https://www.kaggle.com/datasets/uciml/autompg-dataset
LICENSE : CC0: Public Domain
ORIGIN  : Kaggle
OG NAME : autompg-dataset
```

Or you can get the information as a single row polars.DataFrame by running:

``` python
row = fx.about("mpg", mode="row")
print(row)
```

which will print the information on the dataset **ASCII art** as the following:

``` text
shape: (1, 4)
┌──────┬─────────────────────────────────┬────────────────────┬────────┐       
│ name ┆ source                          ┆ license            ┆ origin │       
│ ---  ┆ ---                             ┆ ---                ┆ ---    │       
│ str  ┆ str                             ┆ str                ┆ str    │       
╞══════╪═════════════════════════════════╪════════════════════╪════════╡       
│ mpg  ┆ https://www.kaggle.com/dataset… ┆ CC0: Public Domain ┆ Kaggle │       
└──────┴─────────────────────────────────┴────────────────────┴────────┘ 
```

or you can simply treat `row` as a polars DataFrame in your code.

#### Getting Dataset URLs

In case you need the file links.

``` python
url_pokemon = fx.get_url("pokemon")
```

by default, the format is " feather".

Optionally, you can specify the format of the dataset.

``` python
url_pokemon_csv = fx.get_url("pokemon", format="csv")
```

### CLI

framex CLI has a slight overhead of around 400 milliseconds due to imports. However, operations still take less than a second, unless bottlenecked by the download speed. 

TO see all the available commands, run:
``` shell
fx -h
```

![Banner](https://github.com/datavil/framex/blob/master/.github/mainCLI.png?raw=true)


#### get

Get a single dataset (to the current directory):

``` shell
fx get iris
```

or get multiple datasets:

``` shell
fx get iris mpg titanic
```

which will download dataset(s) to the current directory.

to get the datasets into cache directory:

``` shell
fx get iris mpg titanic --cache
```

or to a specific directory:

``` shell
fx get iris mpg titanic --dir data
```

#### list

To get the name of the available datasets on the **remote server**.

``` shell
fx list
```

this will list all available datasets on the remote server.


to get the names of the available datasets that includes "dia"
``` shell
fx list dia
```

``` shell	
Locally available datasets: (feather, parquet, csv, other)

Remote datasets:
diamonds
```

#### about

To get information on a dataset or datasets, run:

``` shell
fx about mpg iris
```

#### show

To show a preview of a single dataset

``` shell
fx show iris
```

#### describe

To describe (or summarize) a dataset

``` shell
fx describe iris
```

For more parameters

``` shell
fx get --help
```

#### bring

Bring a dataset to the current directory from cache:

``` shell
fx bring iris
```

or bring multiple datasets:

``` shell
fx bring iris mpg titanic
```

which will bring dataset(s) to the current directory from cache directory.
