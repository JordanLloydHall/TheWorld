use heron::prelude::*;

#[derive(PhysicsLayer)]
pub enum Layer {
    Territory,
    Human,
    World,
}

pub struct AddedObject(pub bool);
