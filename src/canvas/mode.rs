use iced::canvas::FillRule;
use iced::pure::widget::canvas::{Cursor, Fill, Frame, Geometry, Path, Stroke};
use iced::{Color, Point, Rectangle};

/// Defines the different Pen modes available for the canvas.
#[derive(Debug, Clone, Copy)]
pub enum Mode {
    View,
    PlaceNode,
    PlaceEdge(PlaceEdgeProgress),
    PlaceCurve(PlaceCurveProgress),
    RemoveNode,
    RemoveEdge,
}

/// Defines progress through the Edge creation process (click startpoint, click endpoint)
#[derive(Debug, Clone, Copy)]
pub enum PlaceEdgeProgress {
    None,
    From { from: Point },
}

#[derive(Debug, Clone, Copy)]
pub enum PlaceCurveProgress {
    None,
    From { from: Point },
    To { from: Point, to: Point }
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
                },
                
                Mode::PlaceEdge(progress) => match progress {
                    PlaceEdgeProgress::From { from } => {
                        frame.stroke(
                            &Path::line(*from, cursor_pos),
                            Stroke::default().with_width(2.0),
                        );
                    }
                    _ => {}
                },

                Mode::PlaceCurve(progress) => match progress {
                    PlaceCurveProgress::None => {},
                    PlaceCurveProgress::From { from } => {
                        frame.stroke(
                            &Path::line(*from, cursor_pos),
                            Stroke::default().with_width(2.0)
                        )
                    },
                    PlaceCurveProgress::To { from, to } => {
                        frame.stroke(
                            &Path::new(|f| {
                                f.move_to(*from);
                                f.quadratic_curve_to(cursor_pos, *to);
                            }),
                            Stroke::default().with_width(2.0)
                        )
                    }
                },
                _ => {}
            }
        }

        frame.into_geometry()
    }
}
