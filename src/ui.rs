use iced::pure::widget::canvas;
use iced::pure::widget::Button;
use iced::pure::widget::Canvas;
use iced::pure::widget::Column;
use iced::pure::widget::Row;
use iced::pure::Element;
use iced::pure::Sandbox;
use iced::Alignment;
use iced::Point;

use crate::canvas::mode::{Mode, PlaceCurveProgress, PlaceEdgeProgress};
use crate::canvas::NetworkCanvas;
use crate::netwk::NetworkGraph;

/// This creates the UI for the application.
/// NetworkUI holds any data needed by the application.
/// Message defines the message passing protocol from the components to the internal state

#[derive(Default)]
pub struct NetworkUI {
    pen_mode: Option<Mode>,
    graph: NetworkGraph,
    canvas_cache: canvas::Cache,
}

#[derive(Debug, Clone)]
pub enum Message {
    Ack,
    ChangePenMode(Mode),
    AddNode(Point),
    RemoveNode,
    AddEdge(Point, Point),
    AddCurve(Point, Point, Point),
    RemoveEdge,
    EditNode,
    EditEdge,
    Clear,
}

impl Sandbox for NetworkUI {
    type Message = Message;

    fn new() -> Self {
        NetworkUI::default()
    }

    fn title(&self) -> String {
        String::from("ntwk ui")
    }

    fn update(&mut self, message: Self::Message) {
        self.pen_mode = match message {
            Message::Clear => {
                self.graph = NetworkGraph::default();
                self.canvas_cache.clear();

                None
            }
            Message::AddNode(point) => {
                self.graph.add_node(&point);
                self.canvas_cache.clear();

                None
            }
            Message::AddEdge(from, to) => {
                if let Some(point_from) = self.graph.get_exact_point(from) {
                    if let Some(point_to) = self.graph.get_exact_point(to) {
                        self.graph.add_edge(&point_from, &point_to);

                        self.canvas_cache.clear();
                    }
                }

                None
            }
            Message::AddCurve(from, to, control) => {
                if let Some(from) = self.graph.get_exact_point(from) {
                    if let Some(to) = self.graph.get_exact_point(to) {
                        self.graph.add_curve(&from, &to, control);
                        self.canvas_cache.clear();
                    }
                }

                None
            }
            Message::ChangePenMode(mode) => Some(mode),
            _ => None,
        };
        // If we didn't want to change the mode of the canvas pen here then set it to None so it doesnt change.
    }

    fn view(&self) -> Element<'_, Self::Message> {
        Column::new()
            .padding(20)
            .align_items(Alignment::Center)
            .push(
                Row::new()
                    .padding(20)
                    .align_items(Alignment::Fill)
                    .push(Button::new("Add Node").on_press(Message::ChangePenMode(Mode::PlaceNode)))
                    .push(Button::new("Add Edge").on_press(Message::ChangePenMode(
                        Mode::PlaceEdge(PlaceEdgeProgress::None),
                    )))
                    .push(Button::new("Add Curve").on_press(Message::ChangePenMode(
                        Mode::PlaceCurve(PlaceCurveProgress::None),
                    )))
                    .push(Button::new("Clear").on_press(Message::Clear)),
            )
            .push(
                Canvas::new(NetworkCanvas::new(
                    &self.graph,
                    &self.canvas_cache,
                    &self.pen_mode,
                ))
                .width(iced::Length::Fill)
                .height(iced::Length::Fill),
            )
            .into()
    }
}
