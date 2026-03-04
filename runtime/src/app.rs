
#![allow(dead_code, unused_imports, unused_variables, unused_parens, unused_mut)]
use std::time::Duration;
use std::io::{self, Write};
use serde_json::Value;

const RESET: &str = "\x1b[0m";
const CYAN: &str = "\x1b[36m";   
const GREEN: &str = "\x1b[32m";  
const MAGENTA: &str = "\x1b[35m";
const YELLOW: &str = "\x1b[33m"; 
const BLUE: &str = "\x1b[34m";
const RED: &str = "\x1b[31m";

/// Zet v0.2 — Untrusted: Dış dünyadan gelen lekeli veri sarmalayıcısı.
/// Bu struct doğrudan String gibi kullanılamaz. validate() ile temizlenmelidir.
#[derive(Clone, Debug)]
struct Untrusted(String);

impl Untrusted {
    /// Veriyi doğrular. Boş veya sadece boşluktan oluşan girdi reddedilir.
    fn validate(self) -> Result<String, String> {
        let s = self.0.trim().to_string();
        if s.is_empty() {
            Err("Dogrulama basarisiz: bos girdi.".to_string())
        } else {
            Ok(s)
        }
    }
}

struct DB;
impl DB {
    async fn log<T: std::fmt::Display>(msg: T) { println!("  {}[DB] Log: {}{}", CYAN, msg, RESET); }
}

struct Console;
impl Console {
    async fn read(prompt: String) -> Untrusted {
        print!("  {}[Console] {}: {}", BLUE, prompt, RESET);
        io::stdout().flush().unwrap();
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        Untrusted(buffer.trim().to_string())
    }
}

struct Util;
impl Util {
    #[inline(always)]
    async fn to_int(s: String) -> i64 { s.trim().parse::<i64>().unwrap_or(0) }
    #[inline(always)]
    async fn now() -> i64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64
    }
}

struct HTTP;
impl HTTP {
    async fn get(url: String) -> Untrusted {
        let client = reqwest::Client::builder().user_agent("ZetLang/0.2").build().unwrap();
        match client.get(&url).send().await {
            Ok(res) => Untrusted(res.text().await.unwrap_or_else(|e| format!("Error: {}", e))),
            Err(e) => Untrusted(format!("Error: {}", e))
        }
    }
}

// TRAITLER (Inline optimize edildi)
trait ZetAdd<Rhs> { type Output; fn z_add(self, rhs: Rhs) -> Self::Output; }
impl ZetAdd<i64> for i64 { type Output = i64; #[inline(always)] fn z_add(self, rhs: i64) -> i64 { self + rhs } }
impl ZetAdd<String> for String { type Output = String; #[inline(always)] fn z_add(self, rhs: String) -> String { self + &rhs } }
impl<'a> ZetAdd<&'a str> for String { type Output = String; #[inline(always)] fn z_add(self, rhs: &'a str) -> String { self + rhs } }
impl ZetAdd<i64> for String { type Output = String; #[inline(always)] fn z_add(self, rhs: i64) -> String { format!("{}{}", self, rhs) } }

trait ZetMul<Rhs> { type Output; fn z_mul(self, rhs: Rhs) -> Self::Output; }
impl ZetMul<i64> for i64 { type Output = i64; #[inline(always)] fn z_mul(self, rhs: i64) -> i64 { self * rhs } }
impl ZetMul<i64> for String { type Output = String; fn z_mul(self, rhs: i64) -> String { self.repeat(rhs as usize) } }
impl<'a> ZetMul<i64> for &'a str { type Output = String; fn z_mul(self, rhs: i64) -> String { self.repeat(rhs as usize) } }
fn topla(a: i64, b: i64) -> i64 {
    return (a + b);
}

fn fib(n: i64) -> i64 {
    if (n <= 1) {
        return n;
    }
    return (fib((n - 1)) + fib((n - 2)));
}

async fn veri_getir() -> () {
    let mut url = "https://jsonplaceholder.typicode.com/todos/1".to_string();
    DB::log("Istek atiliyor...".to_string()).await;
    let mut veri = HTTP::get(url).await;
    match veri.validate() {
        Ok(veri) => {
            DB::log("Gelen Veri: ".to_string().z_add(veri)).await;
        }
        Err(_zet_err) => {
            eprintln!("  {}[VALIDATE FAIL] {}{}", RED, _zet_err, RESET);
        }
    }
}

async fn user_main(args: Untrusted) -> () {
    DB::log("Zet v0.2 baslatiliyor...".to_string()).await;
    match args.validate() {
        Ok(args) => {
            DB::log("Hosgeldin: ".to_string().z_add(args)).await;
        }
        Err(_zet_err) => {
            eprintln!("  {}[VALIDATE FAIL] {}{}", RED, _zet_err, RESET);
        }
    }
    let mut sayilar = vec![10, 20, 30, 40];
    let mut ilk = sayilar[(0 as usize)];
    DB::log("Dizi[0]: ".to_string().z_add(ilk)).await;
    {
    let mut i = 0;
    let _zet_end = 3;
    let _zet_step = 1;
    while (_zet_step > 0 && i < _zet_end) || (_zet_step < 0 && i > _zet_end) {
        let mut val = sayilar[(i as usize)];
        DB::log("Dongu Degeri: ".to_string().z_add(val)).await;
        i = i.z_add(_zet_step);
    }
    }
    // Scope: WebIslemleri
    {
        let mut _zet_handles: Vec<tokio::task::JoinHandle<()>> = Vec::new();
        _zet_handles.push(tokio::spawn(async move { veri_getir().await }));
        _zet_handles.push(tokio::spawn(async move { DB::log("Bu mesaj paralel yazilir!".to_string()).await }));
        for _h in _zet_handles { _h.await.ok(); }
    }
    let mut sonuc = topla(500, 500);
    DB::log("Toplama Sonucu: ".to_string().z_add(sonuc)).await;
    let mut f = fib(30);
    DB::log("fib(30) = ".to_string().z_add(f)).await;
}

#[tokio::main] async fn main() {
    let _zet_input = Untrusted(std::env::args().skip(1).collect::<Vec<_>>().join(" "));
    user_main(_zet_input).await;
}