use crate::components::{Direction, Point, TrafficLight, TrainParts};
use crate::map::Map;
use specs::{Entity, Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

pub struct ActiveTrafficLight {}

impl<'a> System<'a> for ActiveTrafficLight {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Point>,
        ReadStorage<'a, Direction>,
        ReadStorage<'a, TrainParts>,
        WriteStorage<'a, TrafficLight>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, tl_positions, tl_direction, trains, mut lights) = data;

        let mut tl_points: Vec<Point> = Vec::new();
        let mut train_positions: Vec<Point> = Vec::new();

        tl_positions
            .join()
            .for_each(|point: &Point| tl_points.push(*point));

        trains.join().for_each(|t| {
            t.parts.iter().for_each(|p| {
                train_positions.push(*p);
            })
        });

        for (tl_p, tl_d, light) in (&tl_positions, &tl_direction, &mut lights).join() {
            //  Generate the responsibility zone for light

            let mut responsibility_zone: Vec<Point> = Vec::new();
            let mut dir = *tl_d;
            let mut cur_pos = *tl_p + Point::from(dir);

            while !tl_points.contains(&cur_pos) {
                responsibility_zone.push(cur_pos);
                dir = map.get_next_direction(cur_pos, dir);
                cur_pos += Point::from(dir);
            }

            // If  train in responsibility zone set light red
            // Else set light green
            light.is_green = !responsibility_zone
                .iter()
                .any(|p| train_positions.contains(p));

            responsibility_zone.iter().for_each(|p| {
                let i = map.xy_idx(*p);
                map.occupied_tiles[i] = !light.is_green
            })
        }
    }
}
