use specs::Join;
use specs::ReadExpect;
use specs::System;
use specs::WriteStorage;

use crate::components::{Direction, Point, TrainHead, TrainParameters, TrainParts};
use crate::map::Map;

const AIR_RESISTANCE: f32 = 1.0;

pub struct MovingTrain {}

impl<'a> System<'a> for MovingTrain {
    type SystemData = (
        WriteStorage<'a, TrainHead>,
        WriteStorage<'a, TrainParts>,
        WriteStorage<'a, TrainParameters>,
        ReadExpect<'a, Map>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut heads, mut parts, mut parameters, map) = data;

        for (mut head, mut parts, mut params) in (&mut heads, &mut parts, &mut parameters).join() {
            params.acceleration =
                (params.force - (AIR_RESISTANCE * params.velocity.powi(2))) / params.mass;
            params.velocity += params.acceleration;

            // Move train according to velocity
            let cells_travelled = params.velocity as i32;

            for _ in 0..cells_travelled {
                let d = move_train(
                    &mut head,
                    &mut parts,
                    cells_travelled,
                    &map,
                    params.movement_direction,
                );
                params.movement_direction = d;
            }
        }
    }
}

fn move_train(
    head: &mut TrainHead,
    parts: &mut TrainParts,
    n: i32,
    map: &Map,
    d: Direction,
) -> Direction {
    let dir = map.get_next_direction(head.position, d);
    let delta_pos = Point::from(dir);
    head.position += delta_pos;
    parts.parts.remove(parts.parts.len() - 1);
    parts.parts.insert(0, parts.parts[0]);
    parts.parts[0] += delta_pos;

    dir
}
