mod event_system;
mod events;

use crate::event_system::{Job, JobInit, Subscriber};
use crate::events::{
    hit::{HitJob, HitMessage, Weapon},
    r#move::{MoveJob, MoveMessage},
};
use std::{fmt::Debug, time::Instant};

pub struct CustomMoveJob {
    total_moves: u64,
}
impl JobInit for CustomMoveJob {
    fn init() -> Self
    where
        Self: Sized,
    {
        Self { total_moves: 0 }
    }
}
impl Job for CustomMoveJob {
    type ItemMessage = MoveMessage;

    fn process(&mut self, message: &Self::ItemMessage) {
        self.total_moves += (message.delta_x.abs() + message.delta_y.abs()) as u64;
    }
}

impl Drop for CustomMoveJob {
    fn drop(&mut self) {
        println!("Total moves: {}", self.total_moves);
    }
}

#[tokio::main]
async fn main() {
    let n = Instant::now();
    let mut subscriber = Subscriber::new();
    subscriber.add_uninit_handler::<MoveJob>();
    subscriber.add_uninit_handler::<HitJob>();

    let job = CustomMoveJob { total_moves: 100 };
    let custom_handler = subscriber.add_handler(job);

    subscriber.run(MoveMessage {
        delta_x: 10,
        delta_y: 1,
    });

    for i in 0..8 {
        for j in 0..8 {
            subscriber.run(MoveMessage {
                delta_x: i,
                delta_y: j,
            });
        }
    }

    subscriber.run(HitMessage {
        player_id: 1,
        enemy_id: 2,
        weapon: Weapon::Knife,
        distance: 10,
    });

    println!("elapsed: {:?}", n.elapsed().as_micros());
}
