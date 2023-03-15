/*
Learn to navigate to a URL using Selenium in Rust
DISCLAIMER: This code is aimed at Rust BEGINNERS. This is not how Qxf2 engineers write rust code at clients.

AUTHOR: Ajitava Deb

SCOPE:
1) Launch Chrome Driver
2) Navigate to Qxf2 Tutorial page
3) Check the page title
4) Close the browser
*/

use thirtyfour::prelude::*;

#[tokio::test]
async fn navigate_to_url() -> WebDriverResult<()> {
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

    //Storing the Page title
    let page_title = driver.title()
        .await
        .expect("Page title not found");

    //Quit the browser window
    driver.quit().await?;

    //Check if the title of the page is proper
    assert_eq!(
        page_title,
        "Qxf2 Services: Selenium training main"
    );

    Ok(())
}
