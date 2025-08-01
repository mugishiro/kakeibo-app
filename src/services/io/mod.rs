use crate::models;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn read_data_or_create_new_data(file_path: &str) -> Vec<models::Item> {
    let file = File::open(file_path);
    match file {
        Ok(f) => {
            let buf_reader = BufReader::new(f);
            serde_json::from_reader(buf_reader).expect("デシリアライズに失敗しました")
        }
        Err(_) => {
            println!("新規ファイルを作成します");
            Vec::new()
        }
    }
}

pub fn read_date_or_panic(file_path: &str) -> Vec<models::Item> {
    let file = File::open(file_path).expect("ファイルがオープンできませんでした");
    let buf_reader = BufReader::new(file);
    let data: Vec<models::Item> =
        serde_json::from_reader(buf_reader).expect("デシリアライズに失敗しました");
    if data.len() == 0 {
        panic!("データが存在しません");
    }
    data
}

pub fn write_to_json(data: &Vec<models::Item>, file_path: &str) {
    let json_data = serde_json::to_string_pretty(data).expect("JSONのシリアライズに失敗しました");
    let mut file = File::create(file_path).expect("書き込みファイルのオープンに失敗しました");
    writeln!(file, "{}", json_data).expect("ファイルへの書き込みに失敗しました");
    println!("項目の登録が完了しました");
}

#[cfg(test)]
mod io_test {
    use super::*;
    use chrono::NaiveDate;
    use std::fs;

    fn create_test_data() -> Vec<models::Item> {
        vec![models::Item::new(
            "テスト".to_string(),
            models::Category::Income(models::IncomeCategory::Salary),
            1000,
            NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
        )]
    }

    #[test]
    fn test_read_data_or_create_new_data_existing_file() {
        let test_data = create_test_data();
        let test_file = "test_data.json";

        // テストファイルを作成
        write_to_json(&test_data, test_file);

        let result = read_data_or_create_new_data(test_file);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].get_name(), "テスト");

        // テストファイルを削除
        fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_read_data_or_create_new_data_new_file() {
        let test_file = "non_existent_file.json";

        let result = read_data_or_create_new_data(test_file);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_read_date_or_panic_existing_file() {
        let test_data = create_test_data();
        let test_file = "test_data.json";

        // テストファイルを作成
        write_to_json(&test_data, test_file);

        let result = read_date_or_panic(test_file);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].get_name(), "テスト");

        // テストファイルを削除
        fs::remove_file(test_file).unwrap();
    }

    #[test]
    #[should_panic(expected = "ファイルがオープンできませんでした")]
    fn test_read_date_or_panic_file_not_found() {
        read_date_or_panic("non_existent_file.json");
    }

    #[test]
    #[should_panic(expected = "データが存在しません")]
    fn test_read_date_or_panic_empty_file() {
        let test_file = "empty_test_data.json";
        let empty_data: Vec<models::Item> = vec![];

        // 空のファイルを作成
        write_to_json(&empty_data, test_file);

        read_date_or_panic(test_file);

        // テストファイルを削除
        fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_write_to_json() {
        let test_data = create_test_data();
        let test_file = "write_test_data.json";

        write_to_json(&test_data, test_file);

        // 書き込まれたファイルを読み込んで検証
        let result = read_data_or_create_new_data(test_file);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].get_name(), "テスト");

        // テストファイルを削除
        fs::remove_file(test_file).unwrap();
    }
}
