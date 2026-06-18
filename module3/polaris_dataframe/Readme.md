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

```cargo run -- sort --sort-by "Country Code" --rows 5 --order```

```
    ┌──────────────┬──────────────┬─────────────────────────────────────┬────────────────┬─────┬────────┬────────┬────────┬──────┐
    │ Country Name ┆ Country Code ┆ Indicator Name                      ┆ Indicator Code ┆ ... ┆ 2018   ┆ 2019   ┆ 2020   ┆ 2021 │
    │ ---          ┆ ---          ┆ ---                                 ┆ ---            ┆     ┆ ---    ┆ ---    ┆ ---    ┆ ---  │
    │ str          ┆ str          ┆ str                                 ┆ str            ┆     ┆ f64    ┆ f64    ┆ f64    ┆ str  │
    ╞══════════════╪══════════════╪═════════════════════════════════════╪════════════════╪═════╪════════╪════════╪════════╪══════╡
    │ Zimbabwe     ┆ ZWE          ┆ Life expectancy at birth, total ... ┆ SP.DYN.LE00.IN ┆ ... ┆ 61.414 ┆ 61.292 ┆ 61.124 ┆ null │
    ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┤
    │ Zambia       ┆ ZMB          ┆ Life expectancy at birth, total ... ┆ SP.DYN.LE00.IN ┆ ... ┆ 62.342 ┆ 62.793 ┆ 62.38  ┆ null │
    ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┤
    │ South Africa ┆ ZAF          ┆ Life expectancy at birth, total ... ┆ SP.DYN.LE00.IN ┆ ... ┆ 65.674 ┆ 66.175 ┆ 65.252 ┆ null │
    ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┤
    │ Yemen, Rep.  ┆ YEM          ┆ Life expectancy at birth, total ... ┆ SP.DYN.LE00.IN ┆ ... ┆ 64.575 ┆ 65.092 ┆ 64.65  ┆ null │
    ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┤
    │ Kosovo       ┆ XKX          ┆ Life expectancy at birth, total ... ┆ SP.DYN.LE00.IN ┆ ... ┆ 78.696 ┆ 79.022 ┆ 76.567 ┆ null │
    └──────────────┴──────────────┴─────────────────────────────────────┴────────────────┴─────┴────────┴────────┴────────┴──────┘
```
Exercise 2: Add a new CLI command, Filter, to filter rows based on a given condition. For example, filter countries with life expectancy greater than 70 in the year 2020.

```cargo run -- filter --by-col 2020 --op gt --value 70 --rows 5```

```
    ┌──────────────────────┬──────────────┬─────────────────────────────────────┬────────────────┬─────┬───────────┬───────────┬──────────┬──────┐
    │ Country Name         ┆ Country Code ┆ Indicator Name                      ┆ Indicator Code ┆ ... ┆ 2018      ┆ 2019      ┆ 2020     ┆ 2021 │
    │ ---                  ┆ ---          ┆ ---                                 ┆ ---            ┆     ┆ ---       ┆ ---       ┆ ---      ┆ ---  │
    │ str                  ┆ str          ┆ str                                 ┆ str            ┆     ┆ f64       ┆ f64       ┆ f64      ┆ str  │
    ╞══════════════════════╪══════════════╪═════════════════════════════════════╪════════════════╪═════╪═══════════╪═══════════╪══════════╪══════╡
    │ Aruba                ┆ ABW          ┆ Life expectancy at birth, total ... ┆ SP.DYN.LE00.IN ┆ ... ┆ 76.072    ┆ 76.248    ┆ 75.723   ┆ null │
    ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┤
    │ Albania              ┆ ALB          ┆ Life expectancy at birth, total ... ┆ SP.DYN.LE00.IN ┆ ... ┆ 79.184    ┆ 79.282    ┆ 76.989   ┆ null │
    ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┤
    │ Arab World           ┆ ARB          ┆ Life expectancy at birth, total ... ┆ SP.DYN.LE00.IN ┆ ... ┆ 71.633017 ┆ 71.844626 ┆ 70.92336 ┆ null │
    ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┤
    │ United Arab Emirates ┆ ARE          ┆ Life expectancy at birth, total ... ┆ SP.DYN.LE00.IN ┆ ... ┆ 79.627    ┆ 79.726    ┆ 78.946   ┆ null │
    ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┤
    │ Argentina            ┆ ARG          ┆ Life expectancy at birth, total ... ┆ SP.DYN.LE00.IN ┆ ... ┆ 76.999    ┆ 77.284    ┆ 75.892   ┆ null │
    └──────────────────────┴──────────────┴─────────────────────────────────────┴────────────────┴─────┴───────────┴───────────┴──────────┴──────┘
```
```cargo run -- filter --by-col 2020 --op le --value 70 --rows 5```

