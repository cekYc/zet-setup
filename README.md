# ZET PROGRAMLAMA DİLİ — v0.2.0

> **"Zero Trust — Güvenmediğin Kodu Derleme"**

---

## Nasıl Kurulur?

1. `kurulum.bat` dosyasına sağ tıklayıp **"Yönetici Olarak Çalıştır"** deyin.
2. Kurulum bitince siyah pencereleri kapatın.
3. Yeni bir terminal (CMD) açın.

## Nasıl Çalıştırılır?

Terminalde dosyayı çalıştıracağınız klasöre gidip şu komutu yazmanız yeterlidir:

```
C:\Zet\zet.bat ornek.zt
```

---

## Dil Rehberi & Kütüphaneler

### 1. Temel Komutlar

```
let x = 10              -> Değişken tanımlama
x = 20                  -> Değer atama
if x > 10 { ... }       -> Koşul
for i in 0..10 { ... }  -> Döngü
while x < 50 { ... }    -> Döngü
```

### 2. Fonksiyon Tipleri

Zet'te iki tip fonksiyon vardır:

**A) Deterministik:** Saf matematik, yan etkisi yok. Daima aynı girdi = aynı çıktı.

```
deterministic fn topla(a: i64, b: i64) -> i64 {
    return a + b
}
```

**B) Nondeterministik:** I/O, network, konsol gibi dış dünya ile etkileşim.

```
nondeterministic fn main(args: Untrusted) -> Void {
    call DB.log("Merhaba")
}
```

### 3. Fonksiyon Çağırma (call vs normal)

**A) Normal Çağrı:** Deterministik/saf fonksiyonlar için.

```
let sonuc = topla(10, 20)
```

**B) `call` ile Çağrı:** Nondeterministik (I/O) işlemler için.

```
let veri = call HTTP.get("...")
let isim = call Console.read("Adin?")
// 'call' komutu, işlem bitene kadar programı o satırda bekletir.
// Deterministik fonksiyona 'call' kullanırsan derleyici hata verir.
```

### 4. Güvenlik (validate) — Zero Trust

Dışarıdan gelen her veri (Klavye, İnternet, args) **"Untrusted"** (Güvensiz) kabul edilir.
Bunları `validate` bloğuna almadan kullanamazsınız.
Derleyici bunu derleme zamanında zorlar — çalışma zamanı değil!

```
validate degisken {
    success: {
        // Burada artık güvenli (String), kullanabilirsin
    }
}
```

### 5. Eşzamanlılık (Concurrency) — Structured Concurrency

Zet'te paralel işler `scope` (kapsam) içinde yapılır.
`spawn` **SADECE** scope içinde kullanılabilir — dışarıda derleyici hata verir.

```
scope Isim {
    spawn DB.log("Arka planda calis")
    spawn fonksiyon_adi()
}
// Scope kapanmadan program ilerlemez, tüm işçilerin bitmesi beklenir.
// Zombie process oluşması yapısal olarak imkansızdır.
```

### 6. Yerleşik Kütüphaneler (stdlib)

| Kütüphane | Kullanım | Açıklama |
|-----------|----------|----------|
| **DB** | `call DB.log("Mesaj")` | Ekrana renkli log basar |
| **Console** | `call Console.read("Soru?")` | Kullanıcıdan veri okur (Untrusted döner) |
| **Util** | `call Util.now()` | Şu anki zaman (ms cinsinden) |
| **Util** | `call Util.to_int("123")` | Yazıyı sayıya çevirir |
| **HTTP** | `call HTTP.get("url")` | Siteye istek atar, cevabı döner (Untrusted döner) |
| **JSON** | `json(veri, "key")` | JSON içinden veri çeker (call gerekmez) |

---

*Hazırlayan: Ceky*
