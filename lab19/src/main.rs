pub fn luhn(cc_number: &str) -> bool {
    let mut sum = 0;
    let mut double = false;

    for c in cc_number.chars().rev() {
        if let Some(digit) = c.to_digit(10) {
            if double {
                let double_digit = digit * 2;
                sum +=
                    if double_digit > 9 { double_digit - 9 } else { double_digit };
            } else {
                sum += digit;
            }
            double = !double;
        } else {
            continue;
        }
    }

    sum % 10 == 0
}

#[cfg(test)]
mod test {
    use super::*;

    // ===== ИСХОДНЫЕ ТЕСТЫ =====
    
    #[test]
    fn test_valid_cc_number() {
        assert!(luhn("4263 9826 4026 9299"));
        assert!(luhn("4539 3195 0343 6467"));
        assert!(luhn("7992 7398 713"));
    }

    #[test]
    fn test_invalid_cc_number() {
        assert!(!luhn("4223 9826 4026 9299"));
        assert!(!luhn("4539 3195 0343 6476"));
        assert!(!luhn("8273 1232 7352 0569"));
    }

    // ===== НОВЫЕ ТЕСТЫ ДЛЯ ПОИСКА ОШИБОК =====

    #[test]
    fn test_single_digit_should_be_invalid() {
        // По правилам Луна номер должен иметь минимум 2 цифры
        // ОЖИДАЕТСЯ: false
        // БУДЕТ: true (sum = 5, 5 % 10 == 0? нет, вернёт false... хм, это пройдёт)
        // Но попробуем: luhn("0") -> sum = 0, 0 % 10 == 0 -> true! Это ошибка!
        assert!(!luhn("0"), "Одна цифра должна быть невалидна");
        assert!(!luhn("5"), "Одна цифра должна быть невалидна");
    }

    #[test]
    fn test_empty_string_should_be_invalid() {
        // Пустая строка не имеет цифр, sum = 0, 0 % 10 == 0 -> true!
        // Это ошибка!
        assert!(!luhn(""), "Пустая строка должна быть невалидна");
    }

    #[test]
    fn test_only_spaces_should_be_invalid() {
        // Только пробелы -> нет цифр -> sum = 0 -> 0 % 10 == 0 -> true!
        // Это ошибка!
        assert!(!luhn("   "), "Только пробелы должны быть невалидны");
    }

    #[test]
    fn test_no_digits_should_be_invalid() {
        // Нет цифр -> sum = 0 -> вернёт true!
        // Это ошибка!
        assert!(!luhn("abc"), "Строка без цифр должна быть невалидна");
        assert!(!luhn("a-b-c"), "Строка без цифр должна быть невалидна");
    }

    #[test]
    fn test_two_digits_valid() {
        // Минимальный валидный номер: 2 цифры
        // Проверка: "18" -> 1*2 + 8 = 10 -> valid
        assert!(luhn("18"), "18 должно быть валидно (1*2=2, 2+8=10)");
        assert!(luhn(" 1 8 "), "Пробелы должны игнорироваться");
    }

    #[test]
    fn test_two_digits_invalid() {
        // "12" -> 1*2 + 2 = 4 -> invalid
        assert!(!luhn("12"), "12 должно быть невалидно (1*2=2, 2+2=4)");
    }

    #[test]
    fn test_known_valid_numbers() {
        // Известные валидные номера для проверки алгоритма
        assert!(luhn("49927398716")); // Классический тестовый номер
        assert!(luhn("4992 7398 716")); // С пробелами
    }

    #[test]
    fn test_known_invalid_numbers() {
        assert!(!luhn("49927398717")); // Изменили последнюю цифру
        assert!(!luhn("1234567812345678")); // Невалидный номер
    }
}

fn main() {
    // Демонстрация багов
    println!("=== Демонстрация багов в исходном коде ===\n");
    
    println!("luhn(\"\") = {} (должен быть false)", luhn(""));
    println!("luhn(\"   \") = {} (должен быть false)", luhn("   "));
    println!("luhn(\"abc\") = {} (должен быть false)", luhn("abc"));
    println!("luhn(\"0\") = {} (должен быть false)", luhn("0"));
    
    println!("\n=== Правильная работа ===\n");
    
    println!("luhn(\"18\") = {} (должен быть true)", luhn("18"));
    println!("luhn(\"49927398716\") = {} (должен быть true)", luhn("49927398716"));
}
