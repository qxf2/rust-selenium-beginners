/*
Learn to set dropdowns with Selenium in Rust
DISCLAIMER: This code is aimed at Rust BEGINNERS. This is not how Qxf2 engineers write rust code at clients.

AUTHOR: Ajitava Deb

SCOPE:
1) Launch Chrome Driver
2) Navigate to Qxf2 Tutorial page
3) Set Gender to Male in the Example Form
4) Close the browser
*/

use thirtyfour::prelude::*;

#[tokio::test]
async fn set_dropdown() -> WebDriverResult<()> {
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

    //KEY POINT: Identify the dropdown and click on it
    //We are purposefully not showing assert here. We will show how to assert in later tests.
    let dropdown_element = driver
        .find(By::Css("button[type='button']"))
        .await
        .expect("Drop down not found");
    dropdown_element
        .click()
        .await
        .expect("Unable to click the dropdown");

    //KEY POINT: Locate a particular option and click on it
    let option = driver
        .find(By::XPath("//a[text()='Male']"))
        .await
        .expect("Not able to locate a option");
    option.click().await.expect("Unable to click the option");

    //Close the browser window
    driver.quit().await?;

    Ok(())
}
