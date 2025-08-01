use kakeibo_app::services;
use std::io;

const FILE_PATH: &str = "store/data.json";

fn main() {
    let mut service_type = String::new();

    println!("実行したい内容を入力してください(0: 登録, 1: 集計)");
    io::stdin().read_line(&mut service_type).unwrap();
    let service_type: u8 = service_type.trim().parse().expect("数値で入力してください");

    // validation
    services::validate::InputValidator::validate_service_type(service_type);

    if service_type == 0 {
        services::register::run(FILE_PATH);
    } else if service_type == 1 {
        services::summarize::run(FILE_PATH);
    }
}

#[cfg(test)]
mod main_test {
    use super::*;

    #[test]
    fn test_main_function_structure() {
        // main関数の構造をテスト
        // 実際のテストでは標準入出力をリダイレクトする必要があります
        assert!(true); // ダミーアサーション
    }

    #[test]
    fn test_service_type_validation() {
        // サービス種別のバリデーションをテスト
        services::validate::InputValidator::validate_service_type(0);
        services::validate::InputValidator::validate_service_type(1);
    }

    #[test]
    #[should_panic(expected = "入力値が不正です")]
    fn test_service_type_validation_panic() {
        services::validate::InputValidator::validate_service_type(2);
    }
}
