mod d01p1;
mod d01p2;
mod d02p1;
mod d02p2;

fn main() {
    let module = std::env::args().nth(1);
    match module {
        Some(module_name) => {
            if module_name == "d01p1" {
                d01p1::begin();
            }
            if module_name == "d01p2" {
                d01p2::begin();
            }
            if module_name == "d02p1" {
                d02p1::begin();
            }
            if module_name == "d02p2" {
                d02p2::begin();
            }
            else {
                println!("boo");
            }
        },
        None => {
            println!("No module given");
        }
    }
}
