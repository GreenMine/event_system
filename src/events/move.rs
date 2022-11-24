use crate::event_system::{Job, JobInit, Message};

pub struct MoveMessage {
    pub(crate) delta_x: i32,
    pub(crate) delta_y: i32,
}
impl Message for MoveMessage {}
pub struct MoveJob;
impl JobInit for MoveJob {
    fn init() -> Self
    where
        Self: Sized,
    {
        Self
    }
}
impl Job for MoveJob {
    type ItemMessage = MoveMessage;

    fn process(&mut self, message: &Self::ItemMessage) {
        println!("Moving by delta [{}, {}]", message.delta_x, message.delta_y);
    }
}
