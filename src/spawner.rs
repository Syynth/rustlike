use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            max: 50,
            current: 20,
        },
    ));
}

pub fn spawn_amulet_of_yala(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        AmuletOfYala,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('|'),
        },
        Name("Amulet of Yala".to_string()),
    ));
}

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let (hp, name, glyph) = match rng.roll_dice(1, 20) {
        1..=14 => goblin(),
        15..=17 => orc(),
        18..=19 => ogre(),
        _ => ettin(),
    };
    ecs.push((
        Enemy,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph,
        },
        ChasingPlayer,
        Health {
            current: hp,
            max: hp,
        },
        Name(name),
    ));
}

fn goblin() -> (i32, String, FontCharType) {
    (1, "Goblin".to_string(), to_cp437('g'))
}

fn orc() -> (i32, String, FontCharType) {
    (2, "Orc".to_string(), to_cp437('o'))
}

fn ogre() -> (i32, String, FontCharType) {
    (3, "Ogre".to_string(), to_cp437('O'))
}

fn ettin() -> (i32, String, FontCharType) {
    (4, "Ettin".to_string(), to_cp437('E'))
}
