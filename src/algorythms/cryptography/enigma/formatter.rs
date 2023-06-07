use super::{Enigma, Reflector, Rotor};

use std::{
    borrow::Borrow,
    cell::RefCell,
    cmp::Ordering,
    fmt::{Debug, Formatter},
    rc::Rc,
};

impl Debug for Enigma {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        //                            Enigma
        //                            ══════
        //         ┌─────────────┬───────────┬─────────┬──────────┐
        //         │Reflector (A)│Rotor (III)│Rotor (I)│Rotor (II)│
        //         ╞═════════════╪═══════════╪═════════╪══════════╡
        //         │            A│          A│        A│     ╔═══A│←
        //         │            B│          B│        B│     ║   B│
        //         │            C│          C│        C│     ║   C│
        //         │            D│          D│    ╔═══D│═════╝   D│
        //         │            E│          E│   ╔╬═══E│═══╗     E│
        //         │            F│          F│   ║║   F│   ║     F│
        //         │            G│          G│   ║║   G│   ║     G│
        //         │            H│     ╔════H│═══╬╝   H│   ║     H│
        //         │       ╔════I│════╗║    I│   ║    I│   ║     I│
        //         │       ║    J│    ║║    J│   ║    J│   ║     J│
        //         │       ║    K│    ║║    K│   ║    K│   ║     K│
        //         │       ║    L│    ║║    L│   ║    L│   ║     L│
        //         │       ║    M│    ║║    M│   ║    M│   ║     M│
        //         │       ║    N│    ╚╬════N│═══╝    N│   ║     N│
        //         │       ║    O│     ║    O│        O│   ║     O│
        //         │       ║    P│     ║    P│        P│   ║     P│
        //         │       ║    Q│     ║    Q│        Q│   ║     Q│
        //         │       ║    R│     ║    R│        R│   ╚═════R│→
        //         │       ║    S│     ║    S│        S│         S│
        //         │       ║    T│     ║    T│        T│         T│
        //         │       ║    U│     ║    U│        U│         U│
        //         │       ╚════V│═════╝    V│        V│         V│
        //         │            W│          W│        W│         W│
        //         │            X│          X│        X│         X│
        //         │            Y│          Y│        Y│         Y│
        //         │            Z│          Z│        Z│         Z│
        //         └─────────────┴───────────┴─────────┴──────────┘
        // f.debug_struct("Enigma")
        //     // .field("rotors", &self.x)
        //     // .field("reflector", &self.y)
        //     .finish()

        let pt = self.rotors.iter().rev().fold(
            vec![(
                Rc::new(RefCell::new(TablePart::new_reflector(&self.reflector))),
                0,
            )],
            |mut acc, curr| {
                acc.push((Rc::new(RefCell::new(TablePart::new_rotor(curr))), acc.len()));
                acc
            },
        );

        let dagram_width = pt.iter().fold(pt.len() + 1, |acc, curr| {
            acc + curr.0.borrow_mut().width as usize
        });
        let title = "Enigma v0.1";

        f.write_fmt(format_args!("{: ^1$}", title, dagram_width))
            .unwrap();
        f.write_str("\n").unwrap();
        f.write_fmt(format_args!(
            "{: ^1$}",
            "═".repeat(title.len()),
            dagram_width
        ))
        .unwrap();
        f.write_str("\n").unwrap();

        let mut all_done = false;
        let initial_input: usize = 0 as usize;
        let mut line = 0;

        while all_done == false {
            let (mut forward, input) = pt
                .iter()
                .rev() // from right to left
                .fold(
                    (vec![], initial_input),
                    |(mut acc, input_right), (part, index)| {
                        // println!("feeding {} with {}", part.borrow_mut().title, input_right);
                        let (line_part, next_input) = part.borrow_mut().start_line(input_right);
                        // println!("next input = {}", next_input);
                        acc.push((line_part, index));
                        (acc, next_input)
                    },
                );
            forward.reverse();

            let output = pt.iter().zip(forward).fold(
                input,
                |input_left, ((part, index), (started, _index))| {
                    // println!("feeding {} with {}", part.borrow_mut().title, input_left);
                    let (mut out, next_input) = part.borrow_mut().finish_line(input_left, started);
                    if index != &(pt.len() - 1) {
                        out.pop();
                    }
                    f.write_str(&out).unwrap();
                    next_input
                },
            );

            all_done = pt.iter().filter(|x| !x.0.borrow_mut().finished).count() == 0;

            if line >= 3 {
                if output == line - 3 {
                    f.write_str("→").unwrap();
                }
                if initial_input == line - 3 {
                    f.write_str("←").unwrap();
                }
            }

            f.write_str("\n").unwrap();
            line += 1;
        }

