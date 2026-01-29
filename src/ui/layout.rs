use ratatui::layout::{Constraint, Direction, Layout as RatatuiLayout, Rect};

pub struct Layout;

impl Layout {
    pub fn split_horizontal(area: Rect) -> Vec<Rect> {
        RatatuiLayout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .split(area)
            .to_vec()
    }

    pub fn split_dual_pane(area: Rect) -> Vec<Rect> {
        Self::split_dual_pane_with_ratio(area, &[1, 1])
    }

    pub fn split_dual_pane_with_ratio(area: Rect, ratio: &[u16]) -> Vec<Rect> {
        let total: u16 = ratio.iter().sum();
        let left_percent = if total > 0 {
            (ratio[0] as f32 / total as f32 * 100.0) as u16
        } else {
            50
        };
        let right_percent = 100 - left_percent;

        RatatuiLayout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(left_percent),
                Constraint::Percentage(right_percent),
            ])
            .split(area)
            .to_vec()
    }
}
