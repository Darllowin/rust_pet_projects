use std::io;

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponentiation,
}

impl Operation {
    fn from_u8(n: u8) -> Option<Self> {
        match n {
            1 => Some(Operation::Add),
            2 => Some(Operation::Subtract),
            3 => Some(Operation::Multiply),
            4 => Some(Operation::Divide),
            5 => Some(Operation::Exponentiation),
            _ => None,
        }
    }

    fn calculate(&self, a: f64, b: f64) -> Result<f64, String> {
        match self {
            Operation::Add => Ok(a + b),
            Operation::Subtract => Ok(a - b),
            Operation::Multiply => Ok(a * b),
            Operation::Divide => {
                if b == 0.0 {
                    Err("Ошибка: деление на ноль.".to_string())
                } else {
                    Ok(a / b)
                }
            }
            Operation::Exponentiation => Ok(a.powf(b)),
        }
    }
}

fn main() {
    println!("Добро пожаловать в калькулятор!");

    loop {
        println!("\nПожалуйста, выберите операцию:");
        println!("1. Сложение (+)");
        println!("2. Вычитание (-)");
        println!("3. Умножение (*)");
        println!("4. Деление (/)");
        println!("5. Возведение в степень (^)");
        println!("Введите номер операции (или 'выход' для завершения):");

        let operation = match read_operation() {
            Some(op) => op,
            None => {
                println!("Завершение работы калькулятора. До свидания!");
                break;
            }
        };

        // Чтение двух чисел с подсказками
        let (num1, num2) = match read_two_numbers() {
            Ok(nums) => nums,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        // Выполнение операции
        match operation.calculate(num1, num2) {
            Ok(result) => println!("Результат: {}", result),
            Err(e) => println!("{}", e),
        }

        // Спрашиваем о продолжении
        println!("\nХотите выполнить еще один расчет? Введите 'да' или 'нет':");
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Ошибка при чтении");
        if answer.trim().to_lowercase() != "да" && answer.trim().to_lowercase() != "yes" {
            println!("Спасибо за использование калькулятора! До свидания!");
            break;
        }
    }
}

fn read_operation() -> Option<Operation> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Ошибка при чтении");
    let trimmed = input.trim();

    if trimmed.eq_ignore_ascii_case("выход") || trimmed.eq_ignore_ascii_case("exit") {
        return None;
    }

    match trimmed.parse::<u8>() {
        Ok(n) => Operation::from_u8(n),
        Err(_) => {
            println!(
                "Некорректный ввод. Пожалуйста, введите номер операции от 1 до 5 или 'выход'."
            );
            None
        }
    }
}

fn read_two_numbers() -> Result<(f64, f64), String> {
    loop {
        println!("Введите два числа через пробел (или 'выход' для завершения):");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Ошибка при чтении");

        if input.trim().eq_ignore_ascii_case("выход") || input.trim().eq_ignore_ascii_case("exit")
        {
            return Err("Завершение по желанию пользователя.".to_string());
        }

        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.len() != 2 {
            println!("Пожалуйста, введите ровно два числа через пробел.");
            continue;
        }

        let num1 = match parts[0].parse::<f64>() {
            Ok(n) => n,
            Err(_) => {
                println!("Некорректное первое число. Попробуйте снова.");
                continue;
            }
        };

        let num2 = match parts[1].parse::<f64>() {
            Ok(n) => n,
            Err(_) => {
                println!("Некорректное второе число. Попробуйте снова.");
                continue;
            }
        };

        return Ok((num1, num2));
    }
}
