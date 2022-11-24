use crate::event_system::{Job, JobInit, Message};

#[derive(Debug)]
pub enum Weapon {
    Knife,
    Rifle,
    Handgun,
    Bomb,
}
pub struct HitMessage {
    pub player_id: u64,
    pub enemy_id: u64,
    pub weapon: Weapon,
    pub distance: i32,
}
impl Message for HitMessage {}
pub struct HitJob;
impl JobInit for HitJob {
    fn init() -> Self
    where
        Self: Sized,
    {
        Self
    }
}
impl Job for HitJob {
    type ItemMessage = HitMessage;

    fn process(&mut self, message: &Self::ItemMessage) {
        println!(
            "Registered new hit: player #{} -> player #{}",
            message.player_id, message.enemy_id
        );
        println!("Weapon: {:?}", message.weapon);
        println!("Distance: {}", message.distance);
    }
}
