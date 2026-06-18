# External GitHub Lab: Using Polars DataFrame CLI
By the end of this lab, you will:
* Be able to use GitHub Codespaces for Rust development
* Understand how to work with Polars DataFrame CLI for data manipulation tasks
* Familiarize yourself with the core functionalities offered by the Polars library

## Pre-requisites
* A GitHub account
* Basic familiarity with Rust programming language
* Basic understanding of DataFrame operations

# Step 1: Fork and Open the Project in GitHub Codespaces
Fork the repository located at: 
https://github.com/nogibjj/rust-mlops-template/tree/main/polarsdf

Navigate to your forked repository on GitHub.

Click on the Code button near the top-right corner and then select Open with Codespaces.

Create a new Codespace instance.

# Step 2: Explore the Codebase
Once the Codespace is up and running, explore the repository's structure.

Open main.rs and lib.rs to understand the core functionalities and the CLI commands available.

# Step 3: Run the Application
Open the terminal in the Codespace environment.

Navigate to the polarsdf directory.

Run the project using Cargo with the following command:

```cargo run -- print --rows 5```

    ┌─────────────────────────────┬──────────────┬─────────────────────────────────────┬────────────────┬─────┬───────────┬───────────┬───────────┬──────┐
    │ Country Name                ┆ Country Code ┆ Indicator Name                      ┆ Indicator Code ┆ ... ┆ 2018      ┆ 2019      ┆ 2020      ┆ 2021 │
    │ ---                         ┆ ---          ┆ ---                                 ┆ ---            ┆     ┆ ---       ┆ ---       ┆ ---       ┆ ---  │
    │ str                         ┆ str          ┆ str                                 ┆ str            ┆     ┆ f64       ┆ f64       ┆ f64       ┆ str  │
    ╞═════════════════════════════╪══════════════╪═════════════════════════════════════╪════════════════╪═════╪═══════════╪═══════════╪═══════════╪══════╡
    │ Aruba                       ┆ ABW          ┆ Life expectancy at birth, total ... ┆ SP.DYN.LE00.IN ┆ ... ┆ 76.072    ┆ 76.248    ┆ 75.723    ┆ null │
    ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┤
    │ Africa Eastern and Southern ┆ AFE          ┆ Life expectancy at birth, total ... ┆ SP.DYN.LE00.IN ┆ ... ┆ 63.365858 ┆ 63.755674 ┆ 63.313856 ┆ null │
    ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┤
    │ Afghanistan                 ┆ AFG          ┆ Life expectancy at birth, total ... ┆ SP.DYN.LE00.IN ┆ ... ┆ 63.081    ┆ 63.565    ┆ 62.575    ┆ null │
    ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┤
    │ Africa Western and Central  ┆ AFW          ┆ Life expectancy at birth, total ... ┆ SP.DYN.LE00.IN ┆ ... ┆ 57.189139 ┆ 57.555796 ┆ 57.226373 ┆ null │
    ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┤
    │ Angola                      ┆ AGO          ┆ Life expectancy at birth, total ... ┆ SP.DYN.LE00.IN ┆ ... ┆ 62.144    ┆ 62.448    ┆ 62.261    ┆ null │
    └─────────────────────────────┴──────────────┴─────────────────────────────────────┴────────────────┴─────┴───────────┴───────────┴───────────┴──────┘


# Step 4: Examine DataFrame Operations
Use the following command to examine the DataFrame schema:

