/*
Learn to fill text fields with Selenium in Rust
DISCLAIMER: This code is aimed at Rust BEGINNERS. This is not how Qxf2 engineers write rust code at clients.

AUTHOR: Ajitava Deb

SCOPE:
1) Launch Chrome Driver
2) Navigate to Qxf2 Tutorial page
3) Find elements using id, xpath, xpath without id
4) Fill name, email and phone no in the respective fields
5) Close the browser
*/

use thirtyfour::prelude::*;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn fill_text() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();

    //Create an instance of WebDriver
    let driver = WebDriver::new("http://localhost:9515", caps)
                            .await
                            .expect("Please create a instance of WebDriver");

    //KEY POINT: The driver.goto method will navigate to a page given by the URL
    driver.goto("http://qxf2.com/selenium-tutorial-main")
                            .await
                            .expect("Couldn't navigate to the URL");

    //Check if the title of the page is proper
    assert_eq!(driver.title().await?, "Qxf2 Services: Selenium training main");  

    //Find the name field using id
    let name = driver.find(By::Id("name"))
                            .await
                            .expect("Name field not found");
    name.send_keys("Ajitava")
                            .await
                            .expect("Unable to send the name in text field");

    //Find the email field using name
    let email = driver.find(By::Name("email"))
                            .await
                            .expect("Email field not found");
    email.send_keys("ajitava@qxf2.com")
                            .await
                            .expect("Unable to send email in text field");

    //Find the phone no field using id
    let phone = driver.find(By::Id("phone"))
                            .await
                            .expect("Phone number field not found");
    phone.send_keys("9999999999")
                            .await
                            .expect("Unable to enter phone number in the field"); 

    //Sleep is one way to wait for things to load and it's 
    //Future tutorials cover other type of waits like implicit etc.
    sleep(Duration::from_secs(5)).await;

    //Close the browser window
    driver.quit().await?;

    
    Ok(())
}