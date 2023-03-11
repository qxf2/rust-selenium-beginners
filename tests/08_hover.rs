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

use thirtyfour::prelude::*;

#[tokio::test]
async fn hover() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();

    //Create an instance of WebDriver
    let driver = WebDriver::new("http://localhost:9515", caps)
                            .await
                            .expect("Please create a instance of WebDriver");

    //KEY POINT: The driver.goto method will navigate to a page given by the URL
    driver.goto("http://qxf2.com/selenium-tutorial-main")
                            .await
                            .expect("Couldn't navigate to the URL");

    //Maximize the browser window
    driver.maximize_window().await?;

    //Locate the Menu icon and click on it
    let menu = driver.find(By::XPath("//img[@src='./assets/img/menu.png']"))
                            .await
                            .expect("Menu icon not found");
    menu.click()
                            .await
                            .expect("Menu is not clicked");

    //Locate the Resource element to hover over
    let resource = driver.find(By::XPath("//a[text()='Resources']"))
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
    let gui_automation = driver.find(By::XPath("//a[text()='GUI automation']"))
                            .await
                            .expect("The gui automation link was not found");
    gui_automation.click()
                            .await
                            .expect("The gui automation link was not clicked");

    //Close the browser window
    driver.quit().await?;     

    Ok(())
}