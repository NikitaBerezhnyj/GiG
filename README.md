# GiG

GiG repository has [Ukrainian :ukraine:](#gig-ukraine) and [English :uk:](#gig-uk) localizations

## GiG :ukraine:

<p align='center'>
  <img src='./icon.png' alt='Іконка застосунку' style="width:75%">
</p>

***GiG (.gitignore generator)*** - це утиліта командного рядка для спрощення створення примітивних .gitignore файлів. GiG дозволяє швидко генерувати файли .gitignore з попередньо визначеними правилами для різних мов програмування, фреймворків, інструментів розробки та операційних систем. Це допомагає розробникам легко створювати відповідні файли .gitignore для своїх проектів, не витрачаючи час на ручне написання правил.

### Особливості

- Підтримка широкого спектру мов програмування та технологій
- Автоматична оптимізація правил для уникнення дублювання
- Простий інтерфейс командного рядка

## Використання

```bash
gig [OPTIONS] [RULES]
```

#### Опції

- `-h`, `--help`: Показати довідку

#### Правила

Вкажіть одне або кілька правил для включення у файл .gitignore. Наприклад:

```bash
gig react node vscode
```

Це створить файл .gitignore з правилами для React, Node.js та VS Code.

### Підтримувані правила

GiG підтримує широкий спектр правил, включаючи:

- ***Мови програмування*** (C, C++, C#, Java, Go, Rust, Python, PHP, Ruby, Swift, Kotlin)
- ***Фреймворки*** (React, Angular, Vue, Node, Django, Tauri, Spring, Ruby on Rails, Laravel, Express, Flutter, Svelte, Next.js, Nuxt.js, Flask, ASP.NET, Electron)
- ***Ігрові двигуни*** (Unity, Unreal Engine, Godot, Ren'Py)
- ***Інструменти розробника*** (Docker, npm, pnpm, Yarn, Gradle, Maven, Webpack, Composer, pip)
- ***Системи керування базами даних*** (MySQL, MongoDB, SQLite, Firebase)
- ***Редактори коду та IDE*** (VS Code, IntelliJ IDEA, Visual Studio, Eclipse, Android Studio, Xcode, Sublime Text, Atom, Vim)
- ***Операційні системи*** (Windows, Linux, macOS)
- ***Інше*** (SCSS, Less, Jupyter, Terraform)

### Встановлення

1. Клонуйте репозиторій на свій ПК

```bash
git clone https://github.com/NikitaBerezhnyj/GiG.git
```

2. Перейдіть у теку

```bash
cd GiG
```

3. Запустіть команду на збірку

```bash
make install
```

___

## GiG :uk:

<p align='center'>
  <img src='./icon.png' alt='Application icon' style=“width:75%”>
</p>

***GiG (.gitignore generator)*** is a command-line utility to simplify the creation of primitive .gitignore files. GiG allows you to quickly generate .gitignore files with predefined rules for different programming languages, frameworks, development tools, and operating systems. This helps developers easily create appropriate .gitignore files for their projects without wasting time manually writing rules.

### Features

- Supports a wide range of programming languages and technologies
- Automatic optimization of rules to avoid duplication
- Simple command line interface

## Usage

```bash
gig [OPTIONS] [RULES]
```

#### Options

- `-h`, `--help`: Show help

#### Rules.

Specify one or more rules to include in the .gitignore file. For example:

```bash
gig react node vscode
```

This will create a .gitignore file with rules for React, Node.js, and VS Code.

### Supported rules

GiG supports a wide range of rules, including:

- ***Programming languages*** (C, C++, C#, Java, Go, Rust, Python, PHP, Ruby, Swift, Kotlin)
- ***Frameworks*** (React, Angular, Vue, Node, Django, Tauri, Spring, Ruby on Rails, Laravel, Express, Flutter, Svelte, Next.js, Nuxt.js, Flask, ASP.NET, Electron)
- ***Game engines*** (Unity, Unreal Engine, Godot, Ren'Py)
- ***Developer tools*** (Docker, npm, pnpm, Yarn, Gradle, Maven, Webpack, Composer, pip)
- ***Database management systems*** (MySQL, MongoDB, SQLite, Firebase)
- ***Code editors and IDEs*** (VS Code, IntelliJ IDEA, Visual Studio, Eclipse, Android Studio, Xcode, Sublime Text, Atom, Vim)
- ***Operating systems*** (Windows, Linux, macOS)
- ***Other*** (SCSS, Less, Jupyter, Terraform)

### Installation

1. Clone the repository to your PC

```bash
git clone https://github.com/NikitaBerezhnyj/GiG.git
```

2. Change to the folder

```bash
cd GiG
```

3. Run the command to build

```bash
make install
```