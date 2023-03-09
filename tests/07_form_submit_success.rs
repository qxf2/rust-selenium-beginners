/*

Learn to fill and submit a form with Selenium in Rust
DISCLAIMER: This code is aimed at Rust BEGINNERS

AUTHOR: Ajitava Deb

 */

use thirtyfour::prelude::*;

#[tokio::test]
async fn form_submit_success() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();

    //Create an instance of WebDriver
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    //KEY POINT: The driver.goto method will navigate to a page given by the URL
    driver.goto("http://qxf2.com/selenium-tutorial-main").await?;

    //Check if the title of the page is proper
    assert_eq!(driver.title().await?, "Qxf2 Services: Selenium training main");  

    //Find the name field using id
    let name = driver.find(By::Id("name")).await?;
    name.send_keys("Ajitava").await?;

    //Find the email field using name
    let email = driver.find(By::Name("email")).await?;
    email.send_keys("ajitava@qxf2.com").await?;

    //Find the phone no field using id
    let phone = driver.find(By::Id("phone")).await?;
    phone.send_keys("9999999999").await?;

    //KEY POINT: Locate the button and click on it
    let button = driver.find(By::XPath("//button[text()='Click me!']")).await?;
    button.click().await?;

    //Verify user is taken to Qxf2 tutorial redirect url
    let url = driver.current_url().await?;
    assert_eq!(url.as_str(), "https://qxf2.com/selenium-tutorial-redirect");

    //Close the browser window
    driver.quit().await?; 
        
    Ok(())

}    