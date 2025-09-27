
use macroquad::prelude::*;

use crate::{collision, pipes::{Area, Pipe}};


pub struct Bird {
  pub  texture : Texture2D,
  pub  pos : Vec2,
  pub  vel : Vec2,
  pub  rot : f32,
  pub  is_alive : bool 
}

pub fn bird_movement(bird:&mut Bird, pipes: &mut Vec<Pipe> ,score_rects:&mut Vec<Area>,score:&mut i32, bs_rect : Rect){
    let gravity = 20.0;
    let   jump = -400.0;
    bird.vel.y +=  gravity * get_frame_time();
    if is_key_pressed(KeyCode::Space) && bird.is_alive {
        bird.vel.y = jump * get_frame_time();
    }
    bird.rot = lerp(bird.rot , bird.vel.y * 0.1, 0.1); 
    



    bird.pos += bird.vel;
    let dest = Rect{x:bird.pos.x,y:bird.pos.y , w:bird.texture.width() as f32 * 1.1, h:bird.texture.height() as f32 * 1.1 };
    for pipe in pipes.iter_mut() {
       let pdest = Rect{x:pipe.pos.x , y:pipe.pos.y,w:pipe.texture.width() * 1.5 ,  h: pipe.texture.height() * 1.5};
       if collision(dest, pdest) || collision(dest, bs_rect){
            bird.is_alive = false;
       }
    }
    if  bird.pos.y < -2.0  ||bird.pos.y > screen_height()- 110.0{
            bird.is_alive = false;
    }
    for area in score_rects.iter_mut() {
        if !area.is_entred && collision(dest, Rect{x:area.x,y:area.y,w:area.w,h:area.h}){
            *score += 1;
            area.is_entred = true;
        }
    }

    
}

pub fn bird_drawing(bird:&mut Bird){
    draw_texture_ex(&bird.texture, bird.pos.x, bird.pos.y , WHITE, DrawTextureParams { 
        dest_size: Some(Vec2 { x: (bird.texture.width() as f32 * 1.3),  y:( bird.texture.height() as f32  *1.3) }), 
        rotation: bird.rot ,..Default::default()});
   
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}
