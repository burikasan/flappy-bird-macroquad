use macroquad::{prelude::*, rand::{gen_range}};


pub struct Pipe{
    pub  texture: Texture2D,
    pub  pos : Vec2,
    pub  flip_y : bool,
    
}
#[derive(Clone, Debug)] 
pub struct Area {
    pub  x : f32,
    pub  y : f32,
    pub  w : f32,
    pub  h : f32,
    pub  is_entred : bool
}

pub fn spaw_pipes(texture:&Texture2D,pipes:&mut Vec<Pipe>, score_rects:&mut Vec<Area>) {
    let rand_n = gen_range(-350, -10); 
    let pipeup = Pipe {
    texture: texture.clone(),
    pos : Vec2 { x: 500.0, y:rand_n as f32 },
    flip_y : true
    };
    let pipedown = Pipe {
    texture: texture.clone(),
    pos : Vec2 { x: 500.0, y:rand_n as f32  +pipeup.texture.height() *1.5 + gen_range(100, 190)as f32},
    flip_y : false
    };

    let score = Area{
        x : 500.0  ,
        y : pipeup.pos.y  + pipeup.texture.height() * 1.5  ,
        w :  pipeup.texture.width() * 1.5  ,
        h :  pipedown.pos.y   - (pipeup.pos.y + pipeup.texture.height() * 1.5 ),
        is_entred : false 

    };
    score_rects.push(score);

    pipes.push(pipeup);  
    pipes.push(pipedown);
      
}     
      
pub fn pipes_gen(pipes:&mut  Vec<Pipe>,score_rects:&mut Vec<Area>){
    for (_i , pipe ) in pipes.iter_mut().enumerate() {
        draw_texture_ex(&pipe.texture, pipe.pos.x, pipe.pos.y, WHITE, DrawTextureParams { dest_size:Some( Vec2{ x:pipe.texture.width()* 1.5,y :pipe.texture.height() * 1.5}),  flip_y: pipe.flip_y , ..Default::default()}); 
        pipe.pos.x -= 200.0 * get_frame_time();
    }
    for area in score_rects.iter_mut() {
        area.x -= 200.0 * get_frame_time();

    }
 }
      
