mod d01p1;
mod d01p2;
mod d02p1;
mod d02p2;
mod d03p1;
mod d03p2;
mod d04p1;
mod d04p2;
mod d05p1;
mod d05p2;
mod d06p1;
mod d07p1;
mod d07p2;
mod d08p1;
mod template;

fn main() {
    let module = std::env::args().nth(1);
    match module {
        Some(module_name) => {
            let args = std::env::args().skip(1).collect();
            if module_name == "d01p1" {
                d01p1::begin(args);
            }
            else if module_name == "d01p2" {
                d01p2::begin(args);
            }
            else if module_name == "d02p1" {
                d02p1::begin(args);
            }
            else if module_name == "d02p2" {
                d02p2::begin(args);
            }
            else if module_name == "d03p1" {
                d03p1::begin(args);
            }
            else if module_name == "d03p2" {
                d03p2::begin(args);
            }
            else if module_name == "d04p1" {
                d04p1::begin(args);
            }
            else if module_name == "d04p2" {
                d04p2::begin(args);
            }
            else if module_name == "d05p1" {
                d05p1::begin(args);
            }
            else if module_name == "d05p2" {
                d05p2::begin(args);
            }
            else if module_name == "d06p1" {
                d06p1::begin(args);
            }
            else if module_name == "d07p1" {
                d07p1::begin(args);
            }
            else if module_name == "d07p2" {
                d07p2::begin(args);
            }
            else if module_name == "d08p1" {
                d08p1::begin(args);
            }
            else {
                template::begin(args);
            }
        },
        None => {
            println!("No module given");
        }
    }
}
