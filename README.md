# Конструктор внешних компонент для 1С C++/Rust

Конструктор упрощает процесс разработки за счет удобного добавления всех нужных функций и процедур в графическом режиме, с указанием их параметров и типов параметров. На выходе приложение генерирует коготовый код на С++ и Rust и позволяет сразу приступить к реализации, без настройку API компоненты вручную.

Так же конструктор автоматизирует интеграцию библиотек на Rust в компоненту на С++. Таким образом на С++ будет описана только базовая логика взаимодействия компоненты и платформы, а вся основная логика будет реализована в библиотеке на Rust. Этот подход выбран так как реализовать native API на С++ проще и так же взаимодействовать с библиотекой на Rust через FFI интерфейс не сложно и мало отличается от обычного вызова методов из той же программы на С++.

## Сборка
1. Установить nodejs и среду для сборки rust
2. Из каталога `onec_comp_gen\app_tauri\src-tauri` выполнить
```
npm run tauri build
```
Собранные файлы в каталоге `onec_comp_gen\app_tauri\src-tauri\target\release` 
## Использование
Скопировать собранный исполняемый файл из каталога `onec_comp_gen\app_tauri\src-tauri\target\release` в каталог `onec_comp_gen\source` и запустить

Статья на infostart.ru https://infostart.ru/1c/2252892/

Видео инструкция https://t.me/FastAbout1s/69
