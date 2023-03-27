/*
Learn to count the rows in a table using Selenium in Rust
DISCLAIMER: This code is aimed at Rust BEGINNERS. This is not how Qxf2 engineers write rust code at clients.

AUTHOR: Ajitava Deb

SCOPE:
1) Launch Chrome driver
2) Navigate to Qxf2 Tutorial page
3) Find the no of rows in the Example tabel
4) Close the browser
*/

use thirtyfour::prelude::{By,DesiredCapabilities,WebDriver,WebDriverResult};

#[tokio::test]
async fn count_rows() -> WebDriverResult<()> {
    let capabilities = DesiredCapabilities::chrome();

    //Create an instance of WebDriver
    let driver = WebDriver::new("http://localhost:9515", capabilities)
                            .await
                            .expect("Failed to connect to localhost:9515. Have you started the WebDriver process in another terminal?");

    //KEY POINT: The driver.goto method will navigate to a page given by the URL
    driver
        .goto("http://qxf2.com/selenium-tutorial-main")
        .await
        .expect("Couldn't navigate to the URL");

    //Maximize the browser window
    driver.maximize_window().await?;

    //Find the table element in the page
    let table = driver
        .find(By::XPath("//table[@name='Example Table']"))
        .await
        .expect("Couldn't find the table element");

    //KEY POINT: Find the tr elements in the table
    let rows = table
        .find(By::XPath("//tbody/descendant::tr"))
        .await
        .expect("Couldn't find the row elements in the table");

    //Close the browser window
    driver.quit().await?;

    //Assert the row length
    assert_eq!(rows.to_string().len(), 296);

    Ok(())
}
