mod ui {
    use ratatui::widgets::{Paragraph, Block, Borders};
    use ratatui::layout::{Layout, Constraint, Direction};
    use ratatui::text::{Span, Spans};
    use ratatui::backend::Backend;
    use ratatui::Terminal;
    use std::io;

    pub fn show_profile<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default().borders(Borders::ALL).title("Profile");
            let text = vec![
                Spans::from(vec![Span::styled("Alias: Solarian", Style::default())]),
                Spans::from(vec![Span::styled("Skills: Eco-building, Coding", Style::default())]),
                Spans::from(vec![Span::styled("Interests: Rebellion, Nature", Style::default())]),
            ];
            let paragraph = Paragraph::new(text).block(block);
            f.render_widget(paragraph, size);
        })
    }
}
