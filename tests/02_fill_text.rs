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

use std::time::Duration;
use thirtyfour::prelude::*;
use tokio::time::sleep;

#[tokio::test]
async fn fill_text() -> WebDriverResult<()> {
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

    //Strong the Page Title
    let page_title = driver.title()
        .await
        .expect("Page title not found");    

    //Find the name field using id
    let name = driver
        .find(By::Id("name"))
        .await
        .expect("Name field not found");
    name.send_keys("Ajitava")
        .await
        .expect("Unable to send the name in text field");

    //Find the email field using name
    let email = driver
        .find(By::Name("email"))
        .await
        .expect("Email field not found");
    email
        .send_keys("ajitava@qxf2.com")
        .await
        .expect("Unable to send email in text field");

    //Find the phone no field using id
    let phone = driver
        .find(By::XPath("//input[@id='phone']"))
        .await
        .expect("Phone number field not found");
    phone
        .send_keys("9999999999")
        .await
        .expect("Unable to enter phone number in the field");

    //Sleep is one way to wait for things to load and it's an anti-pattern
    //Future tutorials cover other type of waits like implicit etc.
    sleep(Duration::from_secs(5)).await;

    //Close the browser window
    driver.quit().await?;

    //Check if the title of the page is proper
    assert_eq!(
        page_title,
        "Qxf2 Services: Selenium training main"
    );    

    Ok(())
}
