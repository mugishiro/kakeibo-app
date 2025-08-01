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

    pub fn get_name(&self) -> &str {
        &self.name
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
    fn test_get_category() {
        let result = Item::get_category(0, 0);
        let expected = Category::Income(IncomeCategory::Salary);
        assert_eq!(result, expected);

        let result = Item::get_category(0, 1);
        let expected = Category::Income(IncomeCategory::Bonus);
        assert_eq!(result, expected);

        let result = Item::get_category(0, 2);
        let expected = Category::Income(IncomeCategory::Other);
        assert_eq!(result, expected);

        let result = Item::get_category(1, 0);
        let expected = Category::Expense(ExpenseCategory::Food);
        assert_eq!(result, expected);

        let result = Item::get_category(1, 1);
        let expected = Category::Expense(ExpenseCategory::Hobby);
        assert_eq!(result, expected);

        let result = Item::get_category(1, 2);
        let expected = Category::Expense(ExpenseCategory::Other);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_year() {
        let test_data = get_test_data();
        let expected = vec![2022, 2022, 2022, 2022, 2022];
        let result: Vec<i32> = test_data.iter().map(|data| data.get_year()).collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_month() {
        let test_data = get_test_data();
        let expected = vec![1, 1, 2, 4, 1];
        let result: Vec<u32> = test_data.iter().map(|data| data.get_month()).collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_first_day() {
        let test_data = get_test_data();
        let expected = vec![
            NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2022, 2, 1).unwrap(),
            NaiveDate::from_ymd_opt(2022, 4, 1).unwrap(),
            NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
        ];
        let result: Vec<NaiveDate> = test_data.iter().map(|data| data.get_first_day()).collect();
        assert_eq!(result, expected);
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
    }

    #[test]
    fn test_get_name() {
        let item = Item::new(
            "テスト商品".to_string(),
            Category::Income(IncomeCategory::Salary),
            1000,
            NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
        );
        assert_eq!(item.get_name(), "テスト商品");
    }
}
