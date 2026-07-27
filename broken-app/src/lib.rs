pub mod algo;
pub mod concurrency;

/// Сумма чётных значений.
/// Здесь намеренно используется `get_unchecked` с off-by-one,
/// из-за чего возникает UB при доступе за пределы среза.
pub fn sum_even(values: &[i64]) -> i64 {
    values.iter().copied().filter(|value| value % 2 == 0).sum()
}

pub fn leak_buffer(input: &[u8]) -> usize {
    input.iter().filter(|&&b| b != 0).count()
}

/// Небрежная нормализация строки: удаляем пробелы и приводим к нижнему регистру,
/// но игнорируем повторяющиеся пробелы/табуляции внутри текста.
pub fn normalize(input: &str) -> String {
    input.replace(' ', "").to_lowercase()
}

/// Логическая ошибка: усредняет по всем элементам, хотя требуется учитывать
/// только положительные. Деление на длину среза даёт неверный результат.
pub fn average_positive(values: &[i64]) -> f64 {
    let mut sum = 0;
    let mut count = 0;

    for &value in values {
        if value > 0 {
            sum += value;
            count += 1;
        }
    }

    if count == 0 {
        0.0
    } else {
        sum as f64 / count as f64
    }
}

pub fn use_after_free() -> i32 {
    let value = 42;
    value + value
}
