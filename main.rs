// Write code here.
//
// To see what the code looks like after macro expansion:
//     $ cargo expand
//
// To run the code:
//     $ cargo run
//

use derive_builder::Builder;

#[derive(Builder)]
pub struct Command {
    executable: String,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: String,
}

fn main() {
    let builder = Command::builder();

    let _ = builder;
}