```
    ┌─────────────────────────────┬──────────────┬─────────────────────────────────────┬────────────────┬─────┬───────────┬───────────┬───────────┬──────┐
    │ Country Name                ┆ Country Code ┆ Indicator Name                      ┆ Indicator Code ┆ ... ┆ 2018      ┆ 2019      ┆ 2020      ┆ 2021 │
    │ ---                         ┆ ---          ┆ ---                                 ┆ ---            ┆     ┆ ---       ┆ ---       ┆ ---       ┆ ---  │
    │ str                         ┆ str          ┆ str                                 ┆ str            ┆     ┆ f64       ┆ f64       ┆ f64       ┆ str  │
    ╞═════════════════════════════╪══════════════╪═════════════════════════════════════╪════════════════╪═════╪═══════════╪═══════════╪═══════════╪══════╡
    │ Africa Eastern and Southern ┆ AFE          ┆ Life expectancy at birth, total ... ┆ SP.DYN.LE00.IN ┆ ... ┆ 63.365858 ┆ 63.755674 ┆ 63.313856 ┆ null │
    ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┤
    │ Afghanistan                 ┆ AFG          ┆ Life expectancy at birth, total ... ┆ SP.DYN.LE00.IN ┆ ... ┆ 63.081    ┆ 63.565    ┆ 62.575    ┆ null │
    ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┤
    │ Africa Western and Central  ┆ AFW          ┆ Life expectancy at birth, total ... ┆ SP.DYN.LE00.IN ┆ ... ┆ 57.189139 ┆ 57.555796 ┆ 57.226373 ┆ null │
    ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┤
    │ Angola                      ┆ AGO          ┆ Life expectancy at birth, total ... ┆ SP.DYN.LE00.IN ┆ ... ┆ 62.144    ┆ 62.448    ┆ 62.261    ┆ null │
    ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┤
    │ Azerbaijan                  ┆ AZE          ┆ Life expectancy at birth, total ... ┆ SP.DYN.LE00.IN ┆ ... ┆ 72.76     ┆ 73.102    ┆ 66.868    ┆ null │
    └─────────────────────────────┴──────────────┴─────────────────────────────────────┴────────────────┴─────┴───────────┴───────────┴───────────┴──────┘
```

* Added new commands

Generic syntax:

`cargo run -- sort --sort-by <COLUMN_NAME> --rows <N> --order`

`cargo run -- sort --year <COLUMN_NAME> --rows <N> --order`

`cargo run -- filter --by-col <COLUMN_NAME> --op <OPERATOR> --value <VALUE> --rows <N>`

Filter operators:

`gt` greater than, `ge` greater than or equal, `lt` less than, `le` less than or equal, `eq` equal, `ne` not equal

Examples:

`cargo run -- sort --sort-by "Country Code" --rows 5 --order`

`cargo run -- sort --year 2020 --rows 5 --order`

`cargo run -- filter --by-col 2020 --op gt --value 70 --rows 5`

`cargo run -- filter --by-col 2020 --op le --value 70 --rows 5`


Reflection Question:
* Reflect on the ways that data manipulation using Polars DataFrame in Rust compares to similar operations in Python's Pandas library. Consider aspects such as syntax, performance, and flexibility.

    Polars in Rust feels more explicit and strongly typed than Pandas. In Pandas, many operations are quick to write and very convenient for ad-hoc exploration, but they can hide type conversion issues until runtime. In Rust with Polars, I had to be more deliberate about data types, error handling, and operation flow. That made the code a bit more verbose, but also more predictable and safer once compiled.

    From a performance perspective, Polars generally felt fast and memory-efficient for tabular operations like sorting and filtering. The Rust implementation also gives confidence for production-oriented workloads where reliability and throughput matter. In contrast, Pandas is extremely flexible for rapid experimentation, but can require more care for large datasets and repeated pipelines.

