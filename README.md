# Rust ile Komut Satırı Hesap Makinesi

Bu, Rust ile implemente edilmiş basit bir hesap makinesi programıdır. Hesap makinesi, komut satırı arayüzünde (CLI) çalışır ve kullanıcının temel aritmetik işlemleri - toplama, çıkartma, çarpma ve bölme - yapmasını sağlar. Hesap makinesi, birinci operand sağlanmadığında önceki işlemin sonucunu bir sonraki işlemde kullanabilme özelliğine sahiptir.

## Özellikler

- Temel aritmetik işlemler: toplama, çıkartma, çarpma ve bölme.
- Önceki işlemin sonucunu bir sonraki hesaplamada kullanabilme yeteneği.
- Geçersiz işlemlerin ve sıfıra bölme durumunun ele alınması.

## Başlangıç

Bu talimatlar, projenin bir kopyasını geliştirme ve test amaçlı olarak yerel makinenizde çalıştırmanızı sağlar.

### Gereklilikler

Makinenizde Rust'ın kurulu olması gerekmektedir. Rust'ı [resmi web sitesinden](https://www.rust-lang.org/tools/install) indirebilirsiniz.

### Kurulum

1. Depoyu klonlayın
    ```sh
    git clone https://github.com/Celil6p/Rust_Simple_Calculator.git
    ```

2. Klonlanan dizine geçin
    ```sh
    cd Rust_Simple_Calculator
    ```

3. Projeyi oluşturun
    ```sh
    cargo build
    ```

4. Programı çalıştırın
    ```sh
    cargo run
    ```

## Kullanım

Programı çalıştırdığınızda, bir işlem (toplama için "Toplama", Çıkartma için "Cikartma", çarpma için "Carpma", bölme için "Bolme") ve iki sayı isteyecektir.

Önceki işlemin sonucunu bir sonraki işlemin ilk sayısı olarak kullanmak isterseniz, program ilk sayıyı sorduğunda enter tuşuna basın.

Programdan çıkmak için, program bir işlem istediğinde "Bitir" girin.

## Lisans

Bu proje GNU General Public License 3.0 Lisansı altında lisanslanmıştır - detaylar için [LICENSE](LICENSE) dosyasına bakınız.