        Ok(())
    }
}

// use super::{Reflector, Rotor};

#[allow(dead_code)]
pub enum Borders {
    None = 32,                             //" "
    HorizontalSingle = 9472,               //"─"
    VerticalSingle = 9474,                 //"│"
    BottomRightSingle = 9484,              //"┌"
    BottomLeftSingle = 9488,               //"┐"
    TopRightSingle = 9492,                 //"└"
    TopLeftSingle = 9496,                  //"┘"
    VerticalLeftSingle = 9500,             //"├"
    VerticalRightSingle = 9508,            //"┤"
    HorizontalBottomSingle = 9516,         //"┬"
    HorizontalTopSingle = 9524,            //"┴"
    VerticalHorizontalSingle = 9532,       //"┼"
    HorizontalDouble = 9552,               //"═"
    VerticalDouble = 9553,                 //"║"
    BottomSingleRightDouble = 9554,        //"╒"
    BottomDoubleRightSingle = 9555,        //"╓"
    BottomRightDouble = 9556,              //"╔"
    BottomSinlgeLeftSingle = 9557,         //"╕"
    BottomDoubleLeftSingle = 9558,         //"╖"
    BottomLeftDouble = 9559,               //"╗"
    TopSingleRightDouble = 9560,           //"╘"
    TopDoubleRightSingle = 9561,           //"╙"
    TopRightDouble = 9562,                 //"╚"
    TopSingleLeftDouble = 9563,            //"╛"
    TopDoubleLeftSingle = 9564,            //"╜"
    TopLeftDouble = 9565,                  //"╝"
    VerticalSingleRightDouble = 9566,      //"╞"
    VerticalDoubleRightSingle = 9567,      //"╟"
    VerticalRightDouble = 9568,            //"╠"
    VerticalSingleLeftDouble = 9569,       //"╡"
    VerticalDoubleLeftSingle = 9570,       //"╢"
    VerticalDoubleLeftDouble = 9571,       //"╣"
    HorizontalDoubleBottomSingle = 9572,   //"╤"
    HorizontalSingleBottomDouble = 9573,   //"╥"
    HorizontalBottomDouble = 9574,         //"╦"
    HorizontalDoubleTopSingle = 9575,      //"╧"
    HorizontalSingleTopTouble = 9576,      //"╨"
    HorizontalDoubleTopDouble = 9577,      //"╩"
    VerticalSingleHorizontalDouble = 9578, //"╪"
    VerticalDoubleHorizontalSingle = 9579, //"╫"
    VerticalHorizontalDouble = 9580,       //"╬"
}
impl Borders {
    pub fn get_char(self) -> char {
        std::char::from_u32(self as u32).unwrap()
        // self as u8 as char
    }
    pub fn n_times(self, count: usize) -> String {
        self.get_char().to_string().repeat(count)
    }
}

pub struct TablePart {
    pub width: u8,
    title: String,
    has_incoming: bool,
    has_outgoing: bool,
    line_index: usize,
    pub finished: bool,
    configuration: [char; 26],
    reflect: bool,
}
impl TablePart {
    pub fn new_rotor(rotor: &Rc<RefCell<Rotor>>) -> TablePart {
        let title = format!("Rotor ({})", rotor.borrow_mut().get_name());
        TablePart {
            width: title.len() as u8,
            title: title,
            has_incoming: false,
            has_outgoing: false,
            line_index: 0,
            finished: false,
            configuration: rotor.borrow_mut().configuration,
            reflect: false,
        }
    }
    pub fn new_reflector(reflector: &Rc<RefCell<Reflector>>) -> TablePart {
        let title = format!("Reflector ({})", reflector.borrow_mut().get_name());
        TablePart {
            width: title.len() as u8,
            title: title,
            has_incoming: false,
            has_outgoing: false,
            line_index: 0,
            finished: false,
            configuration: reflector.borrow_mut().configuration,
            reflect: true,
        }
    }

