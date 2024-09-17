use std::thread; // Импортируем модуль thread для создания и управления потоками

fn main() {
    let n = 10; // Задаем количество чисел в исходном векторе
    let numbers: Vec<i32> = (1..=n).collect(); // Инициализируем массива 1..N
    let mut handles = vec![]; // создаем вектор handles, для хранения дескрипторов потоков

    for &num in &numbers {
        // Создаем новый поток для каждого числа
        let handle = thread::spawn(move || {
            let square = num * num; // Вычисление квадрата
            println!("{}", square); // Выводим квадрат в потоке
        });
        handles.push(handle); // Сохраняем дескриптор потока
    }
    
    // Ждем завершения всех потоков
    for handle in handles {
        let _ = handle.join();
    }
}
