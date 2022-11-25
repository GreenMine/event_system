use crate::event_system::{Event, Handler, HandlerInit};

pub struct MoveEvent {
    pub(crate) delta_x: i32,
    pub(crate) delta_y: i32,
}
impl Event for MoveEvent {}
pub struct MoveHandler;
impl HandlerInit for MoveHandler {
    fn init() -> Self
    where
        Self: Sized,
    {
        Self
    }
}
impl Handler for MoveHandler {
    type Event = MoveEvent;

    fn process(&mut self, message: &Self::Event) {
        println!("Moving by delta [{}, {}]", message.delta_x, message.delta_y);
    }
}
