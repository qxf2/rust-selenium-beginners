/*

Learn to hover over elements using Selenium in Rust
DISCLAIMER: This code is aimed at Rust BEGINNERS

AUTHOR: Ajitava Deb

 */

use thirtyfour::prelude::*;

#[tokio::test]
async fn hover() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();

    //Create an instance of WebDriver
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    //KEY POINT: The driver.goto method will navigate to a page given by the URL
    driver.goto("http://qxf2.com/selenium-tutorial-main").await?;

    //Maximize the browser window
    driver.maximize_window().await?;

    //Locate the Menu icon and click on it
    let menu = driver.find(By::XPath("//img[@src='./assets/img/menu.png']")).await?;
    menu.click().await?;

    //Locate the Resource element to hover over
    let resource = driver.find(By::XPath("//a[text()='Resources']")).await?;

    driver
        .action_chain()
        .move_to_element_center(&resource)
        .click_and_hold()
        .perform()
        .await?;

    //Click the GUI automation link
    let gui_automation = driver.find(By::XPath("//a[text()='GUI automation']")).await?;
    gui_automation.click().await?;

    //Close the browser window
    driver.quit().await?;     

    Ok(())
}