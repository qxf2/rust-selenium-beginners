/*

Learn to parse the text within each cell of a table in Rust
DISCLAIMER: This code is aimed at Rust BEGINNERS

AUTHOR: Ajitava Deb

 */

use thirtyfour::prelude::*;

#[tokio::test]
async fn table_text() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();

    //Create an instance of WebDriver
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    //KEY POINT: The driver.goto method will navigate to a page given by the URL
    driver.goto("http://qxf2.com/selenium-tutorial-main").await?;

    //Maximize the browser window
    driver.maximize_window().await?;

    //# KEY POINT: Logic to get the text in each cell of the table
    //# Find the Example table element in the page
    let table = driver.find(By::XPath("//table[@name='Example Table']")).await?;

    //# Use find_elements_by_xpath method to get the rows in the table
    let rows = table.find_all(By::XPath("//tbody/descendant::tr")).await?;
    let rows_len = rows.len();

    let mut my_table: Vec<Vec<String>> = Vec::new();

    for i in 0..rows_len{
        //# Find no of columns by getting the td elements in each row
        let cols = rows[i].find_all(By::Tag("td")).await?; 
        let cols_len = cols.len();
        
        let mut column_data: Vec<String> = Vec::new();

        for j in 0..cols_len {
            //# Get the text of each field
            let value = cols[j].text().await?;
            column_data.push(value);
        }
        my_table.push(column_data);
    }
    //# Print the result list
    println!("{:?}", my_table);

    Ok(())
}