use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Enemy)]
pub fn monster_ai(ecs: &mut SubWorld, #[resource] map: &Map)
{
    let mut mobs = <&mut Point>::query().filter(component::<Enemy>());
    mobs.iter_mut(ecs).for_each(|pos| {
        println!("mobs {:?}", pos);
        let mut rng = RandomNumberGenerator::new();

        let x = rng.range(-3, 3);
        // let y = rng.range(1, 3);
        let delta = Point::new(x, 0);

        let destination = *pos + delta;

        if map.can_enter_tile(destination) {
            *pos = destination;
        }
    });
}
