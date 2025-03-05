struct AritmeticnoZaporedje {
    prvi_clen: i32,
    razlika: i32,
}

use AritmeticnoZaporedje as AZ;

impl AZ {
    fn new(a:i32, d:i32) -> AZ {
        return AZ {
            prvi_clen: a,
            razlika: d,
        };
    }
    fn next(self: AZ) -> i32 {
        self.prvi_clen + self.razlika
    }

    fn n_th(self: AZ, n: i32) -> i32 {
        self.prvi_clen + n * self.razlika
    }

    fn reset(&mut self) {
        self.trenutni = self.zacetek;
    }

    fn current(&self) -> i32 {
        self.trenutni
    }

    fn sum(&mut self, n: i32) -> i32 {
        let original = self.trenutni; // Shrani trenutno stanje
        let mut acc = 0;
        for _ in 0..n {
            acc += self.next();
        }
        self.trenutni = original; // Obnovi stanje zaporedja
        acc
}


fn main() {
    println!("Hello, world!");
}
