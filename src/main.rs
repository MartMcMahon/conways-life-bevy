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

fn main() {
    App::build()
        //we initial windows size here:
        .insert_resource(WindowDescriptor {
            title: "Conway's Dead or Alive Xtreme Volleyball".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .add_startup_stage("grid", SystemStage::single(spawn_grid.system()))
        // .add_system_set(
        //     SystemSet::new()
        //         .with_run_criteria(FixedTimestep::step(1.0))
        //         .with_system(iteration.system()),
        // )
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let white_mat = materials.add(Color::rgb(0.9, 0.9, 0.9).into());
    let black_mat = materials.add(Color::rgb(0.1, 0.1, 0.1).into());

    commands.insert_resource(Materials {
        square: materials.add(Color::rgb(0.9, 0.9, 0.9).into()),
        white_material: materials.add(Color::rgb(0.9, 0.9, 0.9).into()),
        black_material: materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
        grey_material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
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

struct Materials {
    square: Handle<ColorMaterial>,
    white_material: Handle<ColorMaterial>,
    black_material: Handle<ColorMaterial>,
    grey_material: Handle<ColorMaterial>,
}
enum Cellstate {
    Alive,
    Dead,
}
struct Cell {
    x: f32,
    y: f32,
    state: Cellstate,
    // neighbors are index of cell
    // north: Option<*const Cell>,
    // east: Option<*const Cell>,
    // south: Option<*const Cell>,
    // west: Option<*const Cell>,
}
impl Cell {
    fn count_live_neighbors(&self) -> i32 {
        let count = 0;
        // n

        // s
        // e
        // w
        // return total
        count
    }

    fn iterate(&mut self) -> Cellstate {
        match self.count_live_neighbors() {
            3 => Cellstate::Alive,
            _ => Cellstate::Dead,
        }
    }
}

fn spawn_grid(mut commands: Commands, materials: Res<Materials>) {
    let mut cells: Vec<Cell> = Vec::new();

    let mut temp = vec![
        Cell {
            x: 0.0,
            y: 0.0,
            state: Cellstate::Alive,
            // north: None,
            // south: None,
            // east: None,
            // west: None,
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

    let mut rng = rand::thread_rng();
    let b: bool = rng.gen();

    for y in 0..10 {
        let y = y as f32;
        for x in 0..10 {
            let x = x as f32;
            if x < 3.0 && y < 3.0 {
                cells.push(temp.remove(0));
            } else {
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

    // for y in 0..10 {
    //     for x in 0..10 {
    //         cells[y * WIDTH + x].north = *cells[(y - 1) * WIDTH + x];
    //     }
    // }

    // let mut neighbor_refs: Vec<&Cell> = Vec::new();

    for y in 0..10 {
        let y = y as f32;
        for x in 0..10 {
            let x = x as f32;
            let index = y * WIDTH as f32 + x;

            commands
                .spawn()
                .insert(cells.remove(0))
                .insert_bundle(SpriteBundle {
                    material: materials.square.clone(),
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
) {
    for (mut cell, mut mat) in cells.iter_mut() {
        // check neighbors
        // cells.iter_mut().map(|(mut neighbor_cell, _)| {
        //     match neighbor_cell.y {
        //         cell.y + 1.0 => {
        //             // north neighbor
        //         },
        //         cell.y - 1.0 => {
        //             // sourth
        //         }
        //     };
        //     match neighbor_cell.x {
        //         cell.x -1.0 => {
        //             // west
        //         },
        //         cell.x +1.0 => {
        //             // east
        //         }
        //     }
        // });

        match &cell.count_live_neighbors() {
            3 => {
                cell.state = Cellstate::Alive;
                *mat = materials.white_material.clone();
            }
            _ => {
                cell.state = Cellstate::Dead;
                *mat = materials.grey_material.clone();
            }
        }
        // match &cell.state {
        // Cellstate::Alive => {
        //     match cell.count_live_neighbors(&cell) {
        // _ => {}
        // }

        // fewer than two live neighbors, die
        // 2-3 live neighbors, live on
        // more than 3, die
        //
        // cell.state = Cellstate::Dead;
        // }
        // Cellstate::Dead => {
        // // ==3 live neighbors, become alive
        // // cell.state = Cellstate::Alive;
        // }
        // }
    }
}

fn square_spawner(mut commands: Commands, materials: Res<Materials>) {
    commands.spawn_bundle(SpriteBundle {
        material: materials.square.clone(),
        ..Default::default()
    });
}

// fn grid_system(mut lines: ResMut<DebugLines>) {
//     let thickness = 1.0;
//     for x in (-1000..1000).step_by(100) {
//         let x = x as f32;
//         let start = Vec3::new(x, -1000.0, 1.1);
//         let end = Vec3::new(x, 1000.0, 1.1);
//         lines.line(start, end, thickness);
//     }
//     for y in (-1000..1000).step_by(100) {
//         let y = y as f32;
//         let start = Vec3::new(-1000.0, y, 1.1);
//         let end = Vec3::new(1000.0, y, 1.1);
//         // lines.line(start, end, thickness);
//     }
// }

// fn draw_square(
//     commands: Commands,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     mut quads: ResMut<Assets<shape::Quad>>,
// ) {
//     commands.spawn_bundle(PbrBundle {
//         mesh: quads.add(shape::Quad::new(34)),
//         material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
//         transform: Transform::from_translation(-1.0, 0.0, 1.0),
//         ..Default::default()
//     });
// }

// fn create_board(
//     commands: &mut Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     let mesh = meshes.add(Mesh::from(shape::Plane { size: 1. }));
//     let white_material = materials.add(Color::rgb(1., 0.9, 0.9).into());
//     let black_material = materials.add(Color::rgb(0., 0.1, 0.1).into());

//     for i in 0..10 {
//         commands.spawn_bundle(PbrBundle {
//             mesh: mesh.clone(),
//             material: white_material.clone(),
//             transform: Transform::from_translation(Vec3::new(i as f32, 0., i as f32)),
//             ..Default::default()
//         });
//     }
// }
