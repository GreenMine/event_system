#![feature(box_syntax)]

mod event_system;
mod events;

use crate::event_system::{Job, Subscriber};
use crate::events::{
    hit::{HitJob, HitMessage, Weapon},
    r#move::{MoveJob, MoveMessage},
};
use std::{fmt::Debug, time::Instant};

pub struct CustomMoveJob {
    total_moves: u64,
}
impl Job for CustomMoveJob {
    type ItemMessage = MoveMessage;

    fn init() -> Self
    where
        Self: Sized,
    {
        CustomMoveJob { total_moves: 0 }
    }

    fn process(&mut self, message: &Self::ItemMessage) {
        self.total_moves += (message.delta_x.abs() + message.delta_y.abs()) as u64;
        println!("Total moves: {}", self.total_moves);
    }
}

#[tokio::main]
async fn main() {
    let n = Instant::now();
    let mut subscriber = Subscriber::new();
    subscriber.add_uninit_handler::<MoveJob>();
    let custom_handler = subscriber.add_uninit_handler::<CustomMoveJob>();
    subscriber.add_uninit_handler::<HitJob>();

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
            if i * 8 + j == 32 {
                subscriber.remove_handler(custom_handler);
            }
            subscriber.run(MoveMessage {
                delta_x: i,
                delta_y: j,
            });
        }
    }

    println!("elapsed: {:?}", n.elapsed().as_micros());
}
