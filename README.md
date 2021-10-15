# Rust Programming Cookbook Çalışma Alanım

Bir süre önce aldığım [Rust Programming Cookbook](https://amzn.to/3Bw5ysw) kitabını çalışmak için açtığım bir çalışma alanıdır. İş hayatında sürekli .Net platformunda ve C# ile kod geliştirdiğimden Rust ile yazma alışkanlığım son derece zayıf. Önceden çalıştığım pek çok konuyu unutmuş durumdayım. Kitabın anlattığı kodları copy-paste yapmadan bakarak yazmayı, bu sayede parmaklarımı rust sözdizimine alıştırmayı ve kısa notlar tutarak önceden baktığım konuları biraz daha özümsemeyi amaçlıyorum.

## Ön Hazırlıklar

Her şeyden önce çalışmalarımı Windows 10 tabanlı bir sistemde yapmaktayım. Rust'ın kurulumu için kitabın da önerdiği üzere [https://rustup.rs](https://rustup.rs) adresine gidip indirdiğim installer'ı çalıştırdım. Kurulumun başarılı olup olmadığını görmek için komut satırından _rustc --version_ komutunu çalıştırdım.

![./assets/screenshot_01.png](./assets/screenshot_01.png)

Geliştirmeler için Visual Studio Code kullanacağım. Rust eklentisi için command-line interface'i açıp(Ctrl+P) sonrasında _ext install rust-lang.rust_ komutunu çalıştırmak yeterli.

![./assets/screenshot_02.png](./assets/screenshot_02.png)

## day01

Kitabın birinci bölümüne ait çalışmalar.

### Komut Satırı ile Temel İşlemler

```bash
# Terminalden aşağıdaki komut çalıştılır ve ilk proje oluşturulur
cargo new wonderful-world

# Örneği çalıştırmak içinse (Tabi kodlama yaptıktan sonra :) )
cd wonderful-world
cargo run
```

![./assets/screenshot_03.png](./assets/screenshot_03.png)

### Veri Türleri

```bash
# Bu sefer bir kütüphane oluşturuyoruz
cargo new data-types --lib
cd data-types
# test fonksiyonları üstünden ilerliyoruz
cargo test
```

![./assets/screenshot_04.png](./assets/screenshot_04.png)

### Karar Yapıları ve Döngüler

```bash
cargo new flows --lib
cd flows
cargo test
```

![./assets/screenshot_05.png](./assets/screenshot_05.png)

### Sandık(Crate) ile Çalışmak

Fonksiyon içeren bir modül oluşturup diğerinden kullanılması. Tipik yürütücü uygulama ve referans olarak kullandığı kütüphane senaryosu.

```bash
# Kütüphane oluşturulması
cargo new rust-matlib --lib

# ana uygulamanın oluşturulması
cargo new calculator

# ana uygulamada bir modül oluşturulması(klasör içinde mod isimli rust dosyası da oluşturulur)
cd calculator
mkdir basic
cd basic
touch mod.rs

# rust-mathlib testlerini çalıştırmak için, o klasörde,
cargo test

# calculator'u çalıştırmak için o klasörde
cargo run
```

calculator'un rust-matlib'i kullanabilmesi için toml dosyasında gerekli dependency eklenmiştir. Ayrıca calculator projesinde basic isimli bir klasör vardır. Burası basic isimli modülü temsil eder.

![./assets/screenshot_06.png](./assets/screenshot_06.png)

### Dokümantasyon Eklenmesi

Çok basit anlamda markdown stili kullanılarak yardım dokümanı oluşturulabilyor. /// veya ///! ile bu işlemler gerçekleştirilebiliyor.

```bash
cd rust-matlib
cargo doc
```

Sonrasında silver-eureka/day01/rust-matlib/target/doc/rust_matlib/index.html sayfası açılarak Help içeriği görülebilir.

![./assets/screenshot_07.png](./assets/screenshot_07.png)

### Trait Kullanımı

Abstract sınıflara benzetebileceğimiz trait enstrümanı ile aynı davranışı birden fazla struct'ın kullanması mümkün olabilir. Örnekte bir key:value çiftinin okunması, yazılması ile ilgili davranış tanımlamaları yapılmakta ve uygulanmakta.

```bash
cargo new traits --lib
```

![./assets/screenshot_08.png](./assets/screenshot_08.png)

### Sequence Tipleri

Tek bir veri türünden oluşabilen ve boyutu değiştirilemeyen dizi, farklı türden veriler barındırabilen ama boyutu değiştirilemeyen Tuple ve generic tipleri kullanıp dinamik olarak boyutu ayarlanabilen Vector tiplerinin kullanım örnekleri.

```bash
cargo new sequences --lib
cd sequences
cargo test
```

![./assets/screenshot_09.png](./assets/screenshot_09.png)

## day02

Kitabın ikinci bölümüne ait çalışmalar.

### enum Kullanımı

Pek çok dilde olduğu gibi Rust tarafında da sayıları anlamlı şekilde isimlendirmek için enum türünden faydalanılıyor ama fazlası da olabilir. Enum sabiti özelliklerinde farklı türleri kullanmak, enum sabitine fonksiyonellik kazandırmak, pattern matching ifadelerinde ele almak vs

```bash
mkdir day02
cd day02
cargo new using_enums --lib
cd using_enums
cargo test
```
![./assets/screenshot_10.png](./assets/screenshot_10.png)

### Null Olmayan Güzel Bir Dünya

Rust dilinde fonksiyonel dillerin bir geleneği olarak her girdinin karşılığında anlamlı bir çıktının üretilmesi amaçlanır. null tipi yoktur. Bunun yerine Option< T> ve Result<T,E> tipleri kullanılır. Bir hata oluşma ihtimali varsa Result enum sabiti tercih edilir. Bu enum türleri aşağıdaki gibidir.

```rust
pub enum Option<T> {
    Some(T),
    None,
}

pub enum Result<T,E>{
    Ok(T),
    Err(E),
}
```

Örnekte Option tipinin farklı kullanım şekilleri ele alınmakta.

```bash
cargo new no_null --lib
cd no_null
cargo test
```

![./assets/screenshot_11.png](./assets/screenshot_11.png)

### Pattern Matching için Verimli Kullanım Senaryoları

Pattern matching kabiliyetinin enum'larla sınırlı olmadığına dair örnek kodlar yer alıyor. _(literal, tuple, String(heap allocation sebebiyle diğer literallerden farklı ama kullanım şekli aynı) türleri ile kullanım, destructuring ve guard mevzusu)_

```bash
cargo new amazing_pattern_matching
cd amazing_pattern_matching
cargo run
```

![./assets/screenshot_12.png](./assets/screenshot_12.png)

### Basit Bir Linked List Oluşturma

Kitabın bir sonraki örneğinde Iterator deseni kullanılan bir örnek yer alıyor. Ancak örnek içerisinde generic türden bir bağlı liste_(Linked List)_ kullanılmakta. Önce bu veri yapısını inşa etmeyi öğrenmek gerekiyor. Bu kodda generic verisyona gitmeden bir sürecin sıralı loglarının bağlı liste olarak tutulduğu basit kurgu söz konusu. Her log bilgisi Node isimli bir struct ile temsil ediliyor. ProcessLog isimli veri yapısı Node örneklerini ardışıl olarak tutan bir bağlı liste veri yapısını temsil ediyor.

```bash
cargo new linked_list --lib
cd linked_list
cargo test
```