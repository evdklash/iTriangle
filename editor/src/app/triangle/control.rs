use crate::app::triangle::content::TriangleMessage;
use crate::app::main::{EditorApp, AppMessage};
use iced::{Alignment, Length};
use iced::widget::{Column, Container, pick_list, Row, Text, slider};

impl EditorApp {
    pub(crate) fn triangle_control(&self) -> Column<AppMessage> {
        let mode_pick_list =
            Row::new()
                .push(Text::new("Mode:")
                    .width(Length::Fixed(90.0))
                    .height(Length::Fill)
                    .align_y(Alignment::Center))
                .push(
                    Container::new(
                        pick_list(
                            &ModeOption::ALL[..],
                            Some(self.state.triangle.mode),
                            on_select_mode,
                        ).width(Length::Fixed(160.0))
                    )
                        .height(Length::Fill)
                        .align_y(Alignment::Center)
                ).height(Length::Fixed(40.0));

        let mut columns = Column::new()
            .push(mode_pick_list);
        
        if self.state.triangle.mode == ModeOption::Tessellation || self.state.triangle.mode == ModeOption::CentroidNet {
            let radius_list = Row::new()
                .push(
                    Text::new("Max Radius:")
                        .width(Length::Fixed(120.0))
                        .height(Length::Fill)
                        .align_y(Alignment::Center),
                )
                .push(
                    Container::new(
                        slider(2.0f64..=10.0f64, self.state.triangle.radius, on_update_radius).step(0.1f32)
                    )
                        .width(410)
                        .height(Length::Fill)
                        .align_y(Alignment::Center),
                )
                .height(Length::Fixed(40.0));
            let area_list = Row::new()
                .push(
                    Text::new("Max Area:")
                        .width(Length::Fixed(120.0))
                        .height(Length::Fill)
                        .align_y(Alignment::Center),
                )
                .push(
                    Container::new(
                        slider(10.0f64..=10000.0f64, self.state.triangle.max_area, on_update_area).step(0.1f32)
                    )
                        .width(410)
                        .height(Length::Fill)
                        .align_y(Alignment::Center),
                )
                .height(Length::Fixed(40.0));
            columns = columns.push(radius_list);
            columns = columns.push(area_list);
        }


        columns
    }
}

fn on_update_radius(value: f64) -> AppMessage {
    AppMessage::Triangle(TriangleMessage::RadiusUpdated(value))
}

fn on_update_area(value: f64) -> AppMessage {
    AppMessage::Triangle(TriangleMessage::AreaUpdated(value))
}

fn on_select_mode(option: ModeOption) -> AppMessage {
    AppMessage::Triangle(TriangleMessage::ModeSelected(option))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum ModeOption {
    #[default]
    Raw,
    Delaunay,
    Convex,
    Tessellation,
    CentroidNet,
}

impl ModeOption {
    const ALL: [ModeOption; 5] = [
        ModeOption::Raw,
        ModeOption::Delaunay,
        ModeOption::Convex,
        ModeOption::Tessellation,
        ModeOption::CentroidNet,
    ];
}

impl std::fmt::Display for ModeOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ModeOption::Raw => "Raw",
                ModeOption::Delaunay => "Delaunay",
                ModeOption::Convex => "Convex",
                ModeOption::Tessellation => "Tessellation",
                ModeOption::CentroidNet => "CentroidNet",
            }
        )
    }
}