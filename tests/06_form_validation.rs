/*
Check for the presence of absence of page elements
DISCLAIMER: This code is aimed at Rust BEGINNERS. This is not how Qxf2 engineers write rust code at clients.

AUTHOR: Ajitava Deb

SCOPE:
1) Launch Chrome driver
2) Navigate to Qxf2 Tutorial page
3) Find the Click me! button and click on it
4) Check for the validation message
5) Close the browser
*/

use thirtyfour::prelude::*;

#[tokio::test]
async fn form_validation() -> WebDriverResult<()> {
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
    
    let name_variable = "Please enter your name";

    //KEY POINT: Check if the validation mesage for name field
    let validation_name = driver
        .find(By::XPath("//label[text()='Please enter your name']"))
        .await;

    match validation_name {
        Ok(_val) => println!("{} is present", name_variable),
        Err(_) => println!("{} is not present", name_variable),
    }

    //Close the browser window
    driver.quit().await?;

    Ok(())
}
