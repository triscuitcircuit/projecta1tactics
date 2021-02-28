use bevy::prelude::*;
use bevy_inspector_egui::InspectorPlugin;
use crate::backend::tactics_audio::AudioState;


pub struct ButtonMaterials{
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>
}

impl FromResources for ButtonMaterials{
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials{
            normal: materials.add(Color::rgb(0.15,0.15,0.15).into()),
            hovered: materials.add(Color::rgb(0.25,0.25,0.25).into()),
            pressed: materials.add(Color::rgb(0.35,0.75,0.35).into()),
        }
    }
}
pub fn button_system(
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &Children),
        (Mutated<Interaction>, With<Button>)
    >,
    mut text_query: Query<&mut Text>,
){
    for (interaction, mut material, children) in interaction_query.iter_mut(){
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction{
            Interaction::Clicked => {
                text.value = "Clicked".to_string();
                *material = button_materials.pressed.clone();
            }
            Interaction::Hovered => {
                text.value = "hovered".to_string();
                *material = button_materials.pressed.clone();
            }
            Interaction::None => {
                text.value = "None".to_string();
                *material = button_materials.pressed.clone();
            }
        }
    }
}
pub fn setup_buttons(commands: &mut Commands,
                     asset_server: Res<AssetServer>,
                     button_materials: Res<ButtonMaterials>,
) {
    commands
        // ui camera
        .spawn(CameraUiBundle::default())
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: button_materials.normal.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    value: "Button".to_string(),
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    style: TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..Default::default()
                    },
                },
                ..Default::default()
            });
        });
}