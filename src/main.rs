mod event_system;
mod events;

use crate::event_system::{Job, Subscriber};
use crate::events::{
    hit::{HitJob, HitMessage, Weapon},
    r#move::{MoveJob, MoveMessage},
};
use std::{fmt::Debug, time::Instant};

pub struct CustomMoveJob;
impl Job for CustomMoveJob {
    type ItemMessage = MoveMessage;

    fn init() -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn process(message: &Self::ItemMessage) {
        println!("custom move");
    }
}

#[tokio::main]
async fn main() {
    let n = Instant::now();
    let mut subscriber = Subscriber::new();
    subscriber.add_handler::<MoveJob>();
    subscriber.add_handler::<CustomMoveJob>();
    subscriber.add_handler::<HitJob>();

    subscriber.run(MoveMessage {
        delta_x: 10,
        delta_y: 1,
    });

    subscriber.run(HitMessage {
        player_id: 1,
        enemy_id: 2,
        weapon: Weapon::Knife,
        distance: 10,
    });

    for i in 0..8 {
        for j in 0..8 {
            subscriber.run(MoveMessage {
                delta_x: i,
                delta_y: j,
            });
        }
    }

    println!("elapsed: {:?}", n.elapsed().as_micros());
}
