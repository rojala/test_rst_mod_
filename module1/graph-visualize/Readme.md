# External GitHub Lab: Graph Visualization in Rust

Objective: In this lab, you will get hands-on experience with graph visualization in Rust. The provided code uses the rasciigraph crate to generate a simple ASCII graph that shows distances travelled between different cities.

## Instructions:

1. Step 1: Go to the following GitHub repository: 
https://github.com/nogibjj/rust-data-engineering

2. Step 2: Click the green "Code" button and then select "Open with GitHub Codespaces". Follow the instructions to create a new Codespace.

3. Step 3: Once your Codespace is ready, navigate to the graph-visualize directory.

4. Step 4: Open the src/main.rs file and review the provided code. This script plots the distances travelled between different cities on an ASCII graph.

4. Step 5: In the Codespaces terminal, compile the program by running cargo build.

5. Step 6: Run the program by using cargo run in the terminal. The program will generate and print an ASCII graph representing the distances travelled.

```bash
Lisbon > Madrid > Paris > Berlin > Copenhagen > Stockholm > Moscow
      4606   ┤     ╭ 
      4146   ┤     │ 
      3685   ┤     │ 
      3224   ┤    ╭╯ 
      2764   ┤   ╭╯  
      2303   ┤  ╭╯   
      1843   ┤  │    
      1382   ┤  │    
       921   ┤ ╭╯    
       461   ┤╭╯     
         0   ┼╯     
                Travelled distances (km)
```

## Reflection Questions:

1. How does the rasciigraph crate generate an ASCII graph?

    The `rasciigraph` crate follows a systematic approach to convert numerical data into visual ASCII line graphs:

    1. **Data Preprocessing & Scaling**
    - **Min/Max Detection**: Finds the global minimum and maximum values across all data series
    - **Height Calculation**: Automatically determines the graph height based on data range or uses user-specified height
    - **Ratio Calculation**: Creates a scaling ratio to map data values to graph coordinates
    - **Data Interpolation**: If a width is specified, uses linear interpolation to fit data points within the desired width

    2. **Grid Creation**
    The algorithm creates a 2D grid (ASCII canvas) where:
    - Rows represent Y-axis values (data magnitude)
    - Columns represent X-axis values (data points)
    - Each cell starts as a space character

    3. **Y-Axis Labeling**
    - Calculates appropriate decimal precision for Y-axis labels based on data magnitude
    - Places numerical labels on the left side of the graph
    - Uses a consistent width alignment for all labels

    4. **Line Drawing Algorithm**
    For each data series, it draws lines between consecutive points using these ASCII characters:

      ```rust
        // Different directional connections
        if y0 == y1 {
            // Horizontal line: ─
            plot[(rows - y0) as usize][(x as u32 + config.offset) as usize] = "─".to_string();
        } else {
            if y0 > y1 {
                // Rising line: ╰ at bottom, ╮ at top
                plot[(rows - y1) as usize][(x as u32 + config.offset) as usize] = "╰".to_string();
                plot[(rows - y0) as usize][(x as u32 + config.offset) as usize] = "╮".to_string();
            } else {
                // Falling line: ╭ at top, ╯ at bottom  
                plot[(rows - y1) as usize][(x as u32 + config.offset) as usize] = "╭".to_string();
                plot[(rows - y0) as usize][(x as u32 + config.offset) as usize] = "╯".to_string();
            }
            
            // Vertical connectors: │
            let start = f64::min(f64::from(y0), f64::from(y1)) as i32 + 1;
            let end = f64::max(f64::from(y0), f64::from(y1)) as i32;
            for y in start..end {
                plot[(rows - y) as usize][(x as u32 + config.offset) as usize] = "│".to_string();
            }
        }
      ```

    5. **Character Mapping**
    - **`┼`**: Used for Y-axis intersection points
    - **`─`**: Horizontal line segments
    - **`│`**: Vertical line segments  
    - **`╭╮╰╯`**: Corner characters for directional changes
    - **`─`**: End points when one value is NaN (missing data)

    6. **Configuration Options**
    The `Config` struct allows customization:
    - **Offset**: Horizontal spacing for Y-axis labels
    - **Height**: Graph height in characters
    - **Width**: Total graph width (with interpolation)
    - **Caption**: Text displayed below the graph

    **Code Breakdown**

    In code:
    ```rust
      distances_travelled = vec![0.0, 502.56, 1053.36, 2187.27, 2636.42, 3117.23, 4606.35]
    ```

    The algorithm:
    1. Finds min=0.0, max=4606.35
    2. Calculates scaling ratio based on configured height (10)
    3. Maps each distance to Y-coordinate positions
    4. Draws connecting lines between consecutive distance points
    5. Adds Y-axis labels and the caption "Travelled distances (km)"

    **Key Design Features**

    - **Zero Dependencies**: No external crates required for core functionality
    - **Multiple Series Support**: Can plot multiple data series on the same graph
    - **NaN Handling**: Gracefully handles missing data points
    - **Automatic Scaling**: Adjusts precision and sizing based on data characteristics
    - **Unicode Support**: Uses proper box-drawing characters for clean visualization

    This approach creates a lightweight, dependency-free solution for generating line graphs in terminal environments using only ASCII/Unicode characters.

