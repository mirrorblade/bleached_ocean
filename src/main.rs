mod scraper;

use thirtyfour::prelude::*;

use clap::Parser;

#[derive(Parser, Default, Debug)]
struct Arguments {
    ///List with excluded categories (example: -e="игровая комната; игровая площадка; деревья" )
    #[clap(short, long, num_args = 1.., value_delimiter = ';')]
    excluded_categories: Option<Vec<String>>,

    ///Path to output csv file (example: "./output.csv")
    output_dir: String,

    ///Url to yandex map with search results
    url: String,
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let args = Arguments::parse();

    let mut excluded_categories: Vec<String> = vec![];

    match args.excluded_categories {
        Some(categories) => {
            excluded_categories = categories
                .into_iter()
                .map(|category| {
                    return category.trim().to_lowercase().to_string();
                })
                .collect();
        }
        None => {}
    }

    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    let scraper = scraper::YandexMapScraper::new(&driver);
    match scraper.parse_search_results(args.url, args.output_dir, excluded_categories).await {
        Ok(_) => {}
        Err(error) => {
            return Err(error)
        }
    }

    driver.quit().await?;

    Ok(())
}
