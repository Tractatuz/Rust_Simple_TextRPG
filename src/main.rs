mod base;

use crate::base::*;
use crate::base::maingame::*;

fn clear_console()
{
    let os = std::env::consts::OS;
    
    match os 
    {
        "macos" => { std::process::Command::new("clear").status().unwrap(); },
        "windows" => { std::process::Command::new("powershell").arg("/c").arg("cls").status().unwrap(); },
        _ => println!("Unknown OS"),
    }
}

fn main() 
{
    let mut main_game : MainGame = MainGame::new();

    main_game.render();

    loop 
    {
        if main_game.update() == 0
        {
            break;
        }
        
        main_game.render();
    }

    clear_console();
}