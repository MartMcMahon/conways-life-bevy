use bevy::core::FixedTimestep;
use bevy::prelude::*;
use rand::Rng;

const WIDTH: u32 = 10;
const HEIGHT: u32 = 10;
const WINDOW_WIDTH: f32 = 1000.0;
const WINDOW_HEIGHT: f32 = 1000.0;
const WORLD_SIZE: f32 = 800.0;
const MARGIN: f32 = (WINDOW_WIDTH - WORLD_SIZE) / 2.0;
const SQUARE_SIZE: f32 = WORLD_SIZE / WIDTH as f32;

struct State {
    playing: bool,
}
struct Materials {
    square: Handle<ColorMaterial>,
    white_material: Handle<ColorMaterial>,
    black_material: Handle<ColorMaterial>,
    grey_material: Handle<ColorMaterial>,
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}
#[derive(Copy, Clone, PartialEq)]
enum Cellstate {
    Alive,
    Dead,
}
#[derive(Copy, Clone)]
struct Cell {
    x: f32,
    y: f32,
    state: Cellstate,
}

#[derive(Debug)]
struct Neighbors {
    nw: Option<i32>,
    n: Option<i32>,
    ne: Option<i32>,
    e: Option<i32>,
    se: Option<i32>,
    s: Option<i32>,
    sw: Option<i32>,
    w: Option<i32>,
}
impl Neighbors {
    fn new(index: usize, width: i32) -> Neighbors {
        let idx = index as i32;
        Neighbors {
            nw: Some(idx - width - 1),
            n: Some(idx - width),
            ne: Some(idx - width + 1),
            e: Some(idx + 1),
            se: Some(idx + width + 1),
            s: Some(idx + width),
            sw: Some(idx + width - 1),
            w: Some(idx - 1),
        }
    }
}

/// This example illustrates how to create a button that changes color and text based on its
/// interaction state.
fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Conway's Dead or Alive Xtreme Volleyball".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..Default::default()
        })
        .insert_resource(State { playing: true })
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(iteration.system()),
        )
        .add_plugins(DefaultPlugins)
        .init_resource::<ButtonMaterials>()
        .add_startup_system(setup.system())
        .add_startup_stage("grid", SystemStage::single(spawn_grid.system()))
        .add_system(button_system.system())
        .add_system(input_system.system())
        .run();
}

struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
        }
    }
}

fn button_system(
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut material, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Press".to_string();
                *material = button_materials.pressed.clone();
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *material = button_materials.normal.clone();
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<ButtonMaterials>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(65.0), Val::Px(65.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: button_materials.normal.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Button",
                    TextStyle {
                        font: asset_server.load("fonts/helvetica.ttf"),
                        font_size: 30.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        });

    let white_mat = materials.add(Color::rgb(0.9, 0.9, 0.9).into());
    let black_mat = materials.add(Color::rgb(0.1, 0.1, 0.1).into());

    commands.insert_resource(Materials {
        square: materials.add(Color::rgb(0.9, 0.9, 0.9).into()),
        white_material: materials.add(Color::rgb(0.9, 0.9, 0.9).into()),
        black_material: materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
        grey_material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
        normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
        hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
        pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
    });
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    for x in 0..WIDTH {
        commands.spawn_bundle(SpriteBundle {
            material: black_mat.clone(),
            transform: Transform::from_xyz(
                x as f32 * SQUARE_SIZE - WINDOW_WIDTH / 2.0 + MARGIN,
                0.0,
                0.0,
            ),
            sprite: Sprite::new(Vec2::new(1.0, WORLD_SIZE)),
            ..Default::default()
        });
    }

    for y in 0..HEIGHT {
        commands.spawn_bundle(SpriteBundle {
            material: black_mat.clone(),
            transform: Transform::from_xyz(
                0.0,
                y as f32 * SQUARE_SIZE - WINDOW_WIDTH / 2.0 + MARGIN,
                0.0,
            ),
            sprite: Sprite::new(Vec2::new(WORLD_SIZE, 1.0)),
            ..Default::default()
        });
    }
}

