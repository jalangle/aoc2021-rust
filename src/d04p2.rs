use regex::Regex;

#[path = "util.rs"]
mod util;

#[derive(Clone)]
struct Square {
    pub value : u32,
    pub marked : bool,
}

impl Square {
    pub fn new(value: u32) -> Square {
        Square {
            value: value,
            marked: false,
        }
    }


    #[allow(dead_code)]
    pub fn format(&self) -> String  {
        let mut result : String = "x".to_string();

        if !self.marked {
            result = format!("{}", self.value);
        }

        result
    }
}

#[derive(Clone)]
struct Card {
    pub squares : Vec<Square>,
    pub bingo : bool,
}

impl Card {
    pub fn new(values: &Vec<u32>) -> Card {

        Card  {
            squares: values.iter().map(|x| Square::new(*x)).collect(),
            bingo:  false
        }
    }

    pub fn value(&self) -> u32 {
        self.squares.iter().filter(|s| !s.marked ).map(|s| s.value).sum()
    }

    pub fn bingo(&mut self) -> bool {
        if self.bingo {
            return self.bingo
        }

        /* horizontal bingo */
        for chunk in self.squares.chunks(5) {
            if chunk.iter().all(|s| s.marked ) {
                self.bingo = true
            }
        }

        /* vertical bingo */
        for n in 0..=4 {
            if self.squares.iter().skip(n).step_by(5).all(|s| s.marked) {
                self.bingo =  true
            }
        }

        return self.bingo
    }

    pub fn mark(&mut self, value: u32) -> bool {
        for n in 0..self.squares.len() {
            if self.squares[n].value == value {
                self.squares[n].marked = true;
                break
            }
        }
        
        self.bingo()
    }

    #[allow(dead_code)]
    pub fn format(&self) -> String {
        format!("[ {} {} {} {} {}\n  {} {} {} {} {}\n  {} {} {} {} {}\n  {} {} {} {} {}\n  {} {} {} {} {}  ]\n",
            self.squares[0].format(),
            self.squares[1].format(),
            self.squares[2].format(),
            self.squares[3].format(),
            self.squares[4].format(),
            self.squares[5].format(),
            self.squares[6].format(),
            self.squares[7].format(),
            self.squares[8].format(),
            self.squares[9].format(),
            self.squares[10].format(),
            self.squares[11].format(),
            self.squares[12].format(),
            self.squares[13].format(),
            self.squares[14].format(),
            self.squares[15].format(),
            self.squares[16].format(),
            self.squares[17].format(),
            self.squares[18].format(),
            self.squares[19].format(),
            self.squares[20].format(),
            self.squares[21].format(),
            self.squares[22].format(),
            self.squares[23].format(),
            self.squares[24].format()
            )
    }
}

pub fn begin(args: Vec<String>) {
    let lines  = util::file_to_lines(&args[1]);

    // line 0 is the draw order
    let draw_order : Vec<u32> = lines[0].split(',').map(|x| x.parse().unwrap()).collect();
    //for x in draw_order { println!("{}", x) }

    // skip line 1 for processing bingo cards
    let mut bingo_cards : Vec<Card> = vec![];

    let seperator = Regex::new(r"\s+").expect("Invalid regex");

    for card_lines in lines.iter().skip(1).collect::<Vec<&String>>().chunks(6) {
        if card_lines.len() < 6 {
            break;
        }
        let mut all_values : Vec<u32> = vec![];
        for line in card_lines.iter().skip(1) {
            let mut line_values : Vec<u32> = seperator.split(line.trim()).map( |x| { x.parse().unwrap() } ).collect();
            all_values.append(&mut line_values);
        }
        let card = Card::new(&all_values);
        bingo_cards.insert(bingo_cards.len(), card);
    }

    let mut last_bingo : Option<Card> = None;
    let mut last_draw : Option<u32> = None;

    for n in draw_order {
        for c in &mut bingo_cards {
            if c.bingo {
                continue;
            }

            if c.mark(n) {
                last_bingo = Some(c.clone());
                last_draw = Some(n);
            }
        }
    }

    let lb = last_bingo.unwrap();
    println!("Value: {}", lb.value());
    println!("Score: {}", lb.value() * last_draw.unwrap());
}