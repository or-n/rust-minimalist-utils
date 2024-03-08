use ecs::entities::Entity;
use ecs::*;

use derived_deref::{Deref, DerefMut};

#[derive(Clone, Deref, DerefMut)]
pub struct Speed(f32);

#[derive(Clone, Deref, DerefMut, Debug)]
pub struct Position(f32);

#[derive(Debug)]
enum Error {
    Speed,
    Position,
}

fn motion(ecs: &mut ECS, entity: Entity) -> Result<Position, Error> {
    let speed = ecs
        .get_component::<Speed>(entity)
        .ok_or(Error::Speed)?
        .clone();
    let position = ecs
        .get_mut_component::<Position>(entity)
        .ok_or(Error::Position)?;
    **position += *speed;
    Ok(position.clone())
}

fn main() {
    let mut ecs = ECS::new();
    ecs.register_component::<Position>();
    ecs.register_component::<Speed>();
    let entity = ecs.spawn();
    ecs.insert_component(entity, Position(0.));
    ecs.register_component::<Position>();
    ecs.insert_component(entity, Speed(1.));
    println!("{:?}", motion(&mut ecs, entity));
    println!("{:?}", motion(&mut ecs, entity));
    ecs.despawn(entity);
}
