use std::fmt::Debug;

// trait bounds sözdiziminin kullanılmasına dair bir örnek

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_summary_behavior_works_test() {
        let entry=BlogEntry{
            title:String::from("Monolitik Sistemlerde Teknik Borçlanma ile Mücadele"),
            author:String::from("Burak Selim Şenyurt"),
            content:String::from("Yazılımcı olmanın bir gerçeği de üretim ortamından gelen problemler ile uğraşmaktır belki de. Çalışmakta olduğumuz sistemlerin giderek büyümesi, iş kurallarının zamanla karmaşıklaşması, nasıl yapılır nereye bakılır bilgisinin ayrılan iş gücü nedeniyle eksilmesi, entegrasyon noktalarının çoğalması ve daha birçok sebepten ötürü bu kaçınılmazdır. Her birimiz yazılım yaşam süresi boyunca farklı tipte mimariler üzerinde çalışırız. Örneğin 2021 yılının ilk çeyreğinde hazırladığım ve yedi yüzden fazla kişinin katıldığı “Teknik Borç Farkındalık Anketi” isimli çalışmanın sonuçlarına göre beşimizden dördünün katmanlı mimari olarak adlandırdığımız monolitik sistemlerde görev aldığını söyleyebiliriz. ")
        };
        let summary_of_entry = entry.get_summary();
        assert_eq!(summary_of_entry, "Monolitik Sistemlerde Teknik Borçlanma ile Mücadele -> Burak Selim Şenyurt -> Yazılımcı olmanın bir gerçeği de üretim ...");

        let notify = Notification {
            owner: String::from("Boba the Fat"),
            note: String::from("The Empire Strikes Back"),
            is_argent: true,
        };
        assert_eq!(
            notify.get_summary(),
            "Boba the Fat -> The Empire Strikes Back"
        );
    }

    #[test]
    fn should_collect_summaries_works_test() {
        let entry=BlogEntry{
            title:String::from("Monolitik Sistemlerde Teknik Borçlanma ile Mücadele"),
            author:String::from("Burak Selim Şenyurt"),
            content:String::from("Yazılımcı olmanın bir gerçeği de üretim ortamından gelen problemler ile uğraşmaktır belki de. Çalışmakta olduğumuz sistemlerin giderek büyümesi, iş kurallarının zamanla karmaşıklaşması, nasıl yapılır nereye bakılır bilgisinin ayrılan iş gücü nedeniyle eksilmesi, entegrasyon noktalarının çoğalması ve daha birçok sebepten ötürü bu kaçınılmazdır. Her birimiz yazılım yaşam süresi boyunca farklı tipte mimariler üzerinde çalışırız. Örneğin 2021 yılının ilk çeyreğinde hazırladığım ve yedi yüzden fazla kişinin katıldığı “Teknik Borç Farkındalık Anketi” isimli çalışmanın sonuçlarına göre beşimizden dördünün katmanlı mimari olarak adlandırdığımız monolitik sistemlerde görev aldığını söyleyebiliriz. ")
        };
        let notify = Notification {
            owner: String::from("Boba the Fat"),
            note: String::from("The Empire Strikes Back"),
            is_argent: true,
        };
        let result: Vec<String> = collect_summaries(entry, notify);
        assert_eq!(result[0],"Monolitik Sistemlerde Teknik Borçlanma ile Mücadele -> Burak Selim Şenyurt -> Yazılımcı olmanın bir gerçeği de üretim ...");
        assert_eq!(result[1], "Boba the Fat -> The Empire Strikes Back");
    }

    // #3
    #[test]
    fn should_debug_trait_works_test() {
        let notify = Notification {
            owner: String::from("Boba the Fat"),
            note: String::from("The Empire Strikes Back"),
            is_argent: true,
        };
        // #3
        // log fonksiyonuna notification veri yapısı türünden bir değişken gönderebilmemiz için Debug trait'inin uygulanması gerekir
        assert_eq!(log(notify), "Notification { owner: \"Boba the Fat\", note: \"The Empire Strikes Back\", is_argent: true }");
    }

    // #4
    #[test]
    fn should_loggable_works_for_notification() {
        let notify = Notification {
            owner: String::from("Boba the Fat"),
            note: String::from("The Empire Strikes Back"),
            is_argent: true,
        };
        assert_eq!(notify.log(), "Notification { owner: \"Boba the Fat\", note: \"The Empire Strikes Back\", is_argent: true }");
    }
}

// #4 Şimdi aşağıdaki trait tanımına bakalım
// trait bound yazılışı kullanılan interface, Debug ve Sized isimli Trait'leri de devralır
pub trait Loggable: Debug + Sized {
    fn log(self) -> String {
        format!("{:?}", &self)
    }
}

// #4 Şu andaNotification veri modelini Loggable arayüzü ile donattık. Yani bir Notification değişkeni üstünden
// Debug ve Sized türevli Loggable trait'inin içerdiği log fonksiyonunu çağırabileceğiz
impl Loggable for Notification {}

// #3
// Şimdi aşağıdaki Trait'i göz önüne alalım
// T tipinin Rust sisteminde önceden tanımlı olan Debug davranışını sergileyebilir olması bekleniyor
// Bu sebeple Notification sınıfına türetme usülü bir bildirimi eklenecek.
pub fn log<T: Debug>(item: T) -> String {
    format!("{:?}", item)
}

// #2 Üstteki fonksiyonu trait bounds formatında ve where koşulunu kullanarak aşağıdaki gibi yazabiliriz.
// item_a ve item_b farklı türden veri yapıları olsun ama her ikisi de Summary davranışını uygulamış olsun istiyoruz
pub fn collect_summaries<T, K>(item_a: T, item_b: K) -> Vec<String>
where
    T: Summary,
    K: Summary,
{
    let mut result: Vec<String> = vec![];
    result.push(item_a.get_summary());
    result.push(item_b.get_summary());
    result
}

// // #1
// // Şimdi Summary trait'ini parametre olarak alıp kullanan bir fonksiyon olduğunu düşünelim.
// // item_a ve item_b Summary trait'ini implemente eden veri yapıları olmalıdır
// pub fn collect_summaries(item_a: impl Summary, item_b: impl Summary) -> Vec<String> {
//     let mut result: Vec<String> = vec![];
//     result.push(item_a.get_summary());
//     result.push(item_b.get_summary());
//     result
// }

// Önce kendi trait tipimizi oluşturalım
// Herhangi bir tip içindeki bilgileri kullanarak özet bir ifade veren davranış tanımı için kullanacağımızı düşünelim
pub trait Summary {
    fn get_summary(&self) -> String;
}

// Örnek bir veri modeli (Bir blog girdisini temsil ediyor)
pub struct BlogEntry {
    pub title: String,
    pub author: String,
    pub content: String,
}

// Bir başka örnek veri modeli (Bir duyuru metnini temsil etsin)
#[derive(Debug)] // #3 kullanımının çalışması için eklenmesi gereken direktif
pub struct Notification {
    pub owner: String,
    pub note: String,
    pub is_argent: bool,
}

// Summary davranışını bu iki tip için uygulayalım
impl Summary for BlogEntry {
    fn get_summary(&self) -> String {
        let short_content: String = self.content.chars().take(40).collect();
        format!("{} -> {} -> {}...", self.title, self.author, short_content)
    }
}

impl Summary for Notification {
    fn get_summary(&self) -> String {
        format!("{} -> {}", self.owner, self.note)
    }
}
