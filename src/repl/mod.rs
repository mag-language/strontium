use linefeed::{Interface, ReadResult};
use colored::*;

use super::Strontium;

const LOGO: &'static str = "
           **                              **   **                    
          /**                             /**  //                     
  ****** ****** ******  ******  *******  ****** ** **   ** ********** 
 **//// ///**/ //**//* **////**//**///**///**/ /**/**  /**//**//**//**
//*****   /**   /** / /**   /** /**  /**  /**  /**/**  /** /** /** /**
 /////**  /**   /**   /**   /** /**  /**  /**  /**/**  /** /** /** /**
 ******   //** /***   //******  ***  /**  //** /**//****** *** /** /**
//////     //  ///     //////  ///   //    //  //  ////// ///  //  // ";

pub fn launch(show_logo: bool) {
	let mut machine = Strontium::new();

	if show_logo {
		println!("{}\n", LOGO.green().bold());
	}

	let mut reader = Interface::new("strontium").unwrap();

	println!("Launching bytecode interpreter");

	reader.set_prompt(format!("{}", "hex> ".bright_red().bold()).as_str()).unwrap();

	while let ReadResult::Input(input) = reader.read_line().unwrap() {
		match input.as_str() {
				":registers" => {
					println!("{:?}", machine.registers);
				},

				_ => {
					if let Ok(bytes) = hex::decode(input.replace(" ", "")) {
						let string = String::from_utf8_lossy(&bytes).into_owned();

						machine.push_bytecode(&bytes[..]);
						
						match machine.execute_until_halt() {
							Ok(_) => {},
							Err(e) => println!("{:?}", e),
						}	
					}
				}
			}
	}
}