/*
Selenium script that performs several common actions like:
click button, select dropdown, enable checkbox, set text, get text from table

DISCLAIMER: This code is aimed at Rust BEGINNERS

AUTHOR: Ajitava Deb

SCOPE:
1) Launch Chrome driver
2) Navigate to Qxf2 Tutorial page
3) Print the contents of the table
4) Fill all the text fields
5) Select Dropdown option
6) Enable the checkbox
7) Take a screenshot
8) Click on Submit button
9) Close the browser
*/

use thirtyfour::prelude::*;

#[tokio::test]
async fn table_text() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();

    //Create an instance of WebDriver
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    //KEY POINT: The driver.goto method will navigate to a page given by the URL
    driver.goto("http://qxf2.com/selenium-tutorial-main").await?;

    //Maximize the browser window
    driver.maximize_window().await?;

    //# KEY POINT: Logic to get the text in each cell of the table
    //# Find the Example table element in the page
    let table = driver.find(By::XPath("//table[@name='Example Table']")).await?;

    //# Use find_elements_by_xpath method to get the rows in the table
    let rows = table.find_all(By::XPath("//tbody/descendant::tr")).await?;
    let rows_len = rows.len();

    let mut my_table: Vec<Vec<String>> = Vec::new();

    for i in 0..rows_len{
        //# Find no of columns by getting the td elements in each row
        let cols = rows[i].find_all(By::Tag("td")).await?; 
        let cols_len = cols.len();
        
        let mut column_data: Vec<String> = Vec::new();

        for j in 0..cols_len {
            //# Get the text of each field
            let value = cols[j].text().await?;
            column_data.push(value);
        }
        my_table.push(column_data);
    }
    //# Print the result list
    println!("{:?}", my_table);

    //Find the name field using id
    let name = driver.find(By::Id("name")).await?;
    name.send_keys("Ajitava").await?;

    //Find the email field using name
    let email = driver.find(By::Name("email")).await?;
    email.send_keys("ajitava@qxf2.com").await?;

    //Find the phone no field using id
    let phone = driver.find(By::Id("phone")).await?;
    phone.send_keys("9999999999").await?;   

    //KEY POINT: Identify the dropdown and click on it
    let dropdown_element = driver.find(By::Css("button[type='button']")).await?;
    dropdown_element.click().await?;

    //KEY POINT: Locate a particular option and click on it
    let option = driver.find(By::XPath("//a[text()='Male']")).await?;
    option.click().await?;

    //KEY POINT: Locate the checkbox and click on it
    let checkbox = driver.find(By::XPath("//input[@type='checkbox']")).await?;
    checkbox.click().await?;

    //KEY POINT: Locate the button and click on it
    let button = driver.find(By::XPath("//button[text()='Click me!']")).await?;
    button.click().await?;

    //Verify user is taken to Qxf2 tutorial redirect url
    let url = driver.current_url().await?;
    assert_eq!(url.as_str(), "https://qxf2.com/selenium-tutorial-redirect");

    //Quit the browser window 
    driver.quit().await?;
    
    Ok(())
}                    



