
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

struct DB;
impl DB {
    async fn log(msg: String) {
        println!("  {}[LOG] {}{}", GREEN, msg, RESET);
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

async fn input(prompt: String) -> Untrusted {
    print!("  {}[>] {}{}", CYAN, prompt, RESET);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer);
    Untrusted(buffer.trim().to_string())
}

async fn inputln(prompt: String) -> Untrusted {
    println!("  {}[>] {}{}", CYAN, prompt, RESET);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer);
    Untrusted(buffer.trim().to_string())
}

// TRAITLER (Inline optimize edildi)
trait ZetAdd<Rhs> { type Output; fn z_add(self, rhs: Rhs) -> Self::Output; }
impl ZetAdd<i64> for i64 { type Output = i64; #[inline(always)] fn z_add(self, rhs: i64) -> i64 { self + rhs } }
impl ZetAdd<String> for String { type Output = String; #[inline(always)] fn z_add(self, rhs: String) -> String { self + &rhs } }
impl<'a> ZetAdd<&'a str> for String { type Output = String; #[inline(always)] fn z_add(self, rhs: &'a str) -> String { self + rhs } }
impl ZetAdd<i64> for String { type Output = String; #[inline(always)] fn z_add(self, rhs: i64) -> String { format!("{}{}", self, rhs) } }
impl ZetAdd<bool> for String { type Output = String; #[inline(always)] fn z_add(self, rhs: bool) -> String { format!("{}{}", self, rhs) } }

trait ZetMul<Rhs> { type Output; fn z_mul(self, rhs: Rhs) -> Self::Output; }
impl ZetMul<i64> for i64 { type Output = i64; #[inline(always)] fn z_mul(self, rhs: i64) -> i64 { self * rhs } }
impl ZetMul<i64> for String { type Output = String; fn z_mul(self, rhs: i64) -> String { self.repeat(rhs as usize) } }
impl<'a> ZetMul<i64> for &'a str { type Output = String; fn z_mul(self, rhs: i64) -> String { self.repeat(rhs as usize) } }
async fn user_main() -> () {
    println!("{}", "Merhaba farkli klasorden!".to_string());
}

#[tokio::main] async fn main() {
    user_main().await;
}