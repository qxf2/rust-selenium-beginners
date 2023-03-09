/*

Check for the presence of absence of page elements
DISCLAIMER: This code is aimed at Rust BEGINNERS

AUTHOR: Ajitava Deb

 */

use thirtyfour::prelude::*;

#[tokio::test]
async fn form_validation() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();

    //Create an instance of WebDriver
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    //KEY POINT: The driver.goto method will navigate to a page given by the URL
    driver.goto("http://qxf2.com/selenium-tutorial-main").await?;

    //Maximize the browser window
    driver.maximize_window().await?;

    //KEY POINT: Locate the button and click on it
    let button = driver.find(By::XPath("//button[text()='Click me!']")).await?;
    button.click().await?;

    //KEY POINT: Check if the validation mesage for name field
    let validation_name = driver.find(By::XPath("//label[text()='Please enter your name']")).await;
    
    match validation_name {
        Ok(_val) => println!("Please enter your name is present"),
        Err(_) => println!("Please enter your name is not present")
    }

    //Close the browser window
    driver.quit().await?;   

    Ok(())

    }