fn spawn_grid(mut commands: Commands, materials: Res<Materials>) {
    let mut cells: Vec<Cell> = Vec::new();

    let mut temp = vec![
        Cell {
            x: 0.0,
            y: 0.0,
            state: Cellstate::Alive,
        },
        Cell {
            x: 1.0,
            y: 0.0,
            state: Cellstate::Dead,
        },
        Cell {
            x: 2.0,
            y: 0.0,
            state: Cellstate::Alive,
        },
        Cell {
            x: 0.0,
            y: 1.0,
            state: Cellstate::Dead,
        },
        Cell {
            x: 1.0,
            y: 1.0,
            state: Cellstate::Alive,
        },
        Cell {
            x: 2.0,
            y: 1.0,
            state: Cellstate::Alive,
        },
        Cell {
            x: 0.0,
            y: 2.0,
            state: Cellstate::Dead,
        },
        Cell {
            x: 1.0,
            y: 2.0,
            state: Cellstate::Alive,
        },
        Cell {
            x: 2.0,
            y: 2.0,
            state: Cellstate::Dead,
        },
    ];

    let mut temp2 = vec![
        Cell {
            x: 0.0,
            y: 0.0,
            state: Cellstate::Dead,
        },
        Cell {
            x: 1.0,
            y: 0.0,
            state: Cellstate::Alive,
        },
        Cell {
            x: 2.0,
            y: 0.0,
            state: Cellstate::Dead,
        },
        Cell {
            x: 0.0,
            y: 1.0,
            state: Cellstate::Dead,
        },
        Cell {
            x: 1.0,
            y: 1.0,
            state: Cellstate::Alive,
        },
        Cell {
            x: 2.0,
            y: 1.0,
            state: Cellstate::Dead,
        },
        Cell {
            x: 0.0,
            y: 2.0,
            state: Cellstate::Dead,
        },
        Cell {
            x: 1.0,
            y: 2.0,
            state: Cellstate::Alive,
        },
        Cell {
            x: 2.0,
            y: 2.0,
            state: Cellstate::Dead,
        },
    ];

    let mut rng = rand::thread_rng();

    for y in 0..10 {
        let y = y as f32;
        for x in 0..10 {
            let x = x as f32;
            if x < 3.0 && y < 3.0 {
                cells.push(temp.remove(0));
            } else {
                // let b: bool = rng.gen();
                let b: bool = false;
                cells.push(Cell {
                    x: x as f32,
                    y: y as f32,
                    state: match b {
                        true => Cellstate::Alive,
                        false => Cellstate::Dead,
                    },
                });
            }
        }
    }

    for y in 0..10 {
        let y = y as f32;
        for x in 0..10 {
            let x = x as f32;
            let index = y * WIDTH as f32 + x;

            let mat = match &cells[0].state {
                Cellstate::Alive => materials.white_material.clone(),
                Cellstate::Dead => materials.grey_material.clone(),
            };

            commands
                .spawn()
                .insert(cells.remove(0))
                .insert_bundle(SpriteBundle {
                    material: mat,
                    sprite: Sprite::new(Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
                    ..Default::default()
                })
                .insert(Transform::from_xyz(
                    x * SQUARE_SIZE - WINDOW_WIDTH / 2.0 + SQUARE_SIZE / 2.0 + MARGIN,
                    y * SQUARE_SIZE - WINDOW_HEIGHT / 2.0 + SQUARE_SIZE / 2.0 + MARGIN,
                    0.0,
                ));
        }
    }
}

fn iteration(
    mut commands: Commands,
    mut cells: Query<(&mut Cell, &mut Handle<ColorMaterial>)>,
    materials: Res<Materials>,
    state: Res<State>,
) {
    if state.playing {
        let mut old_world: Vec<Cell> = Vec::new();
        let mut c: i32 = 0;

        for (mut cell, mut mat) in cells.iter_mut() {
            old_world.push(cell.clone());
        }
        let mut new_world = old_world.clone();

        println!("------------------------");
        for (i, cell) in old_world.iter().enumerate() {
            let live_neighbors = get_neighbors(&old_world, i);

            print!(" | {}", live_neighbors);

            match live_neighbors {
                2 => {
                    if cell.state == Cellstate::Alive {
                        new_world[i].state = Cellstate::Alive;
                    } else {
                        new_world[i].state = Cellstate::Dead;
                    }
                }

                3 => {
                    new_world[i].state = Cellstate::Alive;
                }
                _ => {
                    new_world[i].state = Cellstate::Dead;
                }
            }
        }
        println!("------------------------");

        for (i, (mut cell, mut mat)) in cells.iter_mut().enumerate() {
            match new_world[i].state {
                Cellstate::Alive => {
                    cell.state = Cellstate::Alive;
                    *mat = materials.white_material.clone();
                }
                Cellstate::Dead => {
                    cell.state = Cellstate::Dead;
                    *mat = materials.grey_material.clone();
                }
            }
        }
    }
}

fn get_neighbors(world: &Vec<Cell>, index: usize) -> u32 {
    let idx = index as u32;
    let mut neighbors = Neighbors::new(index, WIDTH as i32);

    if idx < WIDTH {
        neighbors.nw = None;
        neighbors.n = None;
        neighbors.ne = None;
    }
    if idx % WIDTH == WIDTH - 1 {
        neighbors.ne = None;
        neighbors.e = None;
        neighbors.se = None;
    }
    if idx >= (HEIGHT - 1) * WIDTH {
        neighbors.se = None;
        neighbors.s = None;
        neighbors.sw = None;
    }
    if idx % WIDTH == 0 {
        neighbors.sw = None;
        neighbors.w = None;
        neighbors.nw = None;
    }

    if idx == 0 {
        neighbors.nw = None;
        neighbors.n = None;
        neighbors.ne = None;
        neighbors.w = None;
        neighbors.sw = None;
    } else if idx == WIDTH - 1 {
        neighbors.nw = None;
        neighbors.n = None;
        neighbors.ne = None;
        neighbors.e = None;
        neighbors.se = None;
    } else if idx == WIDTH * HEIGHT - 1 {
        neighbors.ne = None;
        neighbors.e = None;
        neighbors.se = None;
        neighbors.s = None;
        neighbors.sw = None;
    } else if idx == WIDTH * HEIGHT - WIDTH {
        neighbors.se = None;
        neighbors.s = None;
        neighbors.sw = None;
        neighbors.w = None;
        neighbors.nw = None;
    }

    let mut count = 0;

    match neighbors.nw {
        Some(i) => match world[i as usize].state {
            Cellstate::Alive => {
                count += 1;
            }
            Cellstate::Dead => {}
        },
        None => {}
    }
    match neighbors.n {
        Some(i) => match world[i as usize].state {
            Cellstate::Alive => {
                count += 1;
            }
            Cellstate::Dead => {}
        },
        None => {}
    }
    match neighbors.ne {
        Some(i) => match world[i as usize].state {
            Cellstate::Alive => {
                count += 1;
            }
            Cellstate::Dead => {}
        },
        None => {}
    }
    match neighbors.e {
        Some(i) => match world[i as usize].state {
            Cellstate::Alive => {
                count += 1;
            }
            Cellstate::Dead => {}
        },
        None => {}
    }

    match neighbors.se {
        Some(i) => match world[i as usize].state {
            Cellstate::Alive => {
                count += 1;
            }
            Cellstate::Dead => {}
        },
        None => {}
    }

    match neighbors.s {
        Some(i) => match world[i as usize].state {
            Cellstate::Alive => {
                count += 1;
            }
            Cellstate::Dead => {}
        },
        None => {}
    }

    match neighbors.sw {
        Some(i) => match world[i as usize].state {
            Cellstate::Alive => {
                count += 1;
            }
            Cellstate::Dead => {}
        },
        None => {}
    }

    match neighbors.w {
        Some(i) => match world[i as usize].state {
            Cellstate::Alive => {
                count += 1;
            }
            Cellstate::Dead => {}
        },
        None => {}
    }

    count
}

fn input_system(
    keys: Res<Input<KeyCode>>,
    btns: Res<Input<MouseButton>>,
    mut state: ResMut<State>,
) {
    if keys.just_pressed(KeyCode::Space) {
        println!("space!");
        state.playing = !state.playing;
    }
}
