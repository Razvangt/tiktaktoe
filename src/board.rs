use bevy::{prelude::*, math::vec3, ecs::query};
use  crate::{
    GameTextures, components::{BoardGame,  TileTriggerEvent, Coordinates, TurnMovement}, BoardPlaces, VALUE_X, VALUE_E, VALUE_O, END_STATE
};
const SIZE: f32 = 100.0;
pub struct Board;
impl Plugin for Board{
    fn build(&self,app : &mut App){
        app
            .add_startup_system(spawn_board)
            .add_system(trigger_event_handler)
            .add_system(end_turn);
        }
}

fn spawn_board(mut commands : Commands){
    let mut board = Vec::new();
    let mut lines = Vec::new();
    let mut tiles = Vec::new();    

    for i in 0..= 3 {
        let line = commands.spawn_bundle(add_line(310.0,10.0, 300.0  ,300.0 + (SIZE * i as f32 ))).id();
        lines.push(line);
     }

    for i in 0..= 3 {
        let line = commands.spawn_bundle(add_line(10.0,310.0, 300.0 + (SIZE * i as f32) ,300.0)).id();
        lines.push(line);
     }
    // Create Outlines
    let  outlines =     commands.spawn()
                                        .insert(Name::new("outlines"))
                                        .insert(Transform::default())
                                        .insert(GlobalTransform::default())
                                        .push_children(&lines).id();
    board.push(outlines);

    //  Placeholder
    
    for j in 0..3 {
        for i in 0..3{
            let pos_x = 300. + (SIZE * i as f32) + SIZE/2.0 ;
            let pos_y = 300. + (SIZE * j as f32) + SIZE/2.0;
            let tile =  commands.spawn_bundle(
                SpriteBundle{
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(SIZE,SIZE)),
                        ..default()
                    },
                    transform : Transform {
                                 translation: Vec3::new(pos_x, pos_y, 0.0),
                                 ..default()
                             },
                    ..default()
                          }    
            ).id();
            tiles.push(tile)
        }
    }
    // Create Outlines
    let  placeholder =     commands.spawn()
                                        .insert(Name::new("placeholder"))
                                        .insert(Transform::default())
                                        .insert(GlobalTransform::default())
                                        .push_children(&tiles).id();
    board.push(placeholder);    
    //  Board 
    commands.spawn().insert(BoardGame)
                    .insert(Transform::default())
                    .insert(GlobalTransform::default())
                    .push_children(&board);
}


fn add_line(width: f32,hight: f32,x:f32,y:f32) -> SpriteBundle{
    SpriteBundle{
        sprite: Sprite {
            color: Color::rgb(0.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(width, hight)),
            ..default()
        },
        transform : Transform {
                     translation: Vec3::new(x + (width /2.), y + (hight /2.), 0.5),
                     ..default()
                 },
        ..default()
              }  
}

fn trigger_event_handler(
    mut commands : Commands,
    game_textures :  Res<GameTextures>,
    mut tile_trigger_evr: EventReader<TileTriggerEvent>,
    mut places : ResMut<BoardPlaces>,
    mut evnt_turn : EventWriter<TurnMovement>,
){  
    
    if places.state != END_STATE {
    for trigger_event in tile_trigger_evr.iter(){
        let values  = (trigger_event.0.x,trigger_event.0.y);
        let mut pos = (0,0);
            match values {
                (300..=400,300..=400)=> pos = (0,0),//Bottom left
                (401..=500,300..=400)=> pos = (0,1),//center 
                (501..=600,300..=400)=> pos = (0,2),//right
                (300..=400,401..=500)=> pos = (1,0),//Center left
                (401..=500,401..=500)=> pos = (1,1),//center 
                (501..=600,401..=500)=> pos = (1,2),//right
                (300..=400,501..=600)=> pos = (2,0),//Top left
                (401..=500,501..=600)=> pos = (2,1),//center
                (501..=600,501..=600)=> pos = (2,2),//right
                _ => println!("no Match")   
        };
        if places.b[pos.0][pos.1] == VALUE_E {
            places.b[pos.0][pos.1] = places.player;
         
        let textures_res = if places.player == VALUE_X {
            game_textures.x.clone()
        }else{
            game_textures.o.clone()
            };
        commands.spawn_bundle(SpriteBundle {
                texture: textures_res,
                transform: Transform{
                    scale: vec3(0.1, 0.1, 0.1),
                    translation : vec3(355.0 + (100.0 * pos.1 as f32),355.0 + (100.0 * pos.0 as f32),1.0),
                    ..default()           
                },
                ..default()
            }).insert(Name::new(places.player.to_string()));           
            // draw 
            println!("event at : {:?}",pos);
            evnt_turn.send(TurnMovement);
        }else{
            println!("Alredy Has value");   
        }
    }
    }
}


fn end_turn(
    mut evnt_red : EventReader<TurnMovement>,
    mut movements : ResMut<BoardPlaces> ,
    textures : Res<GameTextures>,
    mut commands : Commands,
){
    for event in evnt_red.iter(){
        if check_win(movements.b) {
            println!("{} Wins" , movements.player);
            movements.state = END_STATE.to_string();
            commands.spawn_bundle(UiCameraBundle::default());
            commands.spawn_bundle(TextBundle {
                // Use the `Text::with_section` constructor
                text: Text::with_section(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    format!("the winer is {}", movements.player),
                    TextStyle {
                        font: textures.font.clone(),
                        font_size: 100.0,
                        ..Default::default()
                    },
                    // Note: You can use `Default::default()` in place of the `TextAlignment`
                    TextAlignment {
                        ..default()
                    },
                ),
                ..default()
            });
        }else{
            if check_draw(movements.b) {
                println!("Draw");
                movements.state = END_STATE.to_string();
           
                commands.spawn_bundle(UiCameraBundle::default());
                commands.spawn_bundle(TextBundle {
                    // Use the `Text::with_section` constructor
                    text: Text::with_section(
                        // Accepts a `String` or any type that converts into a `String`, such as `&str`
                        format!("DRAW"),
                        TextStyle {
                            font: textures.font.clone(),
                            font_size: 100.0,
                            ..Default::default()
                        },
                        // Note: You can use `Default::default()` in place of the `TextAlignment`
                        TextAlignment {
                            ..default()
                        },
                    ),
                    ..default()
                }); }else{
                if movements.player == VALUE_X {
                    movements.player = VALUE_O;
                }else{
                    movements.player = VALUE_X;
                }
            }
        } 
    }
}

fn check_win(  arr : [[char;3];3]) -> bool{
    for i in 0..3{
        if arr[i][0] == arr[i][1] && arr[i][1] == arr[i][2] && arr[i][0] != VALUE_E {
            return true;
        }
    }
    for i in 0..3{
        if arr[0][i] == arr[1][i] && arr[1][i]  == arr[2][i]&& arr[0][i] != VALUE_E{
            return true;
        }
    }
    if arr[0][0] == arr[1][1] && arr[1][1] == arr[2][2] && arr[1][1] != VALUE_E{
        return true;
    }

    if arr[0][2] == arr[1][1] && arr[1][1] == arr[2][0] && arr[1][1] != VALUE_E{
        return true;
    }
    return false;
}
fn check_draw( arr : [[char;3];3]) -> bool{
    for i in  arr{
        for  j in i{
            if j == VALUE_E {
                return false;
            }
        }
    }
    return true;
}