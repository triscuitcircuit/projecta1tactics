use bevy::asset::LoadState;
use bevy::{prelude::*, render::camera::Camera};
use bevy_tiled_prototype::TiledMapCenter;
use std::collections::HashMap;
use bevy_kira_audio::{Audio, AudioChannel, AudioPlugin, AudioSource};
use bevy_inspector_egui::InspectorPlugin;
use projecta1::backend::tactics_audio::*;
use projecta1::backend::interface::*;


fn main() {
    let mut app = App::build();
    app
        .add_resource(Msaa{samples: 4})
        .add_resource(WindowDescriptor{
            title: "ProjectA1".to_string(),
            ..Default::default()
        })
        .add_plugins_with(DefaultPlugins, |group| {
            group.disable::<bevy::audio::AudioPlugin>()
        })
        .init_resource::<ButtonMaterials>()
        //.add_plugin(InspectorPlugin::<Data>::new())
        .add_plugin(AudioPlugin)
        .add_startup_system(prepare_audio.system())
        //.add_startup_system(setup_sideview.system())
        .add_startup_system(setup.system())
        .add_system(check_audio_loading.system())
        .add_plugin(bevy_tiled_prototype::TiledMapPlugin)
        //.add_system(button_system.system())
        .add_system(camera_movement.system());
    app.run()
}


fn setup(commands: &mut Commands, asset_server: Res<AssetServer>, audio: Res<Audio>, mut materials: ResMut<Assets<ColorMaterial>>) {

     let music = asset_server.load("sounds/Battle1(Looped).wav");
     audio.play_looped(music);


    commands
        .spawn(bevy_tiled_prototype::TiledMapComponents {
            map_asset: asset_server.load("corrected.tmx"),
            center: TiledMapCenter(true),
            origin: Transform::from_scale(Vec3::new(1.6, 1.6, 1.0)),
            ..Default::default()
        })
        .spawn(Camera2dBundle::default())
        .with_children(|parent|{
            parent.spawn(CameraUiBundle::default());
            parent.spawn(NodeBundle{
               style: Style{
                   size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                   justify_content: JustifyContent::SpaceBetween,
                   ..Default::default()
               },
               material: materials.add(Color::NONE.into()),
                ..Default::default()
            })
                .with_children(|parent|{
                    parent
                        .spawn(NodeBundle{
                            style: Style{
                                size: Size::new(Val::Px(200.0),Val::Percent(100.0)),
                                border: Rect::all(Val::Px(2.0)),
                                ..Default::default()
                            },
                            material: materials.add(Color::rgb(0.65,0.65,0.65).into()),
                            ..Default::default()
                        });
                });
        });
}
fn camera_movement(
    commands: &mut Commands,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Camera, &mut Transform)>,
) {
    //let a= commands.lookup(CameraUiBundle);
    for (_, mut transform) in query.iter_mut() {
        let mut direction = Vec3::zero();
        let scale = transform.scale.x;

        if keyboard_input.pressed(KeyCode::A) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::S) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Z) || keyboard_input.pressed(KeyCode::Q){
            let scale = scale + 0.1;
            transform.scale = Vec3::new(scale, scale, scale);
        }

        if (keyboard_input.pressed(KeyCode::X)||keyboard_input.pressed(KeyCode::E)) && scale > 1.1 {
            let scale = scale - 0.1;
            transform.scale = Vec3::new(scale, scale, scale);
        }
        transform.translation += time.delta_seconds() * direction * 1000.;
    }
}
