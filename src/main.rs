use eyre::Result;
use webdriver_install::chromedriver;
use webdriver_install::DriverFetcher;
// use webdriver_install::geckodriver;
// use webdriver_install::installer;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    match chromedriver::Version::find() {
        Ok(info) => println!("Chrome found: {:?}", info.build_version()),
        Err(e) => println!("Error: {:#}", e),
    }

    println!("{}", chromedriver::Chromedriver::new().latest_version()?);

    Ok(())
}

