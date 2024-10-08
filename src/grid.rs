use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ShapePlugin);
        app.add_systems(Startup, spawn);
    }
}

#[derive(Component)]
struct Grid;

fn spawn(
    mut commands: Commands,
){
    let grid_size: i32 = 200;
    let cell_size: f32 = 10.0;
    let cell_distance: f32 = 1.0;
    
    let color: Color = Color::linear_rgb(42.0, 42.0, 42.0);
    let cell:shapes::RegularPolygon = shapes::RegularPolygon {
        sides: 4,
        feature: shapes::RegularPolygonFeature::Radius(cell_size / 2.0),
        ..shapes::RegularPolygon::default()
    };

    for x in 0..grid_size {
        for y in 0..grid_size{
            commands.spawn((
                Grid,
                ShapeBundle {
                    path: GeometryBuilder::build_as(&cell.clone()),
                    spatial: SpatialBundle {
                        transform: Transform::from_xyz((cell_size + cell_distance) * x as f32, (cell_size + cell_distance) * y as f32, 0.0),
                        ..default()
                    },
                    ..default()
                },
                Fill::color(color),
                Stroke::new(color, 0.0),
            ));
        }
    }
}