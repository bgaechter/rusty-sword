use crate::prelude::*;

use self::template::Templates;

mod template;

pub fn spawn_level(ecs: &mut World, resources: &mut Resources, rng: &mut RandomNumberGenerator, level:usize, spawn_points: &[Point]) {
    let template = Templates::load();

    template.spawn_entities(ecs, resources, rng,level,spawn_points);
}

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player{map_level:0},
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: 7,
            max: 10,
        },
        FieldOfView::new(8),
    ));
}

pub fn spawn_amulet_of_greta(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        AmuletOfGreta,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('/'),
        },
        Name {
            name: "Amulet of Greta".to_string(),
        },
    ));
}
