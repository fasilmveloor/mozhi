extern crate mozhi;

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let matches: Vec<String> = env::args().collect();

    if let Some(filename) = matches.get(1) {
        let mut file = File::open(filename).expect("ഫയൽ നിർദിഷ്ട സ്ഥലത്തു കാണാനില്ല!!");
        if !filename.ends_with(".മൊഴി") {
            println!("[പിശക്‌]: മൊഴി പ്രോഗ്രാമുകളുടെ  ഫയൽ എക്സ്റ്റൻഷൻ .മൊഴി ആകുന്നു");
            return;
        }
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("ഫയലിലെ ഉള്ളടക്കങ്ങൾ വായിക്കുന്നതിൽ എന്തോ പിഴവ് സംഭവിച്ചു");
        mozhi::run_file(&contents);
    } 
    // else {
    //     mozhi::run_interactive_shell();
    // }
}
