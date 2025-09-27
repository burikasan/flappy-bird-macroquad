use macroquad::prelude::*;
mod background;
use crate::{background::Background, bird::{bird_drawing, bird_movement, Bird}, pipes::{pipes_gen, spaw_pipes, Area, Pipe}};
mod bird;
mod pipes;
#[macroquad::main("Example Window")]
async fn main() {
    // Request a new screen size (e.g., 800x600 pixels)
    let  birdtexture = load_texture("flappy-bird-assets/sprites/yellowbird-midflap.png").await.unwrap();
    birdtexture.set_filter(FilterMode::Nearest); 
    macroquad::window::request_new_screen_size(450.0, 700.0);
    let mut bird:Bird = Bird {
    texture:birdtexture,
    pos:Vec2 { x: 100.0, y: 200.0 },
    vel:Vec2 { x: 0.0, y: 0.0 },
    rot: 0.0,
    is_alive : true
    };

    let mut score = 0; 
    let mut highest_score = 0; 


    let dpos = bird.pos;
    let pipetexture = load_texture("flappy-bird-assets/sprites/pipe-green.png").await.unwrap();

    
    
    pipetexture.set_filter(FilterMode::Nearest);
    let mut  pipes:Vec<Pipe> = Vec::new();
        
    let mut  timer = 0.0;
    let  timeout = 1.5;
    
    let background_t = load_texture("flappy-bird-assets/sprites/background-day.png").await.unwrap();
    background_t.set_filter(FilterMode::Nearest);        
    // Keep a constant for scaled width (so we don’t repeat 1.5 everywhere)
    let bg_scale = 1.6;
    let bg_width = background_t.width() * bg_scale;
    let bg_height = background_t.height() * bg_scale;
    
    let mut background1 = Background {
        texture: background_t.clone(),
        pos: Vec2 { x: 0.0, y: 0.0 },
    };
    let mut background2 = Background {
        texture: background_t.clone(),
        pos: Vec2 { x: bg_width, y: 0.0 },
    };
   let base_t = load_texture("flappy-bird-assets/sprites/base.png").await.unwrap();
   base_t.set_filter(FilterMode::Nearest);

   let bs_scale = 1.5;
   let bs_width = base_t.width() * bs_scale;
   let bs_height = base_t.height() * bs_scale;
    
    let mut bs1 = Background{
        texture: base_t.clone(),
        pos: Vec2 {x:0.0 , y: screen_height() + 15.0},
    };
    let mut bs2 = Background {
        texture: base_t.clone(),
        pos : Vec2{ x:bs_width , y:screen_height() + 15.0}
    };

    let base_rect = Rect {x:bs1.pos.x , y:bs1.pos.y, w : bs_width,h:bs_height };
    let mut score_rects:Vec<Area> = Vec::new();
    loop {

        timer += get_frame_time();
        if timer > timeout {
            spaw_pipes(&pipetexture, &mut pipes,&mut score_rects); 
            timer = 0.0;
        }
        bird_movement(&mut bird,&mut pipes,&mut score_rects,&mut score,base_rect);
        clear_background(SKYBLUE); // Clear the background

                // Draw both backgrounds
        draw_texture_ex(
            &background1.texture,
            background1.pos.x,
            background1.pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2 { x: bg_width, y: bg_height }),
                ..Default::default()
            },
        );

        draw_texture_ex(
            &background2.texture,
            background2.pos.x,
            background2.pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2 { x: bg_width, y: bg_height }),
                ..Default::default()
            },
        );

        // Scroll speed
        let speed = 53.0 * get_frame_time();

        // Move backgrounds
        background1.pos.x -= speed;
        background2.pos.x -= speed;

        // Reset when fully offscreen
        if background1.pos.x + bg_width <= 0.0 {
            background1.pos.x = background2.pos.x + bg_width;
        }
        if background2.pos.x + bg_width <= 0.0 {
            background2.pos.x = background1.pos.x + bg_width;
        }
          
            

        pipes_gen(&mut pipes,&mut score_rects); 
        // Your game logic here
        
 // Draw both bss
        draw_texture_ex(
            &bs1.texture,
            bs1.pos.x,
            bs1.pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2 { x: bs_width, y: bs_height }),
                ..Default::default()
            },
        );

        draw_texture_ex(
            &bs2.texture,
            bs2.pos.x,
            bs2.pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2 { x: bs_width, y: bs_height }),
                ..Default::default()
            },
        );

        // Scroll speed
        let speedbs = 200.0 * get_frame_time();

        // Move bss
        bs1.pos.x -= speedbs;
        bs2.pos.x -= speedbs;

        // Reset when fully offscreen
        if bs1.pos.x + bs_width <= 0.0 {
            bs1.pos.x = bs2.pos.x + bs_width;
        }
        if bs2.pos.x + bs_width <= 0.0 {
            bs2.pos.x = bs1.pos.x + bs_width;
        }

        bird_drawing(&mut bird);
        if !bird.is_alive {
             if score > highest_score{
                    highest_score = score;
            }
            score = 0; 
            draw_text("You Died!",screen_width() / 3.0 - 10.0, screen_height() / 2.5+ 50.0 , 50.0, BLACK);
            draw_text(&format!("Highest score is {}",highest_score), screen_width()/ 3.8-20.0, screen_height() /1.8 -10.0,35.0, BLACK);
            if is_key_pressed(KeyCode::Space){
                bird.pos = dpos;
                pipes.clear();
                bird.is_alive = true ; 
                bird.vel.y = 0.0;
                bird.rot = 0.0;
                score_rects.clear();
            
            }
        }     
        draw_text(&score.to_string() ,10.0, 50.0,80.0, WHITE);
        next_frame().await; // Wait for the next frame
    }
}

pub fn collision(a: Rect , b : Rect) -> bool{
    a.x < b.x + b.w &&
    a.x + a.w > b.x &&
    a.y < b.y + b.h &&
    a.y + a.h > b.y 
}
