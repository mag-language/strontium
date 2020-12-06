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
		let input_without_spaces = input.replace(" ", "");
		if let Ok(bytes) = hex::decode(input_without_spaces) {

			match input.as_str() {
				":registers" => {
					println!("{:?}", machine.registers);
				},

				_ => {
					println!("got input {:?}", bytes);
					machine.push_bytecode(&bytes[..]);
					machine.execute_until_halt();
				}
			}
		}
	}
}