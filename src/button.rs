use bevy::prelude::*;

pub struct ButtonPlugin;

const READY: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVER: Color = Color::rgb(0.25, 0.25, 0.25);
const CLICK: Color = Color::rgb(0.35, 0.75, 0.35);
const BACKGROUND: Color = Color::rgb(0.9, 0.9, 0.95);

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(button_system.label("menu"));
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();

        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "GO!!".to_string();
                *color = CLICK.into();
            }
            Interaction::Hovered => {
                text.sections[0].value = "READY?".to_string();
                *color = HOVER.into();
            }
            Interaction::None => {
                text.sections[0].value = "HERE".to_string();
                *color = READY.into();
            }
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: READY.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Button",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: BACKGROUND,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        });
}
