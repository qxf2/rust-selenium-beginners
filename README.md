# Learn Selenium using examples for beginners
Selenium lets you control the browser through a script. Selenium lets you interact with browsers in a manner similar to real users interacting with browsers. We present small code samples that help you learn selenium from scratch using Rust with https://qxf2.com/selenium-tutorial-main. These are example code for some of the most commonly performed actions on a webpage like navigating to a URL, fill in the text, click buttons, hover over elements, choose drop downs, validate text, etc. Rust is not particularly good for UI tests or web based automation. UI automation is a small portion in the Rust ecosystem.

**Disclaimer:** This code is aimed at Selenium BEGINNERS with Rust. There are several bad practices here like hardcoding URL's, values, using sleep. Qxf2 engineers doesn't write such rust code at clients.

## Setting up Rust Development enviornment
The following needs to be installed to start programming in Rust.
- Visual Studio code editor
- Microsoft C++ build tools
- Rust Language files

## Install Visual Studio Code on your machine
- Go to the visual studio code download page (https://code.visualstudio.com/download) and download the installer based on your operating system
- Start the installer
- After the installation is done, start visual studio code

## Install Visual C++ build tools
- Rust needs build tools for Visual Studio
- Note: Before installing Rust, you need to install the build tools
- Go to the Microsoft Visual Studio download page and Select Download Build Tools
- Run the Installer after downloading
- In Installer window, under Desktop&Mobile, select the checkbox for C++ build tools
- On the right side panel, under Installation details, check the first 5 optional checkbox
- At the bottom right, click Install
- After it completes, you can do the Rust Installation

## Install Rust
- It is recommended to install rust through rustup which is the Rust tool chain installer
- Go to the website, https://rustup.rs/ and download the rustup-init.exe file and follow the onscreen instruction
- For Linux user, you can use the following command in your terminal and follow the onscreen instruction

        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

## Check the Rust installation
- Run the following command in your terminal or command prompt
        
        rustc --version
        
- The output should be something like, 

        rustc 1.62.1 (e092d0b6b 2022-07-16)
        
- Run the following: 
        
        cargo --version
        
- The output should be something like, 
        
        cargo 1.62.1 (a748cf5a3 2022-06-08)
        
- The installation is succesfull if you see this type of output

## Download the driver
- Download Geckodriver (https://github.com/mozilla/geckodriver/releases) and add it to your PATH environment variable
- Download Chromedriver (https://sites.google.com/chromium.org/driver/) and add it to your PATH environment variable

## Run the Tests

  **Note:** The folder structure contains tests and all tests are present within it because these are integration tests.
  **Note:** When you run the tests for first time, it will take sometime(~5min) to build and run it. From next time, tests would run in a jiffy.
  
- Open a terminal and run the driver
- Open another terminal and run the following to run individual test
        
        cargo test --test 01_url_navigation
        
- To run all the tests sequentially, run the following
        
        cargo test  

We have found that a lot of Rust codes written online is for folks who knows Rust well. Hence, the code provided doesn't always compile or they just provide code snippets. We thought, for someone like a Rust beginner, this would be helpful. 
