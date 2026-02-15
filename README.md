🛠️ zed-cmake 🚀

✨ CMake support for Zed editor ✨
🌈 Сделаем твою разработку на C++ в Zed просто легендарной! 🌈
🌟 Основные фишки (Features) 🌟

    🎨 Syntax highlighting — Красивая подсветка через tree-sitter-cmake 🌈

    🧠 LSP integration — Умный мозг через neocmakelsp-fast 🤖💨

        🎯 Intelligent code completion — Дописывает код за тебя (почти как магия!) ✨

        🚫 Real-time diagnostics — Видит ошибки раньше, чем ты нажмешь "Build" 🔍

        🏃 Go to definition — Прыгай по коду как ниндзя 🥷

        📖 Hover documentation — Подсказки при наведении 📋

        💅 Code formatting — Твой код всегда будет выглядеть на все 💯

        🏷️ Rename support — Меняй имена без боли и страданий ✏️

    🏗️ Build tasks — Полный цикл: Configure ⚙️, Build 🔨, Clean 🧹, Install 📦, Run 🏃‍♂️

🛠️ Что нужно установить (Requirements) 🛠️

Тебе понадобится мощный neocmakelsp-fast! 🦾
Bash

# Собираем из исходников (для суровых профи) 💪
cargo install --git https://github.com/NikitolProject/neocmakelsp-fast

# Или просто скачай готовый бинарник тут 👇
# https://github.com/NikitolProject/neocmakelsp-fast/releases 📦

📥 Как установить (Installation) 📥
🏢 Через Zed Extensions

Просто вбей в поиске "CMake" внутри Zed и жми кнопку Install! 🖱️💥
👨‍💻 Режим разработчика (Dev Extension)
Bash

git clone https://github.com/NikitolProject/zed-cmake 📂
# В самом Zed: Extensions → Install Dev Extension → выбери папку zed-cmake 📁✨

⚙️ Настройка (Configuration) ⚙️

Закинь этот сочный конфиг в свой ~/.config/zed/settings.json 📝👇:
JSON

{
  "lsp": {
    "cmake": {
      "initialization_options": {
        "format": { "enable": true },
        "lint": { "enable": true },
        "scan_cmake_in_package": false
      }
    }
  }
}

📜 Лицензия (License) 📜

📜 MIT — Свобода во всем! 🕊️🔓
