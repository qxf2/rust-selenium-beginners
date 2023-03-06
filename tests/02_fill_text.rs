use thirtyfour::prelude::*;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn fill_text() -> WebDriverResult<()> {
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

    //Sleep is one way to wait for things to load and it's 
    //Future tutorials cover other type of waits like implicit etc.
    sleep(Duration::from_secs(5)).await;

    //Close the browser window
    driver.quit().await?;

    
    Ok(())
}