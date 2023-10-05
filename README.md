# Bleached Ocean
<font style='color: rgb(20, 118, 247);'>Bleached Ocean</font> is an yandex map scraper. This scraper is built on <font style='color: rgb(217, 88, 52);'>Rust</font> with <font style='color: rgb(217, 55, 52);'>thirtyfour</font> (Selenium implementation in Rust)

## Installing
If you want to install that scraper you must have already installed **cargo**, **chromedriver**

There are two ways to build scraper:
1. With debuginfo + unoptimized
```shell
    make build
```

2. Optimized
```shell
    make build-release
```

## Launching
Steps to launch scraper:
1. Run chromedriver on port 9515
2. Launch scraper with required flags (path: ./target/release/map_scraper)