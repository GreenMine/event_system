mod event_system;
mod events;

use crate::event_system::{Handler, HandlerInit, Subscriber};
use crate::events::{
    hit::{HitEvent, HitHandler, Weapon},
    r#move::{MoveEvent, MoveHandler},
};
use std::{fmt::Debug, time::Instant};

pub struct CustomMoveHandler {
    total_moves: u64,
}
impl HandlerInit for CustomMoveHandler {
    fn init() -> Self
    where
        Self: Sized,
    {
        Self { total_moves: 0 }
    }
}
impl Handler for CustomMoveHandler {
    type Event = MoveEvent;

    fn process(&mut self, message: &Self::Event) {
        self.total_moves += (message.delta_x.abs() + message.delta_y.abs()) as u64;
    }
}

fn main() {
    let n = Instant::now();
    let mut subscriber = Subscriber::new();
    subscriber.add_uninit_handler::<MoveHandler>();
    subscriber.add_uninit_handler::<HitHandler>();

    let job = CustomMoveHandler { total_moves: 100 };
    subscriber.add_handler(job);

    subscriber.run(MoveEvent {
        delta_x: 10,
        delta_y: 1,
    });

    subscriber.run(HitEvent {
        player_id: 1,
        enemy_id: 2,
        weapon: Weapon::Knife,
        distance: 10,
    });

    for i in 0..16 {
        for j in 0..16 {
            subscriber.run(MoveEvent {
                delta_x: i,
                delta_y: j,
            });
        }
    }

    subscriber.run(HitEvent {
        player_id: 1,
        enemy_id: 2,
        weapon: Weapon::Knife,
        distance: 10,
    });
    println!("elapsed: {:?}", n.elapsed().as_millis());
}
