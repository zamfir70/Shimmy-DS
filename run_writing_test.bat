@echo off
echo 🚀 SHIMMY-DS WRITING TEST LAUNCHER
echo ================================

echo.
echo 📋 Checking if Shimmy-DS is running...
curl -s http://127.0.0.1:11435/v1/models >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ Shimmy-DS server not running
    echo.
    echo 🔧 Starting Shimmy-DS server...
    echo Please wait for server to start...
    start "Shimmy-DS Server" cmd /c "target\release\shimmy.exe serve"
    echo.
    echo ⏳ Waiting 10 seconds for server startup...
    timeout /t 10 /nobreak >nul
) else (
    echo ✅ Shimmy-DS server is running
)

echo.
echo 🧠 Running narrative intelligence writing test...
echo.

python examples\quick_writing_test.py

echo.
echo Press any key to run the full writing demonstration...
pause >nul

echo.
echo 📚 Running complete writing demonstration...
python examples\shimmy_writing_demo.py

echo.
echo ✨ All tests complete!
pause