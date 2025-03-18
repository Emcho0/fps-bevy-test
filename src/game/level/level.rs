use bevy::prelude::*;

use bevy_rapier3d::prelude::*;

pub struct LevelPlugin;
impl Plugin for LevelPlugin{
    fn build(&self, app: &mut App){
        app.add_systemss(schedule:Startup,systems:init_level);

    }
}
fn init_level{
    mut commands : Commands,
    mut meshes : ResMut<Assets<Mesh>>,
    mut materials : ResMut<Assets<StandardMaterial>>,


}{
    let level_material: Handle<StandardMaterial> = materials.add(asset: StandardMaterial{

        base_color : Color::WHITE,
        ..default()

    });
    commands.spawn(bundle:(
        Collider::cuboid(hx: 1000.,hy: 0.,hz: 1000.),
        PbrBundle{
            material : level_material clone(),
            transform : Transform::IDENTITY,
            mesh : meshes.add(Plane3d::new(Vec3::Y,Vec2::splat(10000)))
            ..default()
        }
    ));
    commands.spawn(bundle:(
        Collider::cuboid(hx: 30.,hy: 30.,hz: 30.),
        PbrBundle{
            material : level_material clone(),
            transform : Transform::from_xyz(x: 0.,y:0.,z:-100.),
            mesh : meshes.add(asset:Cuboid::from_length(60.)),
            ..default()
        }
    ));


    
}