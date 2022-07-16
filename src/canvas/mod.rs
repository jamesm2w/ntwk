use crate::netwk::NetworkGraph;
use crate::ui::Message;
use iced::{
    canvas::Event,
    pure::widget::canvas::{self, Cursor, Fill, FillRule, Frame, Path, Program, Stroke, event::{self, Status}},
    Color,
};
use iced::{mouse, Point};

use self::mode::{PlaceEdgeProgress, PlaceCurveProgress};

pub mod mode;

/// This controls the actual canvas which is rendered for the user.
/// NetworkCanvas struct should hold reference to any external data needed.
/// while State holds the internal state which can't be modified from outside.

#[derive(Default)]
pub struct State {
    mode: mode::Mode,
}

pub struct NetworkCanvas<'a> {
    graph: &'a NetworkGraph,
    cache: &'a canvas::Cache,
    pen_mode: &'a Option<mode::Mode>
}

impl<'a> NetworkCanvas<'a> {
    pub fn new(graph: &'a NetworkGraph, cache: &'a canvas::Cache, pen_mode: &'a Option<mode::Mode>) -> Self {
        NetworkCanvas { graph, cache, pen_mode }
    }
}

impl<'a> Program<Message> for NetworkCanvas<'a> {
    type State = State;

    fn update(
        &self,
        state: &mut Self::State,
        event: iced::canvas::Event,
        bounds: iced::Rectangle,
        cursor: iced::canvas::Cursor,
    ) -> (Status, Option<Message>) {
        let cursor_pos = if let Some(position) = cursor.position_in(&bounds) {
            position
        } else {
            return (Status::Ignored, None);
        };

        if let Some(new_mode) = self.pen_mode {
            state.mode = *new_mode;
        }

        match event {
            Event::Mouse(mouse_event) => {
                let message = match mouse_event {
                    mouse::Event::ButtonPressed(mouse::Button::Left) => match &state.mode {
                        mode::Mode::View => None,
                        mode::Mode::PlaceNode => {
                            state.mode = mode::Mode::View;

                            Some(Message::AddNode(cursor_pos))
                        },
                        mode::Mode::PlaceEdge(progress) => {
                            match progress {
                                PlaceEdgeProgress::None => {
                                    state.mode = mode::Mode::PlaceEdge( match self.graph.get_near_point(cursor_pos) {
                                        Some(from) => (mode::PlaceEdgeProgress::From { from: *from.data() }),
                                        None => mode::PlaceEdgeProgress::None
                                    });
                                    
                                    Some(Message::Ack) // ack so we dont reset progress
                                },
                                PlaceEdgeProgress::From { from } => {
                                    match self.graph.get_near_point(cursor_pos) {
                                        Some(to) => {
                                            let from = from.clone();
                                            let to = *to.data();
                                            state.mode = mode::Mode::View;

                                            Some(Message::AddEdge(from, to))
                                        },
                                        None => None
                                    }
                                }
                            }
                        },

                        mode::Mode::PlaceCurve(progress) => {
                            match progress {
                                PlaceCurveProgress::None => {
                                    state.mode = mode::Mode::PlaceCurve(match self.graph.get_near_point(cursor_pos) {
                                        Some(from) => (mode::PlaceCurveProgress::From { from: *from.data() }),
                                        None => mode::PlaceCurveProgress::None
                                    });

                                    Some(Message::Ack)
                                },
                                PlaceCurveProgress::From { from } => {

                                    state.mode = mode::Mode::PlaceCurve(match self.graph.get_near_point(cursor_pos) {
                                        Some(to) => (mode::PlaceCurveProgress::To { from: *from, to: *to.data() }),
                                        None => mode::PlaceCurveProgress::From { from: *from }
                                    });

                                    Some(Message::Ack)
                                },
                                PlaceCurveProgress::To { from, to } => {
                                    let from = *from;
                                    let to = *to;

                                    state.mode = mode::Mode::View;

                                    Some(Message::AddCurve(from, to, cursor_pos))
                                }
                            }
                        } // end place curve mode

                        // Other modes no action     
                        _ => None
                    },
                    //  other mouse events need no message response
                    _ => None,
                };

                (event::Status::Captured, message) // send back message (if generated)
            }
            // dont care about non-mouse events
            _ => return (event::Status::Ignored, None),
        }
    }

    fn draw(
        &self,
        state: &Self::State,
        bounds: iced::Rectangle,
        cursor: Cursor,
    ) -> Vec<iced::canvas::Geometry> {
        let content = self.cache.draw(bounds.size(), |frame: &mut Frame| {
            for node in self.graph.node_list() {
                frame.fill(
                    &Path::circle(*node.data(), 5.0),
                    Fill {
                        color: Color::BLACK,
                        rule: FillRule::EvenOdd,
                    },
                );

                for conn in node.edges().borrow().iter() {

                    let point = conn.destination();

                    let path = match conn.control() {
                        None => Path::line(*node.data(), *point.data()),
                        Some(cpoint) => Path::new(|f| {
                            f.move_to(*node.data());
                            f.quadratic_curve_to(*cpoint, *point.data())
                        }),
                    };

                    frame.stroke(&path, Stroke::default().with_width(2.0));
                }
            }


            // Draw outline over canvas
            frame.stroke(
                &Path::rectangle(Point::ORIGIN, frame.size()),
                Stroke::default()
            );
        });

        let pen = state.mode.draw(bounds, cursor);

        vec![content, pen]
    }
}
