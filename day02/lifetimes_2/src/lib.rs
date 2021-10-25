#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

// Örnekte standart sapma ve ortalamaya göre varyans(değişiklik) hesaplaması ile ilgili enstrümanlar kullanılıyor.
// Üzerinde çalışılacak sayı dizisini tutan bir struct veri modeli söz konusu.
// Önemli olan Struct'a generic lifetime parametresi geçirilmesi ve içerdeki sayı dizisinin de bu yaşam süresine göre değerlendirilmesi gerektiğinin derleyiciye söylenmiş olması.
pub struct Statistician<'a> {
    numbers: &'a [f64],
}

// new ile yeni bir Statistician nesnesi örneklemeyi kolaylaştıran fonksiyonu yazıyoruz.
// parametre olarak gelen sayı dizisinin lifetimes bilgisi ile birlikte geldiğine dikkat edelim.
impl<'a> Statistician<'a> {
    pub fn new(numbers: &'a [f64]) -> Option<Statistician> {
        if numbers.len() < 5 {
            // Beştan az sayı varsa None değerini döneceğiz
            None
        } else {
            Some(Statistician { numbers: numbers })
        }
    }
    // orta değer bulma fonksiyonu
    pub fn mean(&self) -> f64 {
        let sum: f64 = self.numbers.iter().sum();
        sum / self.numbers.len() as f64
    }
    // varyans hesaplama fonksiyonu
    pub fn variance(&self) -> f64 {
        let mean = self.mean();
        let total_sum_of_squares: f64 = self.numbers.iter().map(|n| (n - mean).powi(2)).sum();
        return total_sum_of_squares / self.numbers.len() as f64;
    }
    // standart sapma fonksiyonu
    pub fn std_deviation(&self) -> f64 {
        self.variance().sqrt()
    }

    // medyan değerini bulan fonksiyon
    pub fn median(&self) -> f64 {
        let mut _numbers = self.numbers.to_vec(); // veri yapısındaki numbers'ın bir klonu oluşturulup vektör dizisi haline getiriliyor
        _numbers.sort_by(|x, y| x.partial_cmp(y).unwrap()); // 
        let m = _numbers.len() / 2;
        if _numbers.len() % 2 == 0 {
            _numbers[m]
        } else {
            (_numbers[m] + _numbers[m - 1]) / 2.0
        }
    }
}
