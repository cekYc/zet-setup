@echo off
setlocal
color 0A

REM --- EVRENSEL GOJO BAŞLATICI ---
set "INSTALL_DIR=%~dp0"
set "RUNTIME_DIR=%INSTALL_DIR%runtime"

REM Eğer kullanıcı dosya sürüklemediyse, elle sorsun (Çift tıklama modu)
if "%~1"=="" (
    echo.
    echo [BILGI] Gojo Motoru Calisiyor...
    echo Lutfen calistirmak istediginiz .gj dosyasini buraya surukleyin
    echo ve ENTER tusuna basin.
    echo.
    set /p "USER_FILE=> "
) else (
    set "USER_FILE=%~f1"
)

REM Dosya hala yoksa kapat
if "%USER_FILE%"=="" goto :eof

REM Kullanıcının kodunu motora taşı
echo Kopyalaniyor...
copy /Y "%USER_FILE%" "%RUNTIME_DIR%\run.gojo" >nul

REM Motorun içine gir ve çalıştır
pushd "%RUNTIME_DIR%"
echo Derleniyor...
echo ---------------------------------------------------
"%INSTALL_DIR%bin\gojo-compiler.exe" run.gojo
echo ---------------------------------------------------

REM İş bitince hemen kapanmasın, sonucu görelim
echo.
echo Cikis yapmak icin bir tusa basin...
pause >nul
popd
endlocal