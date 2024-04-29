extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::Duration;
use rand::seq::SliceRandom;

const SCREEN_WIDTH: u32 = 600;
const SCREEN_HEIGHT: u32 = 600;
const ARRAY_SIZE : usize = 200;



pub fn main() {
    
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("sorting-rs", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap()
    ;

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    
    let mut array: [i32; ARRAY_SIZE] = [0; ARRAY_SIZE];
    for (index, element) in array.iter_mut().enumerate() {
        *element = (index + 1) as i32;
    }
    array.shuffle(&mut rand::thread_rng());

    let bar_px_width: i32 = SCREEN_WIDTH as i32 / ARRAY_SIZE as i32;

    let min_value = *array.iter().min().unwrap();
    let max_value = *array.iter().max().unwrap();

    let mut color_table : [Color; 7] = [
        Color::RGB(0xFF, 0xAD, 0xAD), 
        Color::RGB(0xFF, 0xD6, 0xA5), 
        Color::RGB(0xFD, 0xFF, 0xB6),
        Color::RGB(0x9B, 0xF6, 0xFF), 
        Color::RGB(0xA0, 0xC4, 0xFF),
        Color::RGB(0xBD, 0xB2, 0xFF), 
        Color::RGB(0xFF, 0xC6, 0xFF)
    ];

    'running: loop {       

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        for (index, &value) in array.iter().enumerate() {
            let frac = inv_lerp(min_value as f64, max_value as f64, value as f64);
            let bar_px_height: u32 = lerp(0 as f64, SCREEN_HEIGHT as f64, frac) as u32;
            
            let color_table_i = lerp(0 as f64, 6 as f64, frac) as usize;
            canvas.set_draw_color(color_table[color_table_i]);
            canvas.fill_rect(Rect::new(index as i32 * bar_px_width, (SCREEN_HEIGHT - bar_px_height) as i32, bar_px_width as u32, bar_px_height)).unwrap();
            canvas.set_draw_color(Color::GRAY);
            canvas.draw_rect(Rect::new(index as i32 * bar_px_width, (SCREEN_HEIGHT - bar_px_height) as i32, bar_px_width as u32, bar_px_height)).unwrap();
        }

        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
   
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    array.shuffle(&mut rand::thread_rng());
                },

                Event::KeyDown { keycode: Some(Keycode::Space), .. } => { 
                    

                    /* bubble */
                    
                    let n = array.len();
                    for i in 0..n {
                        for j in 0..n-i-1 {
                            if array[j] > array[j+1] {
                                array.swap(j, j+1);
                
                                canvas.set_draw_color(Color::BLACK);
                                canvas.clear();
                                
                                for (index, &value) in array.iter().enumerate() {
                                    let frac = inv_lerp(min_value as f64, max_value as f64, value as f64);
                                    let bar_px_height: u32 = lerp(0 as f64, SCREEN_HEIGHT as f64, frac) as u32;
                                    
                                    if (index != j) {
                                        let color_table_i = lerp(0 as f64, 6 as f64, frac) as usize;
                                        canvas.set_draw_color(color_table[color_table_i]);
                                        canvas.fill_rect(Rect::new(index as i32 * bar_px_width, (SCREEN_HEIGHT - bar_px_height) as i32, bar_px_width as u32, bar_px_height)).unwrap();
                                        canvas.set_draw_color(Color::GRAY);
                                        canvas.draw_rect(Rect::new(index as i32 * bar_px_width, (SCREEN_HEIGHT - bar_px_height) as i32, bar_px_width as u32, bar_px_height)).unwrap();
                                    } else {
                                        canvas.set_draw_color(Color::WHITE);
                                        canvas.fill_rect(Rect::new(index as i32 * bar_px_width, 0 as i32, bar_px_width as u32, SCREEN_HEIGHT)).unwrap();
                                    }

                                }

                                canvas.present();
                                std::thread::sleep(Duration::new(0, 500_000));
                            }
                        }
                    }

                    /* selection */
                        
                    // let n = array.len();
                    // for i in 0..n {
                    //     canvas.set_draw_color(Color::BLACK);
                    //     canvas.clear();
                        
                    //     let mut min_index = i;
                    //     for j in (i + 1)..n {
                    //         if array[j] < array[min_index] {
                    //             min_index = j;
                    //         }
                    //     }
                    //     if min_index != i {
                    //         array.swap(i, min_index);
                    //     }

                    //     for (index, &value) in array.iter().enumerate() {
                    //         let frac = inv_lerp(min_value as f64, max_value as f64, value as f64);
                    //         let bar_px_height: u32 = lerp(0 as f64, SCREEN_HEIGHT as f64, frac) as u32;
                            
                    //         let color_table_i = lerp(0 as f64, 5 as f64, frac) as usize;
                    //         canvas.set_draw_color(color_table[color_table_i]);
                            
                    //         canvas.fill_rect(Rect::new(index as i32 * bar_px_width, (SCREEN_HEIGHT - bar_px_height) as i32, bar_px_width as u32, bar_px_height)).unwrap();
                    //     }
                    //     canvas.present();
                    //     std::thread::sleep(Duration::new(0, 3_000_000));
                    // }

                },

                _ => {}
            }
        }

    }
}

fn lerp(a: f64, b: f64, t: f64) -> f64 { (1.0 as f64 - t) * a + b * t }
fn inv_lerp(a: f64, b: f64, v: f64) -> f64 { (v-a)/(b-a) }