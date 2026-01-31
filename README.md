## GojoSetup

Gojo Programlama Dili - v0.1.0
"Güvenmediğin Kodu Derleme"

Bu proje, GojoSetup adında bir kurulum ve çalışma ortamı sağlar. Rust tabanlı bir runtime ve çeşitli betik dosyaları içerir.

### Proje Yapısı

- `gojo.bat`, `kurulum.bat`: Windows için kurulum ve başlatma betikleri
- `ornek.gj`: Örnek Gojo dosyası
- `bin/`: Derlenmiş dosyalar veya yardımcı araçlar
- `runtime/`: Rust tabanlı çalışma zamanı
    - `Cargo.toml`: Rust proje yapılandırması
    - `src/`: Kaynak kodları

### Kurulum

1. Rust yüklü değilse [Rust kurulumu](https://www.rust-lang.org/tools/install) yapın.
2. `runtime` klasöründe aşağıdaki komut ile derleyin:
   ```sh
   cargo build --release
   ```
3. Windows için `gojo.bat` veya `kurulum.bat` dosyasını çalıştırın.

### Kullanım

Örnek dosya ile çalışmak için:
```sh
gojo.bat ornek.gj
```

veya doğrudan Rust binary ile:
```sh
runtime\target\release\gojo.exe ornek.gj
```

### Katkı

Pull request ve issue açabilirsiniz.

---

### Orijinal Rehber ve Dil Özellikleri

...existing code...