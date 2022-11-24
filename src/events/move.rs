use crate::event_system::{Job, Message};

pub struct MoveMessage {
    pub(crate) delta_x: i32,
    pub(crate) delta_y: i32,
}
impl Message for MoveMessage {}
pub struct MoveJob;
impl Job for MoveJob {
    type ItemMessage = MoveMessage;

    fn init() -> Self
    where
        Self: Sized,
    {
        MoveJob {}
    }

    fn process(message: &Self::ItemMessage) {
        println!("Moving by delta [{}, {}]", message.delta_x, message.delta_y);
    }
}
