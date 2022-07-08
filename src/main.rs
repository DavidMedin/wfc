use std::{path::Path, sync::Arc, string, io::{BufReader, Read}, error::Error};
use serde::{Deserialize,Serialize};

// Use packages
use ggez::{*, 
    event::EventHandler, 
    graphics::{Color, set_window_title, Image, Drawable, DrawParam}, conf::{WindowSetup, NumSamples}
};

struct Tile {
    id : u32,
    image : Image,
    sides : Vec<u32>
}

struct Node {
    tile : usize,
    neighbors : Vec<usize>
}

// SerDe Version of data =================
#[derive(Deserialize,Serialize)]
struct SerdeTile {
    image : String,
    name : String,
    north : String,
    west : String,
    south : String,
    east : String
}

#[derive(Deserialize,Serialize)]
struct SerdeData {
    tiles : Vec<SerdeTile>
}
//=========================================


// Game Context / Event Handler
struct Game {
    tiles : Vec<Tile>, // Definition of tiles.
    nodes : Vec<Node> // Nodes that are placed.
}

impl Game {
    fn new(ctx : &mut Context) -> GameResult<Self>{
        // Load Data
        let mut serde_data : SerdeData = match ggez::filesystem::open(ctx, "/data.json") {
            Ok(data_f) => {

                // Opened data.json file, read and interpret it.
                let mut reader = BufReader::new(data_f);

                match serde_json::from_reader(reader) {
                    // Interpret the data into SerdeData
                    Ok(data) => data,
                    Err(e) => {
                        // Error interpreting the error, early return with error.
                        return Err(GameError::CustomError( format!("Error interpreting JSON data! : {}", e)  ));
                    },
                }


            },
            Err(e) => {
                // Failed to open file, early return with error.
                return Err(e);
            },
        };

        //libjpeg-progs is removed

        // Create tiles
        let mut tiles : Vec<Tile> = vec![];
        let mut side_enum : u32 = 0;
        let mut side_types : Vec<String> = vec![];
        for tile in serde_data.tiles {

            let image = match Image::new(ctx, "/".to_string() + &tile.image ) {
                Ok(image) => image,
                Err(e) => {
                    return Err(e)
                }
            };

            let sides : Vec<u32> = vec![];
            for dir in [&tile.north , &tile.west, &tile.south, &tile.east] {

                let mut found : bool = false;
                for side_type in &side_types {
                    if side_type == dir {
                        found = true;
                        break; // break early, we found it.
                    }
                }
                if found == false {
                    // this tile introduces a new side type.
                    println!("New side type : {}", dir );
                    side_types.push(*dir);

                }

            }

            tiles.push( 
                Tile { 
                    image,
                    neighbors: ()
                }
            )
        }

        // Load images
        // let image = match Image::new(ctx, Path::new("/left-down.png") ) {
        //     Ok(img) => {img},
        //     Err(e) => {
        //         println!("Failed to create image!");
        //         return Err(e);
        //     },
        // };

        Ok(Game{
            image,
            nodes : vec![]
        })
    }
}
impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        graphics::clear(ctx,Color::from_rgb_u32(0x264653));

        // graphics::set_canvas(ctx, None);
        self.image.draw(ctx, DrawParam::new())?;

        graphics::present(ctx)
    }
}



// Main Function
fn main() -> Result<(),GameError> {
    println!("Hello, world!");
    
    // Create ggez event loop
    let mut cb = ContextBuilder::new("wfc", "David Medin").add_resource_path( "/home/david/repos/wfc/resources" ).window_setup(
        WindowSetup{ 
            title: "wave function collapse".to_owned(),
            samples: NumSamples::One,
            vsync: true,
            icon: "".to_owned(),
            srgb: true,
        }
    );
    
    let (mut ctx, event_loop) = cb.build().expect("Failed to start game!");
    
    
    // //Read Serial
    // let mut file = ggez::filesystem::open(&ctx, "/data.json")?;
    // let mut data_str : [u8;2000] = [0;2000];
    // let read = file.read(&mut data_str)?;
    // if 2000 == read {
    //     println!("Data is too big!");
    // }
    // match serde_json::from_slice::<SerdeData>(&data_str[..read]) {

    //     Ok(data) => {
    //         //Write serial
    //         // let data = test_data{ name: "Hello!".to_string(), num: 4 };
    //         let serial = serde_json::to_string(&data).unwrap();
    //         println!("data = {}",serial);
    //     }
        
    //     Err(e) => {
    //         println!("error! : {}",e);
    //     }

    // }
    

    let game = Game::new(&mut ctx)?;

    event::run(ctx,event_loop,game);
}

