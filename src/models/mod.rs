use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum IncomeCategory {
    Salary,
    Bonus,
    Other,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ExpenseCategory {
    Food,
    Hobby,
    Other,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Category {
    Income(IncomeCategory),
    Expense(ExpenseCategory),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Item {
    name: String,
    category: Category,
    price: u32,
    date: NaiveDate,
}

impl Item {
    pub fn new(name: String, category: Category, price: u32, date: NaiveDate) -> Self {
        Item {
            name,
            category,
            price,
            date,
        }
    }

    pub fn get_category(register_type: u8, category_type: u8) -> Category {
        if register_type == 0 {
            match category_type {
                0 => Category::Income(IncomeCategory::Salary),
                1 => Category::Income(IncomeCategory::Bonus),
                2 => Category::Income(IncomeCategory::Other),
                _ => panic!("不正なカテゴリ種別です"),
            }
        } else {
            match category_type {
                0 => Category::Expense(ExpenseCategory::Food),
                1 => Category::Expense(ExpenseCategory::Hobby),
                2 => Category::Expense(ExpenseCategory::Other),
                _ => panic!("不正なカテゴリ種別です"),
            }
        }
    }

    pub fn get_year(&self) -> i32 {
        self.date.year()
    }

    pub fn get_month(&self) -> u32 {
        self.date.month()
    }

    pub fn get_first_day(&self) -> NaiveDate {
        NaiveDate::from_ymd_opt(self.get_year(), self.get_month(), 1).unwrap()
    }

    pub fn get_price_for_summary(&self) -> i32 {
        match self.category {
            Category::Income(_) => self.price as i32,
            Category::Expense(_) => -1 * self.price as i32,
        }
    }
}

#[cfg(test)]
mod models_test {
    use super::*;

    fn get_test_data() -> Vec<Item> {
        vec![
            Item::new(
                "新年会".to_string(),
                Category::Expense(ExpenseCategory::Food),
                5000,
                NaiveDate::from_ymd_opt(2022, 1, 10).unwrap(),
            ),
            Item::new(
                "給料".to_string(),
                Category::Income(IncomeCategory::Salary),
                300000,
                NaiveDate::from_ymd_opt(2022, 1, 20).unwrap(),
            ),
            Item::new(
                "外食".to_string(),
                Category::Expense(ExpenseCategory::Food),
                3000,
                NaiveDate::from_ymd_opt(2022, 2, 15).unwrap(),
            ),
            Item::new(
                "歓迎会".to_string(),
                Category::Expense(ExpenseCategory::Other),
                10000,
                NaiveDate::from_ymd_opt(2022, 4, 15).unwrap(),
            ),
            Item::new(
                "旅行".to_string(),
                Category::Expense(ExpenseCategory::Hobby),
                100000,
                NaiveDate::from_ymd_opt(2022, 1, 30).unwrap(),
            ),
        ]
    }

    #[test]
    fn test_get_price_for_summary() {
        let test_data = get_test_data();
        let expected = vec![-5000, 300000, -3000, -10000, -100000];
        let result: Vec<i32> = test_data
            .iter()
            .map(|data| data.get_price_for_summary())
            .collect();
        assert_eq!(result, expected);
        // for (data, &expected_price) in test_data.iter().zip(expected.iter()) {
        //     assert_eq!(data.get_price_for_summary(), expected_price);
        // }
    }
}
