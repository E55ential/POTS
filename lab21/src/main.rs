use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Fork;

struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    left_index: usize,
    right_index: usize,
    thoughts: mpsc::Sender<String>,
}

impl Philosopher {
    fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
    }

    fn eat(&self) {
        // Берём левую вилку, затем правую
        let _left = self.left_fork.lock().unwrap();
        println!("{} взял(а) левую вилку {}", &self.name, self.left_index);
        let _right = self.right_fork.lock().unwrap();
        println!("{} взял(а) правую вилку {}", &self.name, self.right_index);

        println!("{} ест ...    [вилки {} и {}]", &self.name, self.left_index, self.right_index);
        thread::sleep(Duration::from_millis(10));
    }
}

static PHILOSOPHERS: &[&str] =
    &["Сократ", "Гипатия", "Платон", "Аристотель", "Пифагор"];

fn main() {
    // Создать вилки (5 вилок для 5 философов)
    let forks: Vec<Arc<Mutex<Fork>>> = (0..5)
        .map(|_| Arc::new(Mutex::new(Fork)))
        .collect();

    // Канал для сбора мыслей
    let (tx, rx) = mpsc::channel();

    // Создать философов и запустить их в отдельных потоках
    let mut handles = vec![];

    for i in 0..5 {
        // Чтобы избежать дедлока, последний философ берёт вилки
        // в обратном порядке (сначала правую, потом левую).
        // Это разрывает циклическое ожидание.
        let left = Arc::clone(&forks[i]);
        let right = Arc::clone(&forks[(i + 1) % 5]);

        let philosopher = if i == 4 {
            // Последний философ берёт сначала правую, потом левую
            // (вилки в полях left/right поменяны местами -> разрыв цикла!)
            Philosopher {
                name: PHILOSOPHERS[i].to_string(),
                left_fork: right,
                right_fork: left,
                left_index: (i + 1) % 5,
                right_index: i,
                thoughts: tx.clone(),
            }
        } else {
            Philosopher {
                name: PHILOSOPHERS[i].to_string(),
                left_fork: left,
                right_fork: right,
                left_index: i,
                right_index: (i + 1) % 5,
                thoughts: tx.clone(),
            }
        };

        let handle = thread::spawn(move || {
            for _ in 0..100 {
                philosopher.think();
                philosopher.eat();
            }
        });

        handles.push(handle);
    }

    // Убить оригинальный отправитель, чтобы канал закрылся
    // после завершения всех потоков
    drop(tx);

    // Дождаться завершения всех философов
    for handle in handles {
        handle.join().unwrap();
    }

    // Вывести их мысли (канал закроется, и цикл завершится)
    for thought in rx {
        println!("{}", thought);
    }
}
