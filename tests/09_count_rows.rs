/*

Learn to count the rows in a table using Selenium in Rust
DISCLAIMER: This code is aimed at Rust BEGINNERS

AUTHOR: Ajitava Deb

 */

use thirtyfour::prelude::*;

#[tokio::test]
async fn count_rows() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();

    //Create an instance of WebDriver
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    //KEY POINT: The driver.goto method will navigate to a page given by the URL
    driver.goto("http://qxf2.com/selenium-tutorial-main").await?;

    //Maximize the browser window
    driver.maximize_window().await?;

    //Find the table element in the page
    let table = driver.find(By::XPath("//table[@name='Example Table']")).await?;

    //KEY POINT: Find the tr elements in the table
    let rows = table.find(By::XPath("//tbody/descendant::tr")).await?;
    println!("Total No of Rows: {:?}", rows.to_string().len());

    //Close the browser window
    driver.quit().await?;    

    Ok(())

}
    
