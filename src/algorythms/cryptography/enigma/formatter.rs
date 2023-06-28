use super::{Enigma, Reflector, Rotor};

use ascii_table::{Align, AsciiTable};

use std::fmt::{Debug, Formatter};

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

        let mut ascii_table = AsciiTable::default();
        ascii_table
            .column(0)
            .set_header("Reflector (A)")
            .set_align(Align::Right);
        ascii_table
            .column(1)
            .set_header("Rotor (III)")
            .set_align(Align::Right);
        ascii_table.print(vec![vec![0, 1, 2, 3], vec![0, 1, 2, 3]]);

        // self.rotors
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::{
        super::{ReflectorWiring, RotorWiring},
        Enigma,
    };

    #[test]
    fn enigma_formatting() {
        let enigma = Enigma::new(
            vec![RotorWiring::Iii, RotorWiring::I, RotorWiring::Ii],
            ReflectorWiring::B,
        );
        println!("{:?}", enigma);
    }
}
