#![allow(unused)]
use bevy::{prelude::*, window::PresentMode, input::mouse::MouseButtonInput};
use bevy::input::ElementState;
use debug::DebugPlugin;
use board::Board;
use components::{
 TileTriggerEvent, Coordinates, TurnMovement
}; 
mod components;
mod board;
mod debug;

// Constants
const X_SPRITE :&str= "x-png.png";
const O_SPRITE :&str= "o-png.png";
const FONT :&str= "COMICATE.TTF";
const VALUE_X  :char= 'x';
const VALUE_O  :char= 'o';
const VALUE_E  :char= 'e';//Empty
const START_STATE    :&str= "START";
const END_STATE    :&str= "END";
// Resrces
struct GameTextures{
    x : Handle<Image>,
    o : Handle<Image>,
    font : Handle<Font>
}
struct BoardPlaces{
    pub b:  [[char;3];3],
    pub player : char,
    pub state  : String
 }

fn main() {
    App::new()
        .insert_resource(WindowDescriptor{
            title : "tic tac toe".to_string(),
            width : 900.,
            height: 900.,
            resizable : false,
            present_mode : PresentMode::Fifo,
            ..Default::default()
        })
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(Board)
        .add_plugin(DebugPlugin)
        .add_system(handle_mouse_clicks)
        .add_event::<TileTriggerEvent>()
        .add_event::<TurnMovement>()
        .run();
}
fn handle_mouse_clicks(
    mouse_input: Res<Input<MouseButton>>, 
    windows: Res<Windows>,
    mut tile_trigger_ewr: EventWriter<TileTriggerEvent>,
) {
    let win = windows.get_primary().expect("no primary window");
    if mouse_input.just_pressed(MouseButton::Left) {
        let coordinates = Coordinates{x : win.cursor_position().unwrap().x  as u16,
                                                   y : win.cursor_position().unwrap().y  as u16};
            if win.cursor_position().unwrap().x >= 300.0 &&  win.cursor_position().unwrap().x <= 600.{
                if win.cursor_position().unwrap().y >= 300.0 &&  win.cursor_position().unwrap().y <= 600.{
                    println!("inside");
                    tile_trigger_ewr.send(TileTriggerEvent(coordinates));
                }
            }
    }
}
fn setup(mut commands: Commands,win: Res<Windows>, asset_server: Res<AssetServer>) {
    // TRICK ORIGIN
    let w = win.get_primary().unwrap();
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform.translation.x += w.width() / 2.;
    camera.transform.translation.y += w.height() / 2.;
    // RESURCES
    commands.spawn_bundle(camera);
    let game_textures = GameTextures{
        x : asset_server.load(X_SPRITE),
        o : asset_server.load(O_SPRITE),
        font : asset_server.load(FONT),
    };
    commands.insert_resource(game_textures);
    // FOR LOGIC
    let board_movements = BoardPlaces{ b : [
        [VALUE_E,VALUE_E,VALUE_E],
        [VALUE_E,VALUE_E,VALUE_E], 
        [VALUE_E,VALUE_E,VALUE_E]],
        player : VALUE_X,
        state : START_STATE.to_string()
    };
    commands.insert_resource(board_movements)
}
