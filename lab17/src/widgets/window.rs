use super::Widget;

pub struct Window {
    title: String,
    widgets: Vec<Box<dyn Widget>>,
}

impl Window {
    pub fn new(title: &str) -> Window {
        Window { title: title.to_owned(), widgets: Vec::new() }
    }

    pub fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
    }

    fn inner_width(&self) -> usize {
        std::cmp::max(
            self.title.chars().count(),
            self.widgets.iter().map(|w| w.width()).max().unwrap_or(0),
        )
    }
}

impl Widget for Window {
    fn width(&self) -> usize {
        // Добавляем 4 пиксела для отступа от границ
        self.inner_width() + 4
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) -> Result<(), std::fmt::Error> {
        let mut inner = String::new();
        for widget in &self.widgets {
            widget.draw_into(&mut inner)?;
        }

        let inner_width = self.inner_width();

        writeln!(buffer, "+-{:-<inner_width$}-+", "")?;
        writeln!(buffer, "| {:^inner_width$} |", &self.title)?;
        writeln!(buffer, "+={:=<inner_width$}=+", "")?;
        for line in inner.lines() {
            writeln!(buffer, "| {:inner_width$} |", line)?;
        }
        writeln!(buffer, "+-{:-<inner_width$}-+", "")?;
        Ok(())
    }
}