* What advantages or limitations did you observe while working with Polars DataFrames in Rust?

    Advantages:
    - Strong type safety and compile-time checks reduced accidental runtime failures.
    - Clear, explicit transformations made behavior easier to reason about.
    - Good performance for filtering/sorting operations.

    Limitations:
    - More boilerplate than Pandas for common data cleaning tasks.
    - API/version differences can require adjustments (for example, handling method return types across versions).
- The learning curve is higher if coming from a purely scripting workflow.

* How did the type safety in Rust impact your DataFrame operations, if at all?

    Type safety had a strong impact. It forced me to handle invalid column names, unsupported operations, and type conversions directly instead of assuming permissive behavior. This added development effort up front, but produced safer CLI behavior with clearer error messages and fewer surprises during execution.

* How comfortable did you feel working in the Codespaces environment compared to a local development setup?

    Codespaces felt very convenient for setup and reproducibility. I could start coding quickly with dependencies already available and avoid local environment issues. For this lab, it was comfortable and productive. A local setup can still be preferable for custom tooling, offline work, or lower latency, but Codespaces was an excellent fit for guided exercises and collaboration.

# Challenge Question:
Imagine you are tasked with integrating this Polars DataFrame CLI tool into a larger MLOps pipeline that includes data ingestion, preprocessing, training a machine learning model, and serving predictions.

* What architecture would you propose to seamlessly integrate this CLI tool?

  I would use a modular pipeline architecture where this CLI tool is the data preparation component:
  1. Data ingestion stage loads raw files from object storage (for example S3 or GCS) into a staging area.
  2. Preprocessing stage runs this Polars CLI (`sort`, `filter`, and future transformations) to produce clean, versioned feature tables.
  3. Training stage consumes those prepared tables to train models and logs metrics/artifacts.
  4. Model registry stores approved model versions with metadata.
  5. Serving stage loads the approved model and uses the same preprocessing contract to avoid train/serve skew.
  6. Orchestration layer (for example Airflow, Prefect, or GitHub Actions for simpler flows) schedules and monitors each step.

  In practice, I would package this CLI as a container image and run it as a reusable pipeline step so preprocessing remains consistent across environments.

* How would you address version control for the data, model, and the codebase?

  I would version all three layers explicitly:
  1. Code: Git with pull requests, code review, semantic tagging, and CI tests.
  2. Data: data versioning via DVC, lakeFS, or Delta/Iceberg snapshots; each training run stores the exact dataset version/hash.
  3. Model: registry-based versioning (MLflow Model Registry, SageMaker Model Registry, or similar) with lineage metadata linking model version to code commit and data snapshot.

  Each experiment/run should capture:
  - Git commit SHA
  - Data snapshot ID
  - CLI command/config used for preprocessing
  - Model artifact URI and metrics

  This gives full reproducibility and supports reliable rollback.

* Can you think of any performance or scaling challenges that might arise? How would you mitigate them?

  Common challenges and mitigations:
  1. Large dataset memory pressure:
      Use lazy execution, chunked processing, and column pruning (read only needed columns).
  2. Repeated full recomputation:
      Add partition-aware processing and incremental updates by date or batch ID.
  3. Single-node bottlenecks:
      Move heavy transformations to distributed engines when needed, or parallelize preprocessing jobs by partition.
  4. Inconsistent preprocessing between training and inference:
      Reuse the same transformation definitions/config and validate schemas with automated checks.
  5. Pipeline failures and observability gaps:
      Add structured logs, metrics, retry policies, and data quality checks (null thresholds, range checks, schema drift alerts).

* Feel free to draw from your own experiences and previous projects to answer this question. Your answer can serve as an advanced discussion point for students who are looking for more challenging material.

  A practical lesson from production-style pipelines is that consistency and traceability matter more than any single tool choice. Even a fast preprocessing tool can become a source of bugs if data versions, feature logic, and model artifacts are not linked together. The best long-term design is one where every prediction can be traced back to the exact code, data snapshot, and model version that produced it.
