pub struct InputValidator {}

impl InputValidator {
    pub fn validate_service_type(service_type: u8) {
        match service_type {
            0 | 1 | 2 => {}
            _ => panic!("入力値が不正です"),
        }
    }

    pub fn validate_register_type(register_type: u8) {
        match register_type {
            0 | 1 => {}
            _ => panic!("入力値が不正です"),
        }
    }

    pub fn validate_category_type(register_type: u8, category_type: u8) {
        if register_type == 0 {
            match category_type {
                0 | 1 | 2 => {}
                _ => panic!("入力値が不正です"),
            }
        } else {
            match category_type {
                0 | 1 | 2 => {}
                _ => panic!("入力値が不正です"),
            }
        }
    }
}

#[cfg(test)]
mod validate_test {
    use super::*;

    #[test]
    fn test_validate_service_type() {
        InputValidator::validate_service_type(0);
        InputValidator::validate_service_type(1);
        InputValidator::validate_service_type(2);
    }

    #[test]
    #[should_panic(expected = "入力値が不正です")]
    fn test_validate_service_type_panic() {
        InputValidator::validate_service_type(3);
    }

    #[test]
    fn test_validate_register_type() {
        InputValidator::validate_register_type(0);
        InputValidator::validate_register_type(1);
    }

    #[test]
    #[should_panic(expected = "入力値が不正です")]
    fn test_validate_register_type_panic() {
        InputValidator::validate_register_type(2);
    }

    #[test]
    fn test_validate_category_type() {
        InputValidator::validate_category_type(0, 0);
        InputValidator::validate_category_type(0, 1);
        InputValidator::validate_category_type(0, 2);
        InputValidator::validate_category_type(1, 0);
        InputValidator::validate_category_type(1, 1);
        InputValidator::validate_category_type(1, 2);
    }

    #[test]
    #[should_panic(expected = "入力値が不正です")]
    fn test_validate_category_type_panic() {
        InputValidator::validate_category_type(0, 3);
        InputValidator::validate_category_type(1, 3);
    }
}