2. What are the advantages and disadvantages of visualizing data in this way?
  Based on the ASCII graph generation approach used by `rasciigraph`, here are the key advantages and disadvantages:

  **Advantages**

  **1. Environment Compatibility**
  - **Works anywhere**: Compatible with terminals, SSH sessions, text-only environments, and headless servers
  - **No GUI dependencies**: Doesn't require graphical libraries, X11, or display systems
  - **Cross-platform consistency**: Renders identically across different operating systems and terminal emulators

  **2. Lightweight and Portable**
  - **Minimal resource usage**: Low CPU and memory footprint compared to graphical libraries
  - **Zero external dependencies** (for core functionality): No need for complex library installations
  - **Small file sizes**: Easy to embed in documentation, logs, or code repositories

  **3. Integration and Automation**
  - **Version control friendly**: Text-based output is easily tracked in Git
  - **Scriptable and reproducible**: Perfect for automated reports and CI/CD pipelines
  - **Easy embedding**: Can be included in markdown, README files, or console output
  - **Log-friendly**: Works well with logging systems and monitoring tools

  **4. Quick Data Exploration**
  - **Rapid visualization**: Fast way to get a visual sense of data trends
  - **Simple debugging**: Useful for examining data flow in algorithms
  - **Interactive development**: Great for REPL environments and exploratory programming

  **Disadvantages**

  **1. Limited Visual Precision**
  - **Coarse resolution**: Character grid provides limited spatial precision
  - **Quantization effects**: Data points are mapped to discrete character positions
  - **Loss of detail**: Fine-grained variations in data may be lost

  **2. Restricted Visual Complexity**
  - **Basic chart types**: Limited to line graphs and basic multi-series plots
  - **No advanced visualizations**: Cannot handle heatmaps, scatter plots, or 3D visualizations
  - **Simple legends**: Basic color coding without rich legends or annotations

  **3. Limited Interactivity**
  - **Static output**: No zoom, pan, or drill-down capabilities
  - **No user interaction**: Cannot select data points or modify views
  - **Fixed formatting**: Cannot dynamically adjust colors or styling

  **4. Accessibility and Readability**
  - **Text-based barriers**: May be difficult for users with visual impairments
  - **Terminal-dependent rendering**: Quality varies between different terminal emulators
  - **Character encoding issues**: May not display properly on all systems

  **5. Scale Limitations**
  - **Small to medium datasets**: Performance degrades with very large datasets
  - **Limited dynamic range**: May struggle with data that has very large or very small values
  - **Single-page constraint**: Not suitable for multi-page or interactive dashboards

  **When to Choose Alternatives:**
  - **Detailed data analysis**: Use matplotlib, ggplot, or D3.js
  - **Interactive dashboards**: Use Plotly, Bokeh, or web-based tools
  - **Professional reports**: Use dedicated reporting tools
  - **Large datasets**: Use specialized big data visualization tools
  - **Complex relationships**: Use network graphs or multi-dimensional plotting

  **Best Practices for ASCII Visualization**
  1. **Keep it simple**: Use for quick trend identification, not detailed analysis
  2. **Appropriate sizing**: Match graph dimensions to your terminal window
  3. **Clear labeling**: Include units, legends, and context
  4. **Complement with numbers**: Don't rely solely on visual interpretation
  5. **Use for the right context**: Logs, monitoring, development tools, documentation

  The ASCII approach represents a sweet spot for certain use cases where simplicity, portability, and automation are more important than visual sophistication.

