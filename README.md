# GiG

Repository has [ukrainian :ukraine:](#gig-ukraine) and [english :uk:](#gig-uk) localization.

## GiG :ukraine:

**GiG** (.gitignore Generator) — це інструмент командного рядка для швидкого створення та керування `.gitignore` файлами у ваших проектах.

### Огляд

GiG автоматизує створення `.gitignore` файлів з попередньо визначеними правилами для різних технологій. Замість того, щоб вручну шукати та копіювати правила для кожної мови програмування, фреймворку чи інструменту, GiG генерує їх миттєво з можливістю вибору через інтерактивний інтерфейс або через аргументи командного рядка.

Інструмент підтримує понад 60 технологій, організованих у зручні категорії, та дозволяє додавати чи видаляти правила з існуючих `.gitignore` файлів без перезапису всього вмісту.

### Функції

- **Інтерактивний режим** — вибір технологій через зручний multiselect інтерфейс з категоризацією
- **Швидка генерація** — створення `.gitignore` файлів однією командою через аргументи CLI
- **Гнучке керування** — додавання та видалення правил з існуючих файлів
- **Автоматична оптимізація** — видалення дублікатів для чистоти файлу
- **Широка підтримка** — понад 60 технологій у 8 категоріях
- **Категоризація** — окремі multiselect для мов, фреймворків, IDE, ОС тощо
- **Валідація** — перевірка введених технологій з інформативними повідомленнями про помилки

### Використані технології

- **Rust** — швидка, надійна та безпечна мова системного програмування
- **Clap** — парсер аргументів командного рядка з derive-макросами
- **Inquire** — інтерактивні CLI запити з підтримкою multiselect

### Початок роботи

Щоб почати працювати з GiG, виконайте наступні кроки:

1. **Клонуйте репозиторій:**

   ```bash
   git clone https://github.com/NikitaBerezhnyj/GiG.git
   cd GiG
   ```

2. **Зберіть проект:**

   ```bash
   cargo build --release
   ```

3. **Встановіть бінарний файл:**
   ```bash
   cargo install --path .
   ```

### Використання

Після встановлення проєкту ви можете виконувати такі дії:

1. **Інтерактивна генерація** `.gitignore` (рекомендований спосіб):

   ```bash
   gig
   ```

   Вам буде запропоновано вибрати технології з кожної категорії через multiselect інтерфейс.

2. **Швидка генерація** через аргументи командного рядка:

   ```bash
   gig generate rust node vscode
   ```

   Створює новий `.gitignore` файл з правилами для Rust, Node.js та VS Code.

3. **Додавання правил** до існуючого `.gitignore`:

   - Через аргументи:
     ```bash
     gig add python django
     ```
   - В інтерактивному режимі:
     ```bash
     gig add --interactive
     ```

4. **Видалення правил** з існуючого `.gitignore`:
   - Через аргументи:
     ```bash
     gig remove react node
     ```
   - В інтерактивному режимі:
     ```bash
     gig remove --interactive
     ```

### Підтримувані технології

GiG підтримує широкий спектр технологій, організованих у категорії:

- **Мови програмування** — C, C++, C#, Java, Kotlin, Swift, Rust, Python, Go, Ruby, PHP
- **Фреймворки** — React, Angular, Vue, Svelte, Node, Next.js, Nuxt.js, Express, Django, Tauri, Electron, Spring, Ruby on Rails, Laravel, Flutter, Flask, ASP.NET
- **Ігрові двигуни** — Unity, Unreal Engine, Godot, Ren'Py
- **Інструменти розробника** — Docker, npm, pnpm, Yarn, Gradle, Maven, Webpack, Composer, pip
- **Бази даних** — MySQL, MongoDB, SQLite, Firebase
- **IDE та редактори** — VS Code, IntelliJ IDEA, Visual Studio, Eclipse, Android Studio, Xcode, Sublime Text, Atom, Vim
- **Операційні системи** — Windows, Linux, macOS
- **Інше** — SCSS, Sass, Less, Jupyter, Terraform

### Приклади використання

```bash
# Створення .gitignore для React проекту
gig generate react node npm vscode

# Додавання Python правил до існуючого файлу
gig add python pip

# Видалення застарілих правил
gig remove atom sublimetext

# Інтерактивний режим для вибору з усіх доступних технологій
gig
```

### Ліцензія та правила спільноти

- [License](LICENSE) — ліцензія проекту
- [Code of Conduct](CODE_OF_CONDUCT.md) — очікувана поведінка учасників
- [Contributing Guide](CONTRIBUTING.md) — як допомогти проекту
- [Security Policy](SECURITY.md) — повідомлення про проблеми безпеки

---

## GiG :uk:

**GiG** (.gitignore Generator) is a command-line tool for quickly creating and managing `.gitignore` files in your projects.

### Overview

GiG automates the creation of `.gitignore` files with predefined rules for various technologies. Instead of manually searching and copying rules for each programming language, framework, or tool, GiG generates them instantly with the ability to select through an interactive interface or command-line arguments.

The tool supports over 60 technologies organized into convenient categories and allows adding or removing rules from existing `.gitignore` files without overwriting the entire content.

### Features

- **Interactive Mode** — Select technologies through a convenient multiselect interface with categorization
- **Quick Generation** — Create `.gitignore` files with a single command via CLI arguments
- **Flexible Management** — Add and remove rules from existing files
- **Automatic Optimization** — Remove duplicates for file cleanliness
- **Wide Support** — Over 60 technologies across 8 categories
- **Categorization** — Separate multiselect for languages, frameworks, IDEs, OS, etc.
- **Validation** — Check entered technologies with informative error messages

### Technologies Used

- **Rust** — Fast, reliable, and memory-safe systems programming language
- **Clap** — Command-line argument parser with derive macros
- **Inquire** — Interactive CLI prompts with multiselect support

### Getting Started

To get started with GiG, follow these steps:

1. **Clone the repository:**

   ```bash
   git clone https://github.com/NikitaBerezhnyj/GiG.git
   cd GiG
   ```

2. **Build the project:**

   ```bash
   cargo build --release
   ```

3. **Install the binary:**
   ```bash
   cargo install --path .
   ```

### Usage

After installing the project, you can perform the following actions:

1. **Interactive generation** of `.gitignore` (recommended method):

   ```bash
   gig
   ```

   You will be prompted to select technologies from each category through a multiselect interface.

2. **Quick generation** via command-line arguments:

   ```bash
   gig generate rust node vscode
   ```

   Creates a new `.gitignore` file with rules for Rust, Node.js, and VS Code.

3. **Add rules** to an existing `.gitignore`:

   - Via arguments:
     ```bash
     gig add python django
     ```
   - In interactive mode:
     ```bash
     gig add --interactive
     ```

4. **Remove rules** from an existing `.gitignore`:
   - Via arguments:
     ```bash
     gig remove react node
     ```
   - In interactive mode:
     ```bash
     gig remove --interactive
     ```

### Supported Technologies

GiG supports a wide range of technologies organized into categories:

- **Programming Languages** — C, C++, C#, Java, Kotlin, Swift, Rust, Python, Go, Ruby, PHP
- **Frameworks** — React, Angular, Vue, Svelte, Node, Next.js, Nuxt.js, Express, Django, Tauri, Electron, Spring, Ruby on Rails, Laravel, Flutter, Flask, ASP.NET
- **Game Engines** — Unity, Unreal Engine, Godot, Ren'Py
- **Developer Tools** — Docker, npm, pnpm, Yarn, Gradle, Maven, Webpack, Composer, pip
- **Databases** — MySQL, MongoDB, SQLite, Firebase
- **IDEs & Editors** — VS Code, IntelliJ IDEA, Visual Studio, Eclipse, Android Studio, Xcode, Sublime Text, Atom, Vim
- **Operating Systems** — Windows, Linux, macOS
- **Other** — SCSS, Sass, Less, Jupyter, Terraform

### Usage Examples

```bash
# Create .gitignore for a React project
gig generate react node npm vscode

# Add Python rules to existing file
gig add python pip

# Remove outdated rules
gig remove atom sublimetext

# Interactive mode to select from all available technologies
gig
```

### License & Community Guidelines

- [License](LICENSE) — project license
- [Code of Conduct](CODE_OF_CONDUCT.md) — expected behavior for contributors
- [Contributing Guide](CONTRIBUTING.md) — how to help the project
- [Security Policy](SECURITY.md) — reporting security issues
