========================================================================
                      GOJO PROGRAMLAMA DİLİ - v0.1.0
                  	"Güvenmediğin Kodu Derleme"
========================================================================

[NASIL KURULUR?]
1. "kurulum.bat" dosyasina sag tiklayip "Yonetici Olarak Calistir" deyin.
2. Kurulum bitince siyah pencereleri kapatin.
3. Yeni bir terminal (CMD) acin.

[NASIL CALISTIRILIR?]
Terminalde dosyayı çalıştıracağınız dosyaya gidip şu komutu yazmaniz yeterlidir:
   gojo ornek.gj

------------------------------------------------------------------------
[DİL REHBERİ & KÜTÜPHANELER]

1. TEMEL KOMUTLAR
   let x = 10              -> Değişken tanımlama
   x = 20                  -> Değer atama
   if x > 10 { ... }       -> Koşul
   for i in 0..10 { ... }  -> Döngü
   while x < 50 { ... }    -> Döngü

2. FONKSIYON ÇAĞIRMA (CALL vs NORMAL)
   Gojo'da iki tip fonksiyon çağrısı vardır:
   
   A) Normal Çağrı: Matematiksel/Hızlı işler için.
      let sonuc = topla(10, 20)

   B) 'call' ile Çağrı: Bekleme gerektiren (Internet/Konsol) işler için.
      let veri = call HTTP.get("...")
      let isim = call Console.read("Adin?")
      // 'call' komutu, islem bitene kadar programi o satirda bekletir.

3. GÜVENLİK (VALIDATE)
   Dışarıdan gelen her veri (Klavye, Internet) "Untrusted" (Güvensiz) kabul edilir.
   Bunları "validate" bloguna almadan kullanamazsınız.

   validate degisken {
       success: {
           // Burada artık güvenli, kullanabilirsin
       }
   }

4. EŞZAMANLILIK (CONCURRENCY)
   Gojo'da işler "scope" (kapsam) içinde yapılır.
   
   scope Isim {
       spawn DB.log("Arka planda calis")  -> Yeni işçi (Thread)
       spawn fonksiyon_adi()              -> Fonksiyonu arka planda başlat
   } 
   // Scope kapanmadan program ilerlemez, tüm işçilerin bitmesi beklenir.

5. YERLEŞİK KÜTÜPHANELER (STDLIB)

   [DB] (Loglama)
   DB.log("Mesaj")         -> Ekrana renkli log basar.

   [Console] (Girdi/Çıktı - call gerekir)
   call Console.read("Soru?")   -> Kullanıcıdan veri okur (Untrusted döner).

   [Util] (Araçlar - call gerekir)
   call Util.now()              -> Şu anki zaman (ms cinsinden).
   call Util.to_int("123")      -> Yazıyı sayıya çevirir.

   [HTTP] (İnternet - call gerekir)
   call HTTP.get("url")         -> Siteye istek atar, cevabı döner.
   
   [JSON]
   json(veri, "key")       -> JSON içinden veri çeker (call gerekmez).

------------------------------------------------------------------------
Hazirlayan: Ceky