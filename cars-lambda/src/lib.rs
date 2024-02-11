/*
 * This program reads the Iris dataset from a CSV file located at "src/data/iris.csv".
 * It assumes the CSV has headers.
 *
 * 1. Filtering:
 *    It filters the rows where the column 'sepal_length' is greater than 5.0.
 *
 * 2. Grouping:
 *    The program then groups the data by the 'species' column.
 *
 * 3. Aggregation:
 *    For each group (species in this case), it calculates the sum of the other columns.
 *
 * Output Explanation:
 *    The resulting DataFrame has a shape of (3, 5), indicating that it has 3 rows and 5 columns.
 *    The rows represent the three species of Iris flowers: 'setosa', 'versicolor', and 'virginica'.
 *    The columns represent:
 *    - 'species': The name of the Iris species
 *    - 'sepal_length': The sum of 'sepal_length' values for each species
 *    - 'sepal_width': The sum of 'sepal_width' values for each species
 *    - 'petal_length': The sum of 'petal_length' values for each species
 *    - 'petal_width': The sum of 'petal_width' values for each species
 *
 *    For example, for 'virginica', the sum of 'sepal_length' values is 324.5,
 *    the sum of 'sepal_width' values is 146.2, and so on.
 */

// Import necessary modules from the `polars` crate
use polars::prelude::*;
use std::io::Cursor;

// Car dataset in CSV format
const CAR_DATA: &str = "brand,model,year,price
Toyota,Camry,2018,25000
Honda,Accord,2019,27000
Toyota,Corolla,2017,20000
Ford,Fusion,2018,22000
Honda,Civic,2020,28000
Ford,Focus,2019,21000
Toyota,Rav4,2020,30000
Honda,CRV,2018,31000
Ford,Escape,2020,32000";

pub fn calculate(filter_value: f64) -> Result<DataFrame, PolarsError> {
    let file = Cursor::new(CAR_DATA);

    let df = CsvReader::new(file)
        .has_header(true)
        .finish()?
        .lazy()
        // add filter
        .filter(col("price").gt(lit(filter_value)))
        .group_by(vec![col("brand")])
        .agg(&[
            col("price").mean().alias("average_price"),
            col("year").mean().alias("average_year"),
        ])
        .collect()?;

    Ok(df)
}


