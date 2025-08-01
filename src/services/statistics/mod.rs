use crate::{models, services};
use std::collections::BTreeMap;

pub fn run(file_path: &str) {
    println!("統計情報を表示します");
    let data = services::io::read_data_or_panic(&file_path);
    show_yearly_statistics(&data);
    show_monthly_statistics(&data);
}

fn show_yearly_statistics(data: &Vec<models::Item>) {
    println!("年ごとの統計情報");
    let mut yearly_statistics = BTreeMap::new();

    for item in data {
        let year = item.get_year();
        let price = item.get_price_for_summary();

        yearly_statistics
            .entry(year)
            .and_modify(|sum| *sum += price)
            .or_insert(price);
    }

    for (year, price) in yearly_statistics {
        println!("{}年: {}", year, price);
    }
}

fn show_monthly_statistics(data: &Vec<models::Item>) {
    println!("月ごとの統計情報");
    let mut monthly_statistics = BTreeMap::new();

    for item in data {
        let month = item.get_month();
        let price = item.get_price_for_summary();

        monthly_statistics
            .entry(month)
            .and_modify(|sum| *sum += price)
            .or_insert(price);
    }
    for (month, price) in monthly_statistics {
        println!("{}月: {}", month, price);
    }
}