```cargo run -- Schema``` faisl use ```cargo run --schema```
```
    Schema:
    name: Country Name, data type: Utf8
    name: Country Code, data type: Utf8
    name: Indicator Name, data type: Utf8
    name: Indicator Code, data type: Utf8
    name: 1960, data type: Float64
    name: 1961, data type: Float64
    name: 1962, data type: Float64
    name: 1963, data type: Float64
    name: 1964, data type: Float64
    name: 1965, data type: Float64
    name: 1966, data type: Float64
    name: 1967, data type: Float64
    name: 1968, data type: Float64
    name: 1969, data type: Float64
    name: 1970, data type: Float64
    name: 1971, data type: Float64
    name: 1972, data type: Float64
    name: 1973, data type: Float64
    name: 1974, data type: Float64
    name: 1975, data type: Float64
    name: 1976, data type: Float64
    name: 1977, data type: Float64
    name: 1978, data type: Float64
    name: 1979, data type: Float64
    name: 1980, data type: Float64
    name: 1981, data type: Float64
    name: 1982, data type: Float64
    name: 1983, data type: Float64
    name: 1984, data type: Float64
    name: 1985, data type: Float64
    name: 1986, data type: Float64
    name: 1987, data type: Float64
    name: 1988, data type: Float64
    name: 1989, data type: Float64
    name: 1990, data type: Float64
    name: 1991, data type: Float64
    name: 1992, data type: Float64
    name: 1993, data type: Float64
    name: 1994, data type: Float64
    name: 1995, data type: Float64
    name: 1996, data type: Float64
    name: 1997, data type: Float64
    name: 1998, data type: Float64
    name: 1999, data type: Float64
    name: 2000, data type: Float64
    name: 2001, data type: Float64
    name: 2002, data type: Float64
    name: 2003, data type: Float64
    name: 2004, data type: Float64
    name: 2005, data type: Float64
    name: 2006, data type: Float64
    name: 2007, data type: Float64
    name: 2008, data type: Float64
    name: 2009, data type: Float64
    name: 2010, data type: Float64
    name: 2011, data type: Float64
    name: 2012, data type: Float64
    name: 2013, data type: Float64
    name: 2014, data type: Float64
    name: 2015, data type: Float64
    name: 2016, data type: Float64
    name: 2017, data type: Float64
    name: 2018, data type: Float64
    name: 2019, data type: Float64
    name: 2020, data type: Float64
    name: 2021, data type: Utf8
```

Use the following command to examine the DataFrame shape:

```cargo run -- shape```
```
    (266, 66)
```

Execute sorting operations:

```cargo run -- sort --year 2020 --rows 5 --order```
```
    ┌──────────────────────┬───────────┐
    │ Country Name         ┆ 2020      │
    │ ---                  ┆ ---       │
    │ str                  ┆ f64       │
    ╞══════════════════════╪═══════════╡
    │ Hong Kong SAR, China ┆ 85.387805 │
    ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
    │ Macao SAR, China     ┆ 85.184    │
    ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
    │ Japan                ┆ 84.61561  │
    ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
    │ Singapore            ┆ 83.743902 │
    ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
    │ Korea, Rep.          ┆ 83.426829 │
    └──────────────────────┴───────────┘
```
# Step 5: Complete the Exercises
Exercise 1: Modify the Sort command to sort based on any given column name, not just "year". Test your implementation.

Exercise 2: Add a new CLI command, Filter, to filter rows based on a given condition. For example, filter countries with life expectancy greater than 70 in the year 2020.

Reflection Question:
Reflect on the ways that data manipulation using Polars DataFrame in Rust compares to similar operations in Python's Pandas library. Consider aspects such as syntax, performance, and flexibility.

What advantages or limitations did you observe while working with Polars DataFrames in Rust?

How did the type safety in Rust impact your DataFrame operations, if at all?

How comfortable did you feel working in the Codespaces environment compared to a local development setup?

# Challenge Question:
Imagine you are tasked with integrating this Polars DataFrame CLI tool into a larger MLOps pipeline that includes data ingestion, preprocessing, training a machine learning model, and serving predictions.

What architecture would you propose to seamlessly integrate this CLI tool?

How would you address version control for the data, model, and the codebase?

Can you think of any performance or scaling challenges that might arise? How would you mitigate them?

Feel free to draw from your own experiences and previous projects to answer this question. Your answer can serve as an advanced discussion point for students who are looking for more challenging material.
