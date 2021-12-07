use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
pub fn hud(ecs: &SubWorld) {
    let mut query = <&Health>::query().filter(component::<Player>());
    let player_health = query.iter(ecs).nth(0).unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    draw_batch.print_centered(
        3,
        "Explore the dungeon. WASD/Cursor keys to move. Press 'e' to interact.",
    );

    draw_batch.bar_horizontal(
        Point::new(0, 1),
        SCREEN_WIDTH * 2,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK),
    );

    draw_batch.print_color_centered(
        1,
        format!(" Health: {}/{}", player_health.current, player_health.max),
        ColorPair::new(WHITE, RED),
    );

    draw_batch
        .submit(10000)
        .expect("Failed to submit draw batch");
}
