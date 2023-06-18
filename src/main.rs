use std::io::{self, Write};
use std::str::FromStr;

enum Islem {
    Toplama,
    Cikartma,
    Carpma,
    Bolme,
}

struct Hesaplama {
    islem: Islem,
    sayi1: Option<f64>,
    sayi2: f64,
}

impl Hesaplama {
    fn hesapla(&mut self, onceki_sonuc: Option<f64>) -> Option<f64> {
        let sayi1 = self.sayi1.unwrap_or(onceki_sonuc.unwrap_or(0.0));
        match self.islem {
            Islem::Toplama => Some(sayi1 + self.sayi2),
            Islem::Cikartma => Some(sayi1 - self.sayi2),
            Islem::Carpma => Some(sayi1 * self.sayi2),
            Islem::Bolme => {
                if self.sayi2 != 0.0 {
                    Some(sayi1 / self.sayi2)
                } else {
                    None
                }
            },
        }
    }
}

fn main() {
    let mut onceki_sonuc = None;

    loop {
        let mut islem_giris = String::new();
        let mut sayi1_giris = String::new();
        let mut sayi2_giris = String::new();

        print!("Bir işlem giriniz (Toplama, Cikartma, Carpma, Bolme) programı bitirmek için (Bitir): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut islem_giris).unwrap();

        // Trim the input and parse to an Operation.
        match Islem::from_str(islem_giris.trim()) {
            Ok(islem) => {
                print!("İlk sayıyı giriniz (ve ya önceki sonuçla devam etmek için enter a basın, {}): ", onceki_sonuc.unwrap_or(0.0));
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut sayi1_giris).unwrap();

                let sayi1: Option<f64> = if sayi1_giris.trim().is_empty() {
                    None
                } else {
                    Some(sayi1_giris.trim().parse::<f64>().unwrap())
                };

                print!("İkinci sayıyı giriniz :");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut sayi2_giris).unwrap();

                let sayi2 = sayi2_giris.trim().parse::<f64>().unwrap();

                let mut hesapla = Hesaplama { islem, sayi1, sayi2 };
                onceki_sonuc = hesapla.hesapla(onceki_sonuc);
                println!("Sonuç: {}", onceki_sonuc.unwrap());
            },
            Err(_) if islem_giris.trim().eq_ignore_ascii_case("Bitir") => {
                break;
            },
            Err(_) => {
                println!("Geçersiz işlem. Lütfen Toplama, Cıkartma, Carpma, Bolme ve ya Bitir giriniz.");
            },
        }
    }
}

impl FromStr for Islem {
    type Err = ();

    fn from_str(s: &str) -> Result<Islem, ()> {
        match s.to_lowercase().as_str() {
            "toplama" => Ok(Islem::Toplama),
            "cıkartma" => Ok(Islem::Cikartma),
            "carpma" => Ok(Islem::Carpma),
            "bolme" => Ok(Islem::Bolme),
            _ => Err(()),
        }
    }
}
