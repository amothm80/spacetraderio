use console_engine::ConsoleEngine;
use console_engine::pixel;
use console_engine::Color;
use console_engine::KeyCode;

use crate::models;
use crate::models::*;



pub fn print_main_screen(engine:&mut ConsoleEngine){


   
    engine.line(0, 4, 119, 4, pixel::pxl_fg('#',Color::Cyan)); //middle horizontal separator

    engine.line(0, 0, 119, 0, pixel::pxl_fg('#',Color::Cyan)); //top line
    engine.line(0, 29, 119, 29, pixel::pxl_fg('#',Color::Cyan)); //bottom line

    engine.line(0, 0, 0, 29, pixel::pxl_fg('#',Color::Cyan));  //left line
    engine.line(119, 0, 119, 29, pixel::pxl_fg('#',Color::Cyan)); //right line
    engine.line(23, 0, 23, 29, pixel::pxl_fg('#',Color::Cyan)); //middle vertical separator
    

    engine.print(50,2, format!("S  P  A  C  E        T  R  A  D  E  R  S").as_str()); //title
    
}

pub fn print_content_contracts(engine:&mut ConsoleEngine, content: &Vec<models::contract::Contract>){
    let mut start_line:i32 = 5;
    for contract in content{
        engine.print(26,start_line, format!("{}",contract).as_str()); // prints some value at [0,4]
        start_line += 3 + contract.terms.deliver.len() as i32;
    }
}