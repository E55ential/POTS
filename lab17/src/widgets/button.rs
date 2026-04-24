use super::Widget;
use super::Label;

pub struct Button {
    label: Label,
}

impl Button {
    pub fn new(label: &str) -> Button {
        Button { label: Label::new(label) }
    }
}

impl Widget for Button {
    fn width(&self) -> usize {
        self.label.width() + 8 // добавим немного отступа от границ
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) -> Result<(), std::fmt::Error> {
        let width = self.width();
        let mut label = String::new();
        self.label.draw_into(&mut label)?;

        writeln!(buffer, "+{:-<width$}+", "")?;
        for line in label.lines() {
            writeln!(buffer, "|{:^width$}|", &line)?;
        }
        writeln!(buffer, "+{:-<width$}+", "")?;
        Ok(())
    }
}
