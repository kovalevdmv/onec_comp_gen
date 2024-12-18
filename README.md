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
Так же готовая сборка для текущей версии в разделе "релизы"
## Использование
Скопировать собранный исполняемый файл из каталога `onec_comp_gen\app_tauri\src-tauri\target\release` в каталог `onec_comp_gen\source` и запустить

[Статья на infostart.ru](https://infostart.ru/1c/2252892/) [![Infostart](https://infostart.ru/bitrix/templates/sandbox_empty/assets/tpl/abo/img/logo.svg)](https://infostart.ru/1c/2252892/ "Статья на infostart.ru")

[Видео инструкция для конструктора](https://t.me/FastAbout1s/69)

[Общая методика разработки внешних компонент по технологи native api (видео)](https://t.me/FastAbout1s/51)
