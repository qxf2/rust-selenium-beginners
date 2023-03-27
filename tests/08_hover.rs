/*
Learn to hover over elements using Selenium in Rust
DISCLAIMER: This code is aimed at Rust BEGINNERS. This is not how Qxf2 engineers write rust code at clients.

AUTHOR: Ajitava Deb

SCOPE:
1) Launch Chrome driver
2) Navigate to Qxf2 Tutorial page
3) Click on Menu icon
4) Hover over Resource and GUI automation and click on GUI automation link
5) Close the browser
*/

use thirtyfour::prelude::{By,DesiredCapabilities,WebDriver,WebDriverResult};

#[tokio::test]
async fn hover() -> WebDriverResult<()> {
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

    //Locate the Menu icon and click on it
    let menu = driver
        .find(By::XPath("//img[@src='./assets/img/menu.png']"))
        .await
        .expect("Menu icon not found");
    menu.click().await.expect("Menu is not clicked");

    //Locate the Resource element to hover over
    let resource = driver
        .find(By::XPath("//a[text()='Resources']"))
        .await
        .expect("Resource is not found");

    driver
        .action_chain()
        .move_to_element_center(&resource)
        .click_and_hold()
        .perform()
        .await
        .expect("The required resource was not found and clicked");

    //Click the GUI automation link
    let gui_automation = driver
        .find(By::XPath("//a[text()='GUI automation']"))
        .await
        .expect("The gui automation link was not found");
    gui_automation
        .click()
        .await
        .expect("The gui automation link was not clicked");

    let current_url = driver
        .current_url()
        .await
        .expect("Failed to get current URL");

    //Close the browser window
    driver.quit().await?;

    //Assert that the URL has changed to the GUI automation page
    assert_eq!(current_url.as_str(), "https://qxf2.com/gui-automation-diy");

    Ok(())
}
