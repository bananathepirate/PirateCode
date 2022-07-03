use wasm_bindgen::prelude::*;
include!(concat!(env!("OUT_DIR"), "/table.rs"));

///Pirate Code Specifications:
/// Treasure°{encoded text}°
///  - URLs:
///  	"." and "/" are ignored, so as to give the user an idea of what is contained
/// 	
fn pirate_decode(input: Vec<char>) -> Vec<char> {
	let mut read: bool = false;
	let mut decoded = Vec::<char>::new();
	for i in 0..input.len() {
		match input[i] {
			'°' if read == false => read = true, //start
			'°' if read == true => read = false, //end
			'.' if read == true => decoded.push('.'), //ignored
			'/' if read == true => decoded.push('/'), //ignored
			' ' if read == true => decoded.push(' '), //ignored
			//'	' => decoded.push('	'), //ignored
			_ => {
				if read == true {
					match PIRATE_LANG.iter().position(|x| x == &input[i]) {
						Some(n) => {
							decoded.push(PIRATE_LANG[(n - 13) % PIRATE_LANG.len()]);
						},
						None => {
							println!("Not supported: {}", input[i]);
							decoded.push(input[i]);
						}
					}
				}
			}
		}
	}
	decoded
}

fn pirate_encode(input: Vec<char>) -> Vec<char> {
	let mut encoded: Vec<char> = "Treasure °".chars().collect();
	for i in 0..input.len() {
		match input[i] {
			'.' => encoded.push('.'), //ignored
			'/' => encoded.push('/'), //ignored
			' ' => encoded.push(' '), //ignored
			//'	' => encoded.push('	'), //ignored
			_ => {
				match PIRATE_LANG.iter().position(|x| x == &input[i]) {
					Some(n) => {
						encoded.push(PIRATE_LANG[(n + 13) % PIRATE_LANG.len()]);
					},
					None => {
						println!("Not supported: {}", input[i]);
						encoded.push(input[i]);
					}
				}
			}
		}
	}
	encoded.push('°');
	encoded
}	
#[wasm_bindgen]
pub fn wasm_pirate_translate(input: String) -> String {
	pirate_translate(&input)
}

pub fn pirate_translate(input: &String) -> String {
	if !input.starts_with("Treasure") {
		let to_encode: Vec<char> = input.chars().collect();
		return pirate_encode(to_encode).iter().collect::<String>();
	}
	else {
		let encoded: Vec<char> = input.trim_start_matches("Treasure").chars().collect();
		return pirate_decode(encoded).iter().collect::<String>();
	}
}