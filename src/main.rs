pub mod pokemon;
pub mod types;

use pokemon::species::Species;

fn main() {
    let path = "data/pokemon/bulbasaur.json";
	match Species::from_json_file(path, None, None) {
		Ok(species) => {
			println!("Species: {}", species.name);
			if let Some(variant) = &species.selected_variant {
				println!("Selected Variant: {}", variant);
			} else {
				println!("No variant selected.");
			}
		}
		Err(e) => println!("Error loading species: {}", e),
	}
}
