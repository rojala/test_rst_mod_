// utilities for working with polars dataframes
//
use polars::prelude::*;

//read in a csv file
pub fn read_csv(path: &str) -> DataFrame {
    CsvReader::from_path(path).unwrap().finish().unwrap()
}

//print "n" rows of a dataframe
pub fn print_df(df: &DataFrame, n: usize) {
    println!("{:?}", df.head(Some(n)));
}

//print the schema of a dataframe
pub fn print_schema(df: &DataFrame) {
    println!("{:?}", df.schema());
}

//print the shape of a dataframe
pub fn print_shape(df: &DataFrame) {
    println!("{:?}", df.shape());
}

pub fn sort_by_column(df: &DataFrame, by_col: &str, order: bool) -> Result<DataFrame, String> {
    let available_columns = df.get_column_names();
    if !available_columns.iter().any(|name| name == &by_col) {
        return Err(format!(
            "Unknown column '{}'. Available columns: {}",
            by_col,
            available_columns.join(", ")
        ));
    }

    Ok(df.sort([by_col], order).unwrap())
}

pub fn filter_by_condition(
    df: &DataFrame,
    by_col: &str,
    op: &str,
    value: &str,
) -> Result<DataFrame, String> {
    let available_columns = df.get_column_names();
    if !available_columns.iter().any(|name| name == &by_col) {
        return Err(format!(
            "Unknown column '{}'. Available columns: {}",
            by_col,
            available_columns.join(", ")
        ));
    }

    let series = df.column(by_col).map_err(|e| e.to_string())?;

    let mask = match series.dtype() {
        DataType::Utf8 => {
            let utf8_col = series
                .utf8()
                .map_err(|e| format!("Column '{}' cannot be treated as Utf8: {}", by_col, e))?;
            match op {
                "eq" => utf8_col.equal(value),
                "ne" => utf8_col.not_equal(value),
                _ => {
                    return Err(format!(
                        "Unsupported operator '{}' for Utf8 column '{}'. Use: eq, ne",
                        op, by_col
                    ));
                }
            }
        }
        DataType::Float64
        | DataType::Float32
        | DataType::Int64
        | DataType::Int32
        | DataType::Int16
        | DataType::Int8
        | DataType::UInt64
        | DataType::UInt32
        | DataType::UInt16
        | DataType::UInt8 => {
            let numeric_value = value.parse::<f64>().map_err(|_| {
                format!(
                    "Value '{}' is not numeric but column '{}' is numeric",
                    value, by_col
                )
            })?;

            let casted = series
                .cast(&DataType::Float64)
                .map_err(|e| format!("Failed to cast column '{}' to Float64: {}", by_col, e))?;
            let numeric_col = casted
                .f64()
                .map_err(|e| {
                    format!("Column '{}' cannot be treated as Float64: {}", by_col, e)
                })?;

            match op {
                "gt" => numeric_col.gt(numeric_value),
                "ge" => numeric_col.gt_eq(numeric_value),
                "lt" => numeric_col.lt(numeric_value),
                "le" => numeric_col.lt_eq(numeric_value),
                "eq" => numeric_col.equal(numeric_value),
                "ne" => numeric_col.not_equal(numeric_value),
                _ => {
                    return Err(format!(
                        "Unsupported operator '{}' for numeric column '{}'. Use: gt, ge, lt, le, eq, ne",
                        op, by_col
                    ));
                }
            }
        }
        _ => {
            return Err(format!(
                "Unsupported column type '{:?}' for '{}'",
                series.dtype(),
                by_col
            ));
        }
    };

    df.filter(&mask).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_by_requested_column() {
        let df = DataFrame::new(vec![
            Series::new("Country Name", &["B", "A"]),
            Series::new("2020", &[2.0_f64, 1.0_f64]),
        ])
        .unwrap();

        let sorted = sort_by_column(&df, "Country Name", false).unwrap();

        let countries = sorted.column("Country Name").unwrap().utf8().unwrap();
        assert_eq!(countries.get(0), Some("A"));
        assert_eq!(countries.get(1), Some("B"));
    }

    #[test]
    fn rejects_unknown_column() {
        let df = DataFrame::new(vec![Series::new("Country Name", &["B", "A"])]).unwrap();

        let error = sort_by_column(&df, "not-a-column", false).unwrap_err();

        assert!(error.contains("not-a-column"));
        assert!(error.contains("Country Name"));
    }

    #[test]
    fn filters_numeric_column() {
        let df = DataFrame::new(vec![
            Series::new("Country Name", &["A", "B", "C"]),
            Series::new("2020", &[71.0_f64, 69.0_f64, 72.5_f64]),
        ])
        .unwrap();

        let filtered = filter_by_condition(&df, "2020", "gt", "70").unwrap();

        assert_eq!(filtered.height(), 2);
    }

    #[test]
    fn filters_string_column() {
        let df = DataFrame::new(vec![Series::new(
            "Country Code",
            &["USA", "CAN", "USA"],
        )])
        .unwrap();

        let filtered = filter_by_condition(&df, "Country Code", "eq", "USA").unwrap();

        assert_eq!(filtered.height(), 2);
    }
}
