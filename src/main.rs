fn main() {
    let module = std::env::args().nth(1);
    match module {
        Some(module_name) => {
            let args = std::env::args().skip(1).collect();
            if module_name == "d01p1" {
                #[path = "d01p1.rs"] // this is the stupidest hack ever to make Rust let me inline declare the mod use
                mod d01p1;
                d01p1::begin(args);
            }
            else if module_name == "d01p2" {
                #[path = "d01p2.rs"] 
                mod d01p2;
                d01p2::begin(args);
            }
            else if module_name == "d02p1" {
                #[path = "d02p1.rs"] 
                mod d02p1;
                d02p1::begin(args);
            }
            else if module_name == "d02p2" {
                #[path = "d02p2.rs"] 
                mod d02p2;
                d02p2::begin(args);
            }
            else if module_name == "d03p1" {
                #[path = "d03p1.rs"] 
                mod d03p1;
                d03p1::begin(args);
            }
            else if module_name == "d03p2" {
                #[path = "d03p2.rs"] 
                mod d03p2;
                d03p2::begin(args);
            }
            else if module_name == "d04p1" {
                #[path = "d04p1.rs"] 
                mod d04p1;
                d04p1::begin(args);
            }
            else if module_name == "d04p2" {
                #[path = "d04p2.rs"] 
                mod d04p2;
                d04p2::begin(args);
            }
            else if module_name == "d05p1" {
                #[path = "d05p1.rs"] 
                mod d05p1;
                d05p1::begin(args);
            }
            else if module_name == "d05p2" {
                #[path = "d05p2.rs"] 
                mod d05p2;
                d05p2::begin(args);
            }
            else if module_name == "d06p1" {
                #[path = "d06p1.rs"] 
                mod d06p1;
                d06p1::begin(args);
            }
            else if module_name == "d07p1" {
                #[path = "d07p1.rs"] 
                mod d07p1;
                d07p1::begin(args);
            }
            else if module_name == "d07p2" {
                #[path = "d07p2.rs"] 
                mod d07p2;
                d07p2::begin(args);
            }
            else {
                #[path = "template.rs"] 
                mod template;
                template::begin(args);
            }
        },
        None => {
            println!("No module given");
        }
    }
}
