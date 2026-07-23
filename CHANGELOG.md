# Changelog

## [WIP] UI/UX Redesign — Save Point

> ⚠️ Это промежуточный коммит. Не все задачи выполнены.

### Что сделано

- **Удалён Wave визуализатор** — оставлены только BARS и MIRROR (оба CAVA-стиль)
- **Catppuccin Mocha палитра** — все hardcoded RGB заменены на именованные константы (ROSE, BLUE, TEAL, RED, MAUVE, etc.)
- **Music Deck переписан:**
  - Прогресс-бар с таймстампами (`02:34 ━━━━━━━━━━━ 05:12`)
  - Строка состояния снизу (SPACE pause, M mute, V vis, Q quit)
  - Пропорции: header 2 + track 2 + viz flex + progress 1 + status 1
  - Compact mode при маленьком терминале
- **mpv diagnostics** — stderr теперь pipe'ится, при ошибке mpv показывается его сообщение
- **Визуализаторы** — BARS и MIRROR используют Catppuccin цвета (TEAL→GREEN→YELLOW→RED)
- **Progress bar** — `━` заполненная часть, `─` остаток, красный при >85%

### Что ещё НЕ сделано (TODO)

- [ ] Home screen也需要 обновить до Catppuccin
- [ ] List/selection screen — улучшить пропорции и стили
- [ ] Notice и Input screen — обновить стили
- [ ] Settings screen — добавить визуальный интерфейс
- [ ] Более плавная анимация визуализатора (smoothing)
- [ ] Горизонтальный спектр (как ncmpcpp) — может вернуть как 3-й стиль
- [ ] Responsive layout для очень широких/узких терминалов
- [ ] Тесты на новый layout
- [ ] CI/CD workflow для автоматических релизов

### Изменённые файлы

| Файл | Описание |
|------|----------|
| `src/config.rs` | Убран `VisualizerStyle::Wave` |
| `src/tui.rs` | Catppuccin palette, новый music deck, progress bar |
| `src/player.rs` | stderr capture для диагностики ошибок mpv |
| `src/mpv_ipc.rs` | Добавлен `duration()` для progress bar |
