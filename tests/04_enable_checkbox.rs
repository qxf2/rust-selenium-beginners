/*
Learn to select a checkbox using Selenium in Rust
DISCLAIMER: This code is aimed at Rust BEGINNERS. This is not how Qxf2 engineers write rust code at clients.

AUTHOR: Ajitava Deb

SCOPE:
1) Launch Chrome Driver
2) Navigate to Qxf2 Tutorial page
3) Find the Checkbox element in the Example form and enable it
4) Close the browser
*/

use thirtyfour::prelude::*;

#[tokio::test]
async fn enable_checkbox() -> WebDriverResult<()> {
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

    //KEY POINT: Locate the checkbox and click on it
    //We are purposefully not showing assert here. We will show how to assert in later tests.
    let checkbox = driver
        .find(By::XPath("//input[@type='checkbox']"))
        .await
        .expect("Unable to locate the checkbox");
    checkbox
        .click()
        .await
        .expect("Not able to click the checkbox");

    //Close the browser window
    driver.quit().await?;

    Ok(())
}
