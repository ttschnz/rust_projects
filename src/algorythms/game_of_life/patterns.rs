pub enum Patterns {
    Copperhead,
    SnarkLoop,
    GosperGliderGun,
    SirRobin,
    GPT,
}

impl Patterns {
    fn rle(&self) -> String {
        match self {
            Self::Copperhead => {
                "$5bob2o$4bo6bo$3b2o3bo2bo$2obo5b2o$2obo5b2o$3b2o3bo2bo$4bo6bo$5bob2o!".to_string()
            }
            Self::SnarkLoop =>{
                "27b2o$27bobo$29bo4b2o$25b4ob2o2bo2bo$25bo2bo3bobob2o$28bobobobo$29b2obobo$33bo2$19b2o$20bo8bo$20bobo5b2o$21b2o$35bo$36bo$34b3o2$25bo$25b2o$24bobo4b2o22bo$31bo21b3o$32b3o17bo$34bo17b2o2$45bo$46b2o12b2o$45b2o14bo$3b2o56bob2o$4bo9b2o37bo5b3o2bo$2bo10bobo37b2o3bo3b2o$2b5o8bo5b2o35b2obo$7bo13bo22b2o15bo$4b3o12bobo21bobo12b3o$3bo15b2o22bo13bo$3bob2o35b2o5bo8b5o$b2o3bo3b2o37bobo10bo$o2b3o5bo37b2o9bo$2obo56b2o$3bo14b2o$3b2o12b2o$19bo2$11b2o17bo$12bo17b3o$9b3o21bo$9bo22b2o4bobo$38b2o$39bo2$28b3o$28bo$29bo$42b2o$35b2o5bobo$35bo8bo$44b2o2$31bo$30bobob2o$30bobobobo$27b2obobo3bo2bo$27bo2bo2b2ob4o$29b2o4bo$35bobo$36b2o!".to_string()
            }
            Self::GosperGliderGun=>{
                "24bo11b$22bobo11b$12b2o6b2o12b2o$11bo3bo4b2o12b2o$2o8bo5bo3b2o14b$2o8bo3bob2o4bobo11b$10bo5bo7bo11b$11bo3bo20b$12b2o!".to_string()
            }
            Self::SirRobin=>{
                "6bo3bob2o$6b3obo3bo18b3o$5b2o4b3o3b2o15b2o2b2o$5bo3bo6b2o14bo2bobo$o3bo4b2obo3bo2bobo11bo4bobo$bob3o5b3o2bo4bo13bo4b2o$bo3b4obob2o3bob2obo2b3o2bo3bobo4bobob2o$6b3o2bo2b2o5b2o2b2obo3b2o2b2o3bobob2o$7b2o2bo2b2o5b2o2b2obobob2o4bo2bo11b2o$12bobo6b2o2bo2bob2obo5bob2o4bo5bobo$21bobob2ob2o3b2o3b2o2b2o3bo5bo3bo$20bo5b3o5bob2obo4bo2bo5bo3b4o$20bo3b3o8bo9b2o3bo2bo3bo5b3o$20bo24bob3obo6bo2bo$22bo22b3o3bo10bobob2o$45b2o2b2o3bo3b2ob4obobo2b2o$46bob3obo5bo5bo7bo$45b2o2b2obo6bobo4bo5b3o$52b2o6bobobo3bo4b2o$47bobo4b2obob2o2b3ob2ob2ob2o$48bo11b4obo4b2ob2o2$68bo6b2o$68bo6bobo$68bo3bo2bo$69bobobo4bo$71bobo2b3o$67bo3bo2bo$67bo3bob2o$69bo2bo$70bo!".to_string()
            }
            Self::GPT=>{
                "obo$b2o$bo!".to_string()
            }
        }
    }
    pub fn get_life_positions(&self) -> Vec<(u8, u8)> {
        let rle = self.rle();
        let mut map = vec![];
        let mut position = (0, 0);
        let mut cumulated = String::new();
        for c in rle.chars() {
            let count = str::parse(&cumulated).unwrap_or(1);
            match c {
                'b' => {
                    position.0 += count;
                    cumulated = String::new()
                }
                'o' => {
                    for x in position.0..position.0 + count {
                        map.push((x, position.1))
                    }
                    position.0 += count;
                    cumulated = String::new()
                }
                '$' => {
                    position.1 += count;
                    position.0 = 0;

                    cumulated = String::new()
                }
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {
                    cumulated.push(c);
                }
                _ => {
                    //skip
                }
            };
        }
        map
    }
}

#[cfg(test)]
mod test {
    use super::Patterns;

    #[test]
    fn manual_check() {
        println!("{:?}", Patterns::SnarkLoop.get_life_positions())
    }
    #[test]
    fn rle_parsing() {
        assert_eq!(
            Patterns::Copperhead.get_life_positions(),
            vec![
                (5, 0),
                (7, 0),
                (8, 0),
                (4, 1),
                (11, 1),
                (3, 2),
                (4, 2),
                (8, 2),
                (11, 2),
                (0, 3),
                (1, 3),
                (3, 3),
                (9, 3),
                (10, 3),
                (0, 4),
                (1, 4),
                (3, 4),
                (9, 4),
                (10, 4),
                (3, 5),
                (4, 5),
                (8, 5),
                (11, 5),
                (4, 6),
                (11, 6),
                (5, 7),
                (7, 7),
                (8, 7)
            ]
        )
    }
}
