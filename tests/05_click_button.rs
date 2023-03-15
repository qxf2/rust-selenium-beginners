/*
Learn to click a button with Selenium in Rust
DISCLAIMER: This code is aimed at Rust BEGINNERS. This is not how Qxf2 engineers write rust code at clients.

AUTHOR: Ajitava Deb

SCOPE:
1) Launch Chrome driver
2) Navigate to Qxf2 Tutorial page
3) Find the Click me! button and click on it
4) Close the driver
*/

use thirtyfour::prelude::*;

#[tokio::test]
async fn click_button() -> WebDriverResult<()> {
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

    //KEY POINT: Locate the button and click on it
    let button = driver
        .find(By::XPath("//button[text()='Click me!']"))
        .await
        .expect("Unable to locate the button, Click Me");
    button
        .click()
        .await
        .expect("Unable to click the button Click me");

    //Fetching the button title
    let button_title = button.inner_html().await?;

    //Close the browser window
    driver.quit().await?;

    //Shows how to assert on button text.
    assert_eq!(
        button_title,
        "Click me!"
    );    

    Ok(())
}
