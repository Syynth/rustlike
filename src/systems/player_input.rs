use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
#[write_component(Health)]
#[read_component(Enemy)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
    if let Some(key) = *key {
        let delta = match key {
            VirtualKeyCode::W | VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::S | VirtualKeyCode::Down => Point::new(0, 1),
            VirtualKeyCode::A | VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::D | VirtualKeyCode::Right => Point::new(1, 0),
            _ => Point::new(0, 0),
        };

        let (player_entity, destination) = players
            .iter(ecs)
            .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
            .unwrap();

        let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
        if delta.x != 0 || delta.y != 0 {
            let mut hit_something = false;
            enemies
                .iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(entity, _)| {
                    hit_something = true;
                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: player_entity,
                            target: *entity,
                        },
                    ));
                });
            if !hit_something {
                commands.push((
                    (),
                    WantsToMove {
                        entity: player_entity,
                        destination,
                    },
                ));
            }

            *turn_state = TurnState::PlayerTurn;
        } else if key == VirtualKeyCode::Space {
            if let Ok(mut health) = ecs
                .entry_mut(player_entity)
                .unwrap()
                .get_component_mut::<Health>()
            {
                health.current = i32::min(health.max, health.current + 1);
            }
            *turn_state = TurnState::PlayerTurn;
        }
    }
}
