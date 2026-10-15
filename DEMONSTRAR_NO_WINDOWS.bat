@echo off
setlocal
cd /d "%~dp0"
echo ========================================================
echo CONECTASTORE - DEMONSTRACAO E VALIDACAO REAL
 echo ========================================================
cargo --version
if errorlevel 1 (
  echo Rust/Cargo nao encontrado. Instale em https://rustup.rs/
  pause
  exit /b 1
)
echo.
echo [1/4] Verificando compilacao
cargo check --all-targets
if errorlevel 1 goto error
 echo.
echo [2/4] Executando demonstracao
cargo run -- demo
if errorlevel 1 goto error
 echo.
echo [3/4] Executando testes
cargo test --all-targets
if errorlevel 1 goto error
 echo.
echo [4/4] Medindo desempenho real
cargo run --release -- benchmark
if errorlevel 1 goto error
 echo.
echo CONCLUIDO. Registre os tempos reais no relatorio.
pause
exit /b 0
:error
 echo.
echo ERRO: verifique a mensagem acima antes de apresentar a entrega.
pause
exit /b 1
