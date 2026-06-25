use iced::window::Direction;
use iced::{Point, Size, window};

pub struct WindowChrome {
    pub window_id: Option<window::Id>,
    pub window_size: Size,
    pub cursor: Point,
}

impl Default for WindowChrome {
    fn default() -> Self {
        Self {
            window_id: None,
            window_size: Size::new(1200.0, 800.0),
            cursor: Point::ORIGIN,
        }
    }
}

pub enum WindowChromeMessage {
    Opened(window::Id),
    Resized(window::Id, Size),
    CursorMoved(Point),
    StartResize,
    Minimize,
    ToggleMaximize,
    Close,
}
impl WindowChrome {
    const BORDER: f32 = 8.0;
    pub fn update(
        state: &mut Self,
        message: WindowChromeMessage,
    ) -> iced::Task<WindowChromeMessage> {
        match message{
            WindowChromeMessage::Opened(id) => {
                state.window_id = Some(id);
            },
            WindowChromeMessage::Resized(id, size) => {
                state.window_size.width = size.width;
                state.window_size.height = size.height;
            },
            WindowChromeMessage::ToggleMaximize=>{

            },
            _=>todo!("WindowChrome update"),
        }

        iced::Task::none()
    }
    pub fn view(state: &'_ Self) -> iced::Element<'_, WindowChromeMessage> {
        iced::widget::column![].into()
    }
    fn resize_direction(cursor: Point, size: Size, border: f32) -> Option<Direction> {
        let left = cursor.x <= border;
        let right = cursor.x >= size.width - border;
        let top = cursor.y <= border;
        let bottom = cursor.y >= size.height - border;
        match (top, bottom, left, right) {
            (true, _, true, _) => Some(Direction::NorthWest),
            (true, _, _, true) => Some(Direction::NorthEast),
            (_, true, true, _) => Some(Direction::SouthWest),
            (_, true, _, true) => Some(Direction::SouthEast),
            (true, _, _, _) => Some(Direction::North),
            (_, true, _, _) => Some(Direction::South),
            (_, _, true, _) => Some(Direction::West),
            (_, _, _, true) => Some(Direction::East),
            _ => None,
        }
    }
}
