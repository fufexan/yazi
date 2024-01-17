use ratatui::{buffer::Buffer, layout::{self, Constraint, Direction, Rect}, widgets::{List, ListItem, Widget}};
use yazi_config::THEME;

use crate::Ctx;

pub(super) struct Bindings<'a> {
	cx: &'a Ctx,
}

impl<'a> Bindings<'a> {
	pub(super) fn new(cx: &'a Ctx) -> Self { Self { cx } }
}

impl Widget for Bindings<'_> {
	fn render(self, area: Rect, buf: &mut Buffer) {
		let bindings = &self.cx.help.window();
		if bindings.is_empty() {
			return;
		}

		// On
		let col1 = bindings
			.iter()
			.map(|c| ListItem::new(c.on()).style(THEME.help.on.into()))
			.collect::<Vec<_>>();

		// Exec
		let col2 = bindings
			.iter()
			.map(|c| ListItem::new(c.exec()).style(THEME.help.exec.into()))
			.collect::<Vec<_>>();

		// Desc
		let col3 = bindings
			.iter()
			.map(|c| ListItem::new(c.desc.as_deref().unwrap_or("-")).style(THEME.help.desc.into()))
			.collect::<Vec<_>>();

		let chunks = layout::Layout::new(Direction::Horizontal, [
			Constraint::Ratio(2, 10),
			Constraint::Ratio(3, 10),
			Constraint::Ratio(5, 10),
		])
		.split(area);

		let cursor = self.cx.help.rel_cursor() as u16;
		buf.set_style(
			Rect { x: area.x, y: area.y + cursor, width: area.width, height: 1 },
			THEME.help.hovered.into(),
		);

		List::new(col1).render(chunks[0], buf);
		List::new(col2).render(chunks[1], buf);
		List::new(col3).render(chunks[2], buf);
	}
}
