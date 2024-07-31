use ratatui::text::{Line, Span};
use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph},
    Frame,
};
use specs::{Join, World, WorldExt};

use crate::app::App;
use crate::components::{Point, TrainColors, TrainHead, TrainParts};
use crate::map::Map;

/// Renders the user interface widgets.

fn render_map(ecs: &World) -> Paragraph {
    let map = ecs.fetch::<Map>();

    let train_parts = ecs.read_storage::<TrainParts>();
    let train_heads = ecs.read_storage::<TrainHead>();
    let train_colors = ecs.read_storage::<TrainColors>();

    let mut spans: Vec<Span> = Vec::new();

    for y in 0..map.h() {
        for x in 0..map.w() {
            let tile = map.get_tile_at_xy(Point::new(x, y));
            spans.push(Span::styled(
                String::from(char::from(tile)),
                Style::default().fg(Color::from(tile)),
            ))
        }
    }
    (&train_parts, &train_heads, &train_colors)
        .join()
        .for_each(|(t, h, c)| {
            t.parts.iter().for_each(|&p| {
                let i = map.xy_idx(p);
                let tile = map.get_tile_at_xy(p);
                spans[i] = Span::styled(
                    String::from(tile.get_train_char()),
                    Style::default().fg(if p == h.position {
                        c.head_color
                    } else {
                        c.main_color
                    }),
                )
            })
        });

    // All rendering happens up to this point
    let mut lines: Vec<Line> = Vec::with_capacity(map.h() as usize);
    spans
        .chunks(map.w() as usize)
        .for_each(|line| lines.push(Line::from(Vec::from(line))));

    Paragraph::new(lines)
}

pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    frame.render_widget(
        render_map(&app.ecs)
            .block(
                Block::bordered()
                    .title("Template")
                    .title_alignment(Alignment::Center)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
            .centered(),
        frame.size(),
    )
}
