use std::{ thread, time::Duration };

use nanoid::nanoid;

use thirtyfour::prelude::*;

const SCROLL_BAR_X_OFFSET: i64 = 384;

pub struct YandexMapScraper<'a> {
    driver: &'a WebDriver,
}

impl<'a> YandexMapScraper<'a> {
    pub fn new(driver: &'a WebDriver) -> Self {
        YandexMapScraper {
            driver: driver,
        }
    }

    pub async fn parse_search_results(
        &self,
        url: String,
        output_dir: String,
        excluded_categories: Vec<String>
    ) -> WebDriverResult<()> {
        self.driver.goto(url).await?;

        self.scroll_to_end(By::XPath("//div[@class='add-business-view']"), 150).await?;

        let mut writer = csv::WriterBuilder::new()
            .delimiter(b';')
            .has_headers(true)
            .double_quote(false)
            .from_path(output_dir)
            .unwrap();

        writer.write_record(&["COORDINATES", "ADDRESS", "ID"]).unwrap();

        let search_list = self.driver
            .find(By::XPath("//ul[@class='search-list-view__list']"))
            .await?;

        let search_results = search_list.find_all(By::Tag("li")).await?;

        let check_exist = if excluded_categories.len() != 0 {
            true
        } else {
            false 
        };

        let mut count = 0;

        println!("search results length: {}", search_results.len());

        'main: for result in search_results {
            let coordinates = result
                .find(
                    By::XPath(
                        ".//div[contains(concat(' ', normalize-space(@class), ' '), ' search-snippet-view__body _type_business ')]"
                    )
                ).await?
                .attr("data-coordinates")
                .await?
                .unwrap();
            let splited_coordinates: Vec<&str> = coordinates.split(",").collect();
            let reversed_coordinates = format!(
                "{},{}",
                splited_coordinates[1],
                splited_coordinates[0]
            );

            let address = result
                .find(By::XPath(".//*[@class='search-business-snippet-view__address']"))
                .await?
                .text()
                .await?;

            if check_exist {
                let categories_list = result
                    .find_all(
                        By::XPath(".//a[@class='search-business-snippet-view__category']")
                    ).await?;

                let mut text_list: Vec<String> = vec![];

                for category in &categories_list {
                    match category.text().await {
                        Ok(el) => text_list.push(el.to_lowercase()),
                        Err(error) => {
                            return Err(error)
                        }
                    };
                }

                for category in &excluded_categories {
                    if text_list.contains(category) {
                        continue 'main;
                    }
                }

                count += 1;
            }

            match writer.write_record(&[reversed_coordinates, address, nanoid!()]) {
                Ok(_) => {}
                Err(error) => {
                    return Err(WebDriverError::CustomError(error.to_string()))
                }
            }

            match writer.flush() {
                Ok(_) => {}
                Err(error) => {
                    return Err(WebDriverError::CustomError(error.to_string()))
                }
            }
        }

        if check_exist {
            println!("search results with excluded categories length: {}", count);
        }

        println!("file uploaded!");

        Ok(())
    }

    async fn scroll_to_end(&self, end_element: By, scrolling_speed: i64) -> WebDriverResult<()> {
        let scroll_bar = self.driver
            .find(By::XPath("//div[@class='scroll__scrollbar-thumb']")).await?;

        let mut new_scrolling_speed = scrolling_speed;

        let mut error: Option<WebDriverError> = None;

        'main: loop {
            match
                self.driver
                    .action_chain()
                    .drag_and_drop_element_by_offset(
                        &scroll_bar,
                        SCROLL_BAR_X_OFFSET,
                        new_scrolling_speed
                    )
                    .perform().await
            {
                Ok(_) => {}
                Err(err) => {
                    if matches!(err, WebDriverError::CmdError(_)) {
                        new_scrolling_speed /= 2;

                        continue 'main;
                    } else {
                        error = Some(err);

                        break 'main;
                    }
                }
            }

            match self.driver.find(end_element.clone()).await {
                Ok(_) => {
                    thread::sleep(Duration::from_secs(1));

                    let search_list = self.driver
                        .find(By::XPath("//ul[@class='search-list-view__list']")).await?;

                    self.driver
                        .execute(
                            r#"arguments[0].scrollIntoView(false)"#,
                            vec![search_list.to_json().unwrap()]
                        ).await?;

                    println!("nice scrolling");

                    break 'main;
                }
                Err(err) => {
                    if matches!(err, WebDriverError::NoSuchElement(_)) {
                        new_scrolling_speed = scrolling_speed;
                    } else {
                        error = Some(err);

                        break 'main;
                    }
                }
            }

            thread::sleep(Duration::from_millis(200));
        }
    
        if let Some(error) = error {
            Err(error)
        } else {
            Ok(())
        }
    }
}