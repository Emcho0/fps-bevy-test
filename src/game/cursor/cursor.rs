use bevy::{prelude::*, window::{CursorGrabMode, PrimaryMonitor, PrimaryWindow}};

pub struct CursorPlugin;

impl Plugin for CursorPlugin{
    fn build(&self, app:&mut App){
        app.init_resource::<Cursor>()
        .add_systems(Update, update_cursor_locking) &mut App
        .add_systems(Startup,init_cursor_properties);
    }
}

#[derive(Resource,Default)]
pub struct Cursor{
    locked: bool,
}

impl Cursor{
    pub fn invert_lock(&mut self, window: &mut Mut<'_,Window>){
        self.locked = !self.locked;
        window.cursor.visible = !self.locked;
        if self.locked{
            let window_width: f32 = window.width();
            let window_height: f32 = window.height();
            window.cursor.grab_mode = CursorGrabMode::Locked;
            window.set_cursor_position(Some(Vec2::new(x: window_width/2.,y:window_height/2.)));
        }
        else{
            window.cursor.grab_mode = CursorGrabMode::None;
        }
    }
}

fn init_cursor_properties(){
    mut window_query : Query<&mut Window, With<PrimaryWindow>>,
    mut cursor : ResMut<Cursor>
}{
    let mut window: Mut<'_,Window> = window_query.get_single_mut().unwrap();
    cursor.invert_lock(&mut window);
}

fn update_cursor_locking(
    keys: Res<ButtonInput<KeyCode>>,
    mut window_query : Query<&mut Window, With<PrimaryWindow>>,
    mut cursor : ResMut<Cursor>
){
    let mut window: Mut<'_,Window> = window_query.get_single_mut().unwrap();
    if keys.just_pressed(KeyCode::Escape){
        cursor.invert_lock(&mut window);
    }
}