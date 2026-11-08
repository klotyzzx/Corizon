## Русский

Первый публичный предварительный релиз **Corizon Launcher** для Windows x64, основанный на Horizon.

### Добавлено и улучшено
- Локальные профили: сохранение никнеймов и запуск одиночной игры, LAN и серверов с отключённой проверкой аккаунтов.
- Название **Corizon Launcher**, оформление Corizon и исполняемый файл **Corizon.exe**.
- Проверенная нативная сборка Windows x64 со встроенной средой выполнения C/C++.
- Импорт собственных сборок игры через раздел «Версии».

### Убрано и исправлено
- Убрана обязательная проверка владения игрой для запуска с локальным профилем.
- Обычный локальный запуск больше не требует инициализации хранилища токенов Microsoft.
- Вход Microsoft сохранён как дополнительная возможность. Серверы с проверкой аккаунтов по-прежнему требуют подходящий авторизованный аккаунт.
- Отключено самообновление лаунчера из upstream Horizon, чтобы оно не заменяло Corizon.
- Исправлена проверка запуска Windows GUI-приложения при упаковке: процесс дожидаются до завершения.
- Добавлены русские и английские тексты для новых функций.

### Чистая поставка и первый запуск
Архив содержит только программу, лицензии, инструкцию, идентификатор сборки и контрольные суммы. В нём нет профилей разработчика, паролей, токенов, личных настроек, миров, логов или подготовленных ресурсов Minecraft.

1. Скачайте **Corizon-Launcher-Windows-x64.zip** из Assets.
2. Нажмите «Извлечь всё» и откройте **Corizon.exe**.
3. В «Локальных профилях» добавьте свой никнейм.
4. Установите нужную версию игры и подготовьте ресурсы из своей установленной копии Minecraft.

Лаунчер пока использует стандартные папки данных Horizon. На компьютере с существующей установкой он может увидеть ранее сохранённые данные; чистый архив не удаляет их. Ресурсы Minecraft не включены, их подготовка нужна отдельно. Это предварительный релиз: совместимость с каждым ПК не гарантируется.

**Оптимизация игры и заголовок игрового окна Corizon распространяются отдельно** в сборке Corizon Game. Они не встроены в этот лаунчер и не означают гарантированного прироста FPS. Для её установки распакуйте архив игры и выберите horizon-client.exe через «Версии» → «Импортировать сборку игры…».

### Проверено
- Тесты лаунчера, локальных профилей и интеграционные проверки Windows.
- Архитектура x64, запуск --version, создание и сохранение локального профиля.
- Совпадение SHA-256 исполняемого файла; состав релизного архива проверен по списку разрешённых файлов.
- Пользователь подтвердил работу лаунчера на своём Windows-компьютере.

---

## English

First public preview release of **Corizon Launcher** for Windows x64, based on Horizon.

### Added and improved
- Saved local profiles for single player, LAN and servers with account authentication disabled.
- **Corizon Launcher** branding and the **Corizon.exe** executable.
- Tested native Windows x64 build with the C/C++ runtime linked statically.
- Custom game-build import through the Versions page.

### Removed and fixed
- Removed the mandatory ownership gate for launches using local profiles.
- Ordinary local launches no longer initialize the Microsoft token store.
- Microsoft sign-in remains optional. Account-authenticated servers still require an appropriate signed-in account.
- Disabled launcher self-updates from upstream Horizon so they cannot replace Corizon.
- Fixed Windows GUI executable packaging checks by explicitly waiting for the process to exit.
- Added Russian and English copy for the new features.

### Clean package and first launch
The archive contains only the application, licenses, instructions, build identity and checksums. It contains no developer profiles, passwords, tokens, personal settings, worlds, logs or prepared Minecraft resources.

1. Download **Corizon-Launcher-Windows-x64.zip** from Assets.
2. Extract the entire archive and open **Corizon.exe**.
3. Add your nickname under Local profiles.
4. Install a game version and prepare resources from your own installed Minecraft copy.

The launcher currently retains Horizon's default data directories. On a computer with an existing installation it may discover previously saved data; this clean package does not erase it. Minecraft resources are not bundled and require separate preparation. This is a preview; compatibility with every PC has not been established.

**Game optimizations and the Corizon game-window title are distributed separately** in the Corizon Game build. They are not included in this launcher executable and do not promise an FPS increase. To install that build, extract its archive and select horizon-client.exe under Versions → Import a game build.

### Validation
- Windows launcher, local-profile and integration tests.
- x64 architecture, --version startup, local-profile creation and persistence.
- Executable SHA-256 verification and release archive allowlist inspection.
- The user confirmed the launcher works on their Windows computer.

Based on Horizon; original license and third-party notices are included.
