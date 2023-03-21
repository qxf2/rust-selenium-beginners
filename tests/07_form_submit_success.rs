/*
Learn to fill and submit a form with Selenium in Rust
DISCLAIMER: This code is aimed at Rust BEGINNERS. This is not how Qxf2 engineers write rust code at clients.

AUTHOR: Ajitava Deb

SCOPE:
1) Launch Chrome driver
2) Navigate to Qxf2 Tutorial page
3) Fill all the text field in Example form
4) Click on Click me! button
5) Verify user is taken to Selenium Tutorial redirect page
6) Close the browser
*/

use thirtyfour::prelude::*;

#[tokio::test]
async fn form_submit_success() -> WebDriverResult<()> {
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

    //Check if the title of the page is proper
    assert_eq!(
        driver.title().await.expect("Page title not found"),
        "Qxf2 Services: Selenium training main"
    );

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
        .find(By::Id("phone"))
        .await
        .expect("Phone number field not found");
    phone
        .send_keys("9999999999")
        .await
        .expect("Unable to enter phone number in the field");

    //KEY POINT: Locate the button and click on it
    let button = driver
        .find(By::XPath("//button[text()='Click me!']"))
        .await
        .expect("Unable to locate the button, Click Me");
    button
        .click()
        .await
        .expect("Unable to click the button Click me");

    //Verify user is taken to Qxf2 tutorial redirect url
    let url = driver
        .current_url()
        .await
        .expect("Unable to fetch the current url");

    //Close the browser window
    driver.quit().await?;

    //Assert the Qxf2 tutorial redirect url
    assert_eq!(url.as_str(), "https://qxf2.com/selenium-tutorial-redirect");

    Ok(())
}