3. How might you modify this code to visualize a different dataset?

  **Time Series Data (Stock Prices, Temperature, etc.)**

  ```rust
  extern crate rasciigraph;
  use rasciigraph::{plot, Config};

  fn main() {
      // Stock prices over time (example: AAPL)
      let dates = vec!["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug"];
      let stock_prices = vec![150.25, 148.75, 155.80, 162.45, 158.90, 165.20, 170.35, 168.75];
      
      println!("Apple Stock Price Trend (AAPL)");
      println!("{}", dates.join(" → "));
      
      println!(
          "{}",
          plot(
              stock_prices,
              Config::default()
                  .with_offset(8)
                  .with_height(12)
                  .with_caption("Stock Price ($)".to_string())
          )
      );
  }
  ```

  **Performance Metrics (Response Times, CPU Usage)**
  ```rust
  extern crate rasciigraph;
  use rasciigraph::{plot, Config};

  fn main() {
      // Server response times in milliseconds
      let time_points = vec!["00:00", "04:00", "08:00", "12:00", "16:00", "20:00", "23:59"];
      let response_times = vec![45.2, 38.7, 125.3, 89.1, 156.8, 92.4, 67.9];
      
      println!("Daily API Response Times");
      println!("{}", time_points.join(" | "));
      
      println!(
          "{}",
          plot(
              response_times,
              Config::default()
                  .with_offset(10)
                  .with_height(15)
                  .with_caption("Response Time (ms)".to_string())
          )
      );
  }
  ```

  **Sales Data by Quarter**

  ```rust
  extern crate rasciigraph;
  use rasciigraph::{plot, Config};

  fn main() {
      // Quarterly sales data (in thousands)
      let quarters = vec!["Q1 2024", "Q2 2024", "Q3 2024", "Q4 2024", "Q1 2025"];
      let sales_data = vec![125.5, 189.3, 156.8, 234.7, 198.2];
      
      println!("Quarterly Sales Performance");
      println!("{}", quarters.join(" → "));
      
      println!(
          "{}",
          plot(
              sales_data,
              Config::default()
                  .with_offset(8)
                  .with_height(10)
                  .with_caption("Revenue ($K)".to_string())
          )
      );
  }
  ```

  **Multi-Series Data (Comparing Multiple Metrics)**

  ```rust
  extern crate rasciigraph;
  use rasciigraph::{plot_many, Config};

  fn main() {
      // Weekly metrics for two products
      let weeks = vec!["Week 1", "Week 2", "Week 3", "Week 4", "Week 5"];
      
      let product_a = vec![45.2, 52.1, 48.7, 61.3, 55.8];
      let product_b = vec![38.9, 41.2, 47.8, 43.6, 49.1];
      let product_c = vec![52.1, 48.3, 55.7, 58.2, 62.4];
      
      println!("Product Performance Comparison");
      println!("{}", weeks.join(" | "));
      
      println!(
          "{}",
          plot_many(
              vec![product_a, product_b, product_c],
              Config::default()
                  .with_offset(12)
                  .with_height(8)
                  .with_width(30)
                  .with_caption("Sales Units per Week".to_string())
          )
      );
  }
  ```

  **Scientific Data (Sensor Readings, Experimental Results)**

  ```rust
  extern crate rasciigraph;
  use rasciigraph::{plot, Config};

  fn main() {
      // Temperature readings from an experiment
      let measurements = vec![
          23.1, 23.4, 23.2, 24.1, 25.3, 26.7, 27.2, 26.8, 25.9, 24.7,
          23.8, 23.5, 23.9, 24.2, 25.1, 26.3, 27.1, 26.9, 25.7, 24.8
      ];
      
      println!("Temperature Sensor Data");
      println!("Measurement points: 1 → 20");
      
      println!(
          "{}",
          plot(
              measurements,
              Config::default()
                  .with_offset(10)
                  .with_height(12)
                  .with_width(40)
                  .with_caption("Temperature (°C)".to_string())
          )
      );
  }
  ```

  **Key Modifications Explained:**

  1. **Data Structure Changes:**
  - Replace `distances_travelled` with your new data vector
  - Ensure data is `Vec<f64>` for the `plot()` function
  - For multi-series, use `Vec<Vec<f64>>` for `plot_many()`

  2. **Configuration Adjustments:**
    **Offset (`with_offset(n)`):**
    - Increase if you have larger numerical values requiring more Y-axis label space
    - Example: `with_offset(12)` for values in thousands
    **Height (`with_height(n)`):**
    - Larger values provide more vertical detail
    - Too tall can be hard to read in terminal
    - Example: `with_height(15)` for high-precision data
    **Width (`with_width(n)`):**
    - Only for multi-series plots
    - Controls horizontal resolution through interpolation
    - Example: `with_width(50)` for detailed multi-series visualization
    **Caption:**
    - Change units and context to match your data
    - Example: `"CPU Usage (%)"` or `"Memory (GB)"`

