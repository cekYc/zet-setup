@echo off
title Zet Lang Kurulum Sihirbazi
color 0A

echo ==========================================
echo      ZET LANG KURULUM SIHIRBAZI
echo         Zero Trust Language v0.2
echo ==========================================
echo.

REM --- 1. RUST KONTROLÜ (Zet'in kalbi Rust'tir) ---
echo [1/5] Rust derleyicisi kontrol ediliyor...
cargo --version >nul 2>&1
if %errorLevel% neq 0 (
    color 0C
    echo.
    echo [KRITIK HATA] Bilgisayarinizda Rust -Cargo- yuklu degil!
    echo Zet, Rust altyapisini kullanir. Once Rust'i kurmalisiniz.
    echo.
    echo Rust yukleme sayfasi aciliyor...
    timeout /t 3 >nul
    start https://rustup.rs/
    echo.
    echo Rust'i kurduktan sonra lutfen bu kuruluma tekrar tiklayin.
    pause
    exit
)
echo     - Rust bulundu. Devam ediliyor...
echo.

REM --- 2. YÖNETİCİ İZNİ KONTROLÜ ---
net session >nul 2>&1
if %errorLevel% neq 0 (
    color 0C
    echo [HATA] Yonetici izni gerekli!
    echo Lutfen bu dosyaya Sag Tiklayip 'Yonetici Olarak Calistir' deyin.
    pause
    exit
)

REM --- 3. KAYNAK VE HEDEF ---
set "SOURCE_DIR=%~dp0"
set "TARGET_DIR=C:\Zet"

echo [2/5] Eski kurulumlar temizleniyor...
if exist "%TARGET_DIR%" rmdir /S /Q "%TARGET_DIR%"
mkdir "%TARGET_DIR%"

echo [3/5] Dosyalar kopyalaniyor...
xcopy /E /I /Y "%SOURCE_DIR%bin" "%TARGET_DIR%\bin" >nul
xcopy /E /I /Y "%SOURCE_DIR%runtime" "%TARGET_DIR%\runtime" >nul
copy /Y "%SOURCE_DIR%zet.bat" "%TARGET_DIR%\" >nul

echo [4/5] Sistem yoluna (PATH) ekleniyor...
powershell -Command "if (-not ([Environment]::GetEnvironmentVariable('Path','Machine') -like '*C:\Zet*')) { [Environment]::SetEnvironmentVariable('Path', [Environment]::GetEnvironmentVariable('Path','Machine') + ';C:\Zet', 'Machine') }"

echo [5/5] Tamamlaniyor...
echo.
echo ==========================================
echo      KURULUM BASARIYLA TAMAMLANDI!
echo ==========================================
echo.
echo Test etmek icin yeni bir terminal acip su komutu yazin:
echo zet ornek.zt
echo.
pause