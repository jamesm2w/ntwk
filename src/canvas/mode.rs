use iced::canvas::FillRule;
use iced::pure::widget::canvas::{Cursor, Fill, Frame, Geometry, Path, Stroke};
use iced::{Color, Point, Rectangle};

/// Defines the different Pen modes available for the canvas.
#[derive(Debug, Clone, Copy)]
pub enum Mode {
    View,
    PlaceNode,
    PlaceEdge(PlaceEdgeProgress),
    RemoveNode,
    RemoveEdge,
}

/// Defines progress through the Edge creation process (click startpoint, click endpoint)
#[derive(Debug, Clone, Copy)]
pub enum PlaceEdgeProgress {
    None,
    From { from: Point },
}

impl Default for Mode {
    fn default() -> Self {
        Mode::View
    }
}

impl Mode {
    pub fn draw(&self, bounds: Rectangle, cursor: Cursor) -> Geometry {
        let mut frame = Frame::new(bounds.size());

        if let Some(cursor_pos) = cursor.position_in(&bounds) {
            match self {
                Mode::PlaceNode => {
                    frame.fill(
                        &Path::circle(cursor_pos, 5.0),
                        Fill {
                            color: Color::BLACK,
                            rule: FillRule::EvenOdd,
                        },
                    );
                }
                Mode::PlaceEdge(progress) => match progress {
                    PlaceEdgeProgress::From { from } => {
                        frame.stroke(
                            &Path::line(*from, cursor_pos),
                            Stroke::default().with_width(2.0),
                        );
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        frame.into_geometry()
    }
}