    ///
    /// returns String and output on the left
    ///
    pub fn start_line(&mut self, input_right: usize) -> (String, usize) {
        let output_left =
            (self.configuration[input_right].to_ascii_uppercase() as u8 - 65) as usize;
        let line = if self.finished {
            "".to_string()
        } else {
            match self.line_index {
                0 => {
                    Borders::HorizontalBottomSingle.get_char().to_string()
                        // + &Borders::HorizontalSingle.n_times(self.width as usize)
                        + &format!("{}->{}", input_right, output_left)
                        + &Borders::HorizontalBottomSingle.get_char().to_string()
                }

                1 => {
                    Borders::VerticalSingle.get_char().to_string()
                        + &self.title.clone()
                        + &Borders::VerticalSingle.get_char().to_string()
                }
                2 => {
                    Borders::VerticalSingleHorizontalDouble
                        .get_char()
                        .to_string()
                        + &Borders::HorizontalDouble.n_times(self.width as usize)
                        + &Borders::VerticalSingleHorizontalDouble
                            .get_char()
                            .to_string()
                }
                29 => {
                    Borders::HorizontalTopSingle.get_char().to_string()
                        + &Borders::HorizontalSingle.n_times(self.width as usize)
                        + &Borders::HorizontalTopSingle.get_char().to_string()
                }

                _ => {
                    //  A = 0, Z = 25
                    // println!(
                    //     "\n{}->{}({})\n",
                    //     input_right,
                    //     output_left,
                    //     self.configuration[input_right].to_uppercase()
                    // );
                    let half_width = (self.width as usize - 3) / 2;

                    let conn_right = if input_right == (self.line_index - 3) {
                        self.has_incoming = true;
                        Borders::HorizontalDouble.n_times(half_width)
                    } else {
                        ' '.to_string().repeat(half_width)
                    };

                    let pipe_right = match (
                        input_right == self.line_index - 3,
                        output_left == self.line_index - 3,
                        output_left.cmp(&input_right),
                        (self.line_index - 3).cmp(&input_right),
                        (self.line_index - 3).cmp(&output_left),
                    ) {
                        // BottomRightDouble if right == now && left > right
                        (true, false, Ordering::Greater, _, _) => Borders::BottomRightDouble,
                        // TopRightDouble if right == now && left < right
                        (true, false, Ordering::Less, _, _) => Borders::TopRightDouble,
                        // BottomLeftDouble if left == now && left > right
                        (false, true, Ordering::Greater, _, _) => Borders::TopLeftDouble,
                        // TopLeftDouble if left == now && left < right
                        (false, true, Ordering::Less, _, _) => Borders::BottomLeftDouble,
                        // HorizontalDouble if right == now && left == now
                        (true, true, Ordering::Equal, _, _) => Borders::HorizontalDouble,
                        // VerticalDouble if now < right && now > left || now > right && now < left
                        (false, false, _, Ordering::Less, Ordering::Greater)
                        | (false, false, _, Ordering::Greater, Ordering::Less) => {
                            Borders::VerticalDouble
                        }
                        _ => Borders::None,
                    }
                    .get_char();

                    let conn_left = if output_left == (self.line_index - 3) {
                        self.has_incoming = false;
                        Borders::HorizontalDouble
                            .n_times(half_width + 1 + ((self.width as usize - 3) % 2))
                    } else {
                        ' '.to_string()
                            .repeat(half_width + 1 + ((self.width as usize - 3) % 2))
                    };

                    // let pipe_left = match (
                    //     output_left == self.line_index,
                    //     self.has_incoming,
                    //     output_left.cmp(&input_right),
                    // ) {
                    //     (true, _, Ordering::Equal) => Borders::HorizontalDouble,
                    //     (true, _, Ordering::Less) => Borders::BottomLeftDouble,
                    //     (true, _, Ordering::Greater) => Borders::TopLeftDouble,
                    //     (false, true, _) => Borders::VerticalDouble,
                    //     _ => Borders::None,
                    // }
                    // .get_char();

                    format!(
                        "{1}{conn_left}{pipe_right}{conn_right}{0}{1}",
                        (self.line_index as u8 - 3 + 65) as char, // char on the right
                        // self.line_index - 3,
                        Borders::VerticalSingle.get_char()
                    )
                }
            }
        };
        if self.line_index >= 29 {
            self.finished = true;
        } else {
            self.line_index += 1;
        }
        (line, output_left)
        // line 0 = ----
        // line 1 = Title
        // line 2 = ====
        // line 3... = A...
        // line 26+3 = ----
    }

    ///
    /// Returns String and output on the right
    /// TODO
    pub fn finish_line(&mut self, input_left: usize, started: String) -> (String, usize) {
        (started, input_left)
    }
}

#[cfg(test)]
mod test {
    use super::Borders;
    use super::{
        super::{ReflectorWiring, RotorWiring},
        Enigma,
    };
    #[test]
    fn gets_char() {
        let b = Borders::HorizontalSingle;
        assert_eq!(b.get_char(), '─');
    }

    #[test]
    fn enigma_formatting() {
        let enigma = Enigma::new(
            vec![RotorWiring::Iii, RotorWiring::I, RotorWiring::Ii],
            ReflectorWiring::B,
        );
        println!("{:?}", enigma);
    }
}
