pub trait Logger {
    /// Помещает в лог сообщения заданного уровня.
    fn log(&self, verbosity: u8, message: &str);
}

struct StderrLogger;

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: &str) {
        eprintln!("verbosity={verbosity}: {message}");
    }
}

/// Фильтр с замыканием
struct Filter<L, F> {
    inner: L,
    filter: F,
}

impl<L, F> Filter<L, F> {
    fn new(inner: L, filter: F) -> Self {
        Self { inner, filter }
    }
}

impl<L, F> Logger for Filter<L, F>
where
    L: Logger,
    F: Fn(u8, &str) -> bool,
{
    fn log(&self, verbosity: u8, message: &str) {
        if (self.filter)(verbosity, message) {
            self.inner.log(verbosity, message);
        }
    }
}

fn main() {
    let logger = Filter::new(
        StderrLogger,
        |_verbosity, msg| msg.contains("yikes"),
    );

    logger.log(5, "FYI"); // не выведется
    logger.log(1, "yikes, something went wrong"); // выведется
    logger.log(2, "uhoh"); // не выведется
}