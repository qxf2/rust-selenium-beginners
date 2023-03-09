/*
Learn to set dropdowns with Selenium in Rust
DISCLAIMER: This code is aimed at Rust BEGINNERS

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
    let caps = DesiredCapabilities::chrome();

    //Create an instance of WebDriver
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    //KEY POINT: The driver.goto method will navigate to a page given by the URL
    driver.goto("http://qxf2.com/selenium-tutorial-main").await?;

    //Maximize the browser window
    driver.maximize_window().await?;

    //KEY POINT: Identify the dropdown and click on it
    let dropdown_element = driver.find(By::Css("button[type='button']")).await?;
    dropdown_element.click().await?;

    //KEY POINT: Locate a particular option and click on it
    let option = driver.find(By::XPath("//a[text()='Male']")).await?;
    option.click().await?;

    //Close the browser window
    driver.quit().await?;
    
    Ok(())

}