## Challenge Questions:

1. Modify the code to visualize data from a file.

  **Original Hardcoded Version**
  ```rust
  let cities = vec!["Lisbon", "Madrid", "Paris", "Berlin", "Copenhagen", "Stockholm", "Moscow"];
  let distances_travelled = vec![0.0, 502.56, 1053.36, 2187.27, 2636.42, 3117.23, 4606.35];

  plot(distances_travelled.into_iter().map(|d| d as f64).collect(), Config::default()...)
  ```

  **Read form JSON File**

  1. **Data File Changes:**
    ```json
    [
        {"city": "Lisbon", "distance": 0.0},
        {"city": "Madrid", "distance": 502.56},
        ...
    ]
    ```

  2. **Code Changes:**
  - **Added dependencies**:
    ```toml
    serde = { version = "1.0", features = ["derive"] }
    serde_json = "1.0"
    ```
  - **Added struct definition**:
    ```rust
    #[derive(Debug, Serialize, Deserialize)]
    struct CityData {
        city: String,
        distance: f64,
    }
    ```
  - **Added JSON parsing**:
    ```rust
    let city_data: Vec<CityData> = serde_json::from_str(&file_content)
        .expect("Failed to parse JSON data");
    ```
  - **Changed data extraction**:
    ```rust
    let cities: Vec<String> = city_data.iter().map(|data| data.city.clone()).collect();
    let distances_travelled: Vec<f64> = city_data.iter().map(|data| data.distance).collect();
    ```
  - **Additional Imports:**
  ```rust
  use serde::{Deserialize, Serialize};
  ```

  **JSON Version Advantages:**
  - Type safety with serde
  - Self-documenting structure
  - Easy to extend with new fields
  - Industry standard format
  - Better for complex data relationships

2. How might you use a graph like this to detect patterns in the data?

    Detecting Patterns with the Graph
    **Trend Analysis**  
    - Look for **increasing or decreasing sequences** in the distances.  
    - Example: If the graph steadily rises, it indicates longer trips over time; if it falls, trips are getting shorter.

    **Outliers / Anomalies**  
    - Sharp spikes or drops in the graph highlight unusual trips.  
    - Example: A single very long distance compared to others might suggest a special journey or data error.

    **Consistency / Regularity**  
    - Flat or repeating patterns show **consistent travel distances**.  
    - Example: If the graph oscillates between two levels, it might mean alternating short and long trips.

    **Comparisons Between Cities**  
    - Since you print the city sequence (`cities.join(" > ")`), you can visually align the graph with the city order.  
    - This helps you see which cities correspond to longer or shorter distances.

    **Clustering**  
    - Groups of similar values will appear as plateaus or clusters in the graph.  
    - Example: Several consecutive trips of ~50 km will form a flat region.


3. Extend the functionality to create more complex visualizations, such as multiple line graphs or bar charts.

    ```ascii
    Lisbon > Madrid > Paris > Berlin > Copenhagen > Stockholm > Moscow
      4606   ┤     ╭ 
      4146   ┤     │ 
      3685   ┤     │ 
      3224   ┤    ╭╯ 
      2764   ┤   ╭╯  
      2303   ┤  ╭╯   
      1843   ┤  │    
      1382   ┤  │    
       921   ┤ ╭╯    
       461   ┤╭╯     
         0   ┼╯     
                Travelled distances (km)
      3685   ┤     ╭ 
      3317   ┤     │ 
      2948   ┤     │ 
      2580   ┤    ╭╯ 
      2211   ┤   ╭╯  
      1843   ┤  ╭╯   
      1474   ┤  │    
      1106   ┤  │    
       737   ┤ ╭╯    
       369   ┤╭╯     
         0   ┼╯     
                Alternative route distances (km)
      1489   ┤    ╭ 
      1385   ┤    │ 
      1281   ┤    │ 
      1177   ┤ ╭╮ │ 
      1073   ┤ ││ │ 
       969   ┤ ││ │ 
       865   ┤ ││ │ 
       761   ┤ ││ │ 
       657   ┤ ││ │ 
       553   ┼─╯│╭╯ 
       449   ┤  ╰╯ 
                Differences between consecutive trips (km)
    ```