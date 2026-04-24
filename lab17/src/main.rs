mod widgets;

use widgets::{Button, Label, Window, Widget};

fn main() {
    let mut window = Window::new("Демонстрация графического интерфейса Rust 1.23");
    window.add_widget(Box::new(Label::new("Это маленькая демонстрация графического интерфейса.")));
    window.add_widget(Box::new(Button::new("Нажми меня!")));
    window.draw();
}
