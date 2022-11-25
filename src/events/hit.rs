use crate::event_system::{Event, Handler, HandlerInit};

#[derive(Debug)]
pub enum Weapon {
    Knife,
    Rifle,
    Handgun,
    Bomb,
}
pub struct HitEvent {
    pub player_id: u64,
    pub enemy_id: u64,
    pub weapon: Weapon,
    pub distance: i32,
}
impl Event for HitEvent {}
pub struct HitHandler;
impl HandlerInit for HitHandler {
    fn init() -> Self
    where
        Self: Sized,
    {
        Self
    }
}
impl Handler for HitHandler {
    type Event = HitEvent;

    fn process(&mut self, message: &Self::Event) {
        println!(
            "Registered new hit: player #{} -> player #{}",
            message.player_id, message.enemy_id
        );
        println!("Weapon: {:?}", message.weapon);
        println!("Distance: {}", message.distance);
    }
}
