use std::path::Path;
use std::env;
use std::fs::{File, OpenOptions};
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Write};

// Структура для зберігання правил .gitignore
struct GitignoreRules {
    rules: HashMap<String, Vec<String>>,
}

impl GitignoreRules {
    fn new() -> Self {
        let mut rules = HashMap::new();
        // Заповнення правил для кожної технології
        rules.insert("c".to_string(), vec!["*.o", "*.obj", "*.so", "*.exe"].into_iter().map(String::from).collect());
        rules.insert("c++".to_string(), vec!["*.o", "*.obj", "*.so", "*.exe"].into_iter().map(String::from).collect());
        rules.insert("csharp".to_string(), vec!["*.dll", "*.exe", "*.pdb", "*.user", "*.cache", "*.mdb"].into_iter().map(String::from).collect());
        rules.insert("java".to_string(), vec!["/bin/", "/build/", "*.class"].into_iter().map(String::from).collect());
        rules.insert("kotlin".to_string(), vec!["/build/"].into_iter().map(String::from).collect());
        rules.insert("swift".to_string(), vec!["build/", ".build/", "Packages/"].into_iter().map(String::from).collect());
        rules.insert("rust".to_string(), vec!["/target/"].into_iter().map(String::from).collect());
        rules.insert("python".to_string(), vec!["__pycache__/", "*.py[cod]"].into_iter().map(String::from).collect());
        rules.insert("go".to_string(), vec!["/bin/", "/pkg/"].into_iter().map(String::from).collect());
        rules.insert("ruby".to_string(), vec!["/.bundle/", "/vendor/bundle/", "/log/", "/tmp/"].into_iter().map(String::from).collect());
        rules.insert("php".to_string(), vec!["vendor/"].into_iter().map(String::from).collect());
        rules.insert("react".to_string(), vec!["node_modules/", "build/"].into_iter().map(String::from).collect());
        rules.insert("angular".to_string(), vec!["node_modules/", "dist/"].into_iter().map(String::from).collect());
        rules.insert("vue".to_string(), vec!["node_modules/", "dist/"].into_iter().map(String::from).collect());
        rules.insert("svelte".to_string(), vec!["node_modules/", "public/build/"].into_iter().map(String::from).collect());
        rules.insert("node".to_string(), vec!["node_modules/"].into_iter().map(String::from).collect());
        rules.insert("next".to_string(), vec![".next/", "node_modules/"].into_iter().map(String::from).collect());
        rules.insert("nuxt".to_string(), vec![".nuxt/", "node_modules/", "dist/"].into_iter().map(String::from).collect());
        rules.insert("express".to_string(), vec!["node_modules/"].into_iter().map(String::from).collect());
        rules.insert("django".to_string(), vec!["*.log", "*.pot", "*.pyc", "__pycache__/", "db.sqlite3", "/media/", "/static/"].into_iter().map(String::from).collect());
        rules.insert("tauri".to_string(), vec!["/src-tauri/target/", "/src-tauri/.bundle/", "/src-tauri/.parcel-cache/"].into_iter().map(String::from).collect());
        rules.insert("electron".to_string(), vec!["node_modules/", "dist/"].into_iter().map(String::from).collect());
        rules.insert("spring".to_string(), vec!["bin/", "logs/", "*.log", "target/"].into_iter().map(String::from).collect());
        rules.insert("rubyonrails".to_string(), vec!["/.bundle/", "/vendor/bundle/", "/log/", "/tmp/"].into_iter().map(String::from).collect());
        rules.insert("laravel".to_string(), vec!["/vendor/", "/node_modules/"].into_iter().map(String::from).collect());
        rules.insert("flutter".to_string(), vec![".flutter-plugins", ".flutter-plugins-dependencies", ".packages", ".dart_tool/", ".pub-cache/", "build/"].into_iter().map(String::from).collect());
        rules.insert("flask".to_string(), vec!["instance/", "*.pyc", "__pycache__/"].into_iter().map(String::from).collect());
        rules.insert("asp.net".to_string(), vec!["bin/", "obj/"].into_iter().map(String::from).collect());
        rules.insert("unity".to_string(), vec!["Library/", "Temp/", "Obj/", "Build/", "Builds/", "Logs/", "MemoryCaptures/"].into_iter().map(String::from).collect());
        rules.insert("unreal".to_string(), vec!["*.ncb", "*.sdf", "*.pch", "Binaries/", "DerivedDataCache/", "Intermediate/", "Saved/"].into_iter().map(String::from).collect());
        rules.insert("godot".to_string(), vec!["export/"].into_iter().map(String::from).collect());
        rules.insert("renpy".to_string(), vec!["cache/", "saves/", "*.rpyc", "*.rpymc", "*.rpyb", "*.rpy", "*.pyo"].into_iter().map(String::from).collect());
        rules.insert("docker".to_string(), vec![".dockerignore"].into_iter().map(String::from).collect());
        rules.insert("npm".to_string(), vec!["node_modules/"].into_iter().map(String::from).collect());
        rules.insert("pnpm".to_string(), vec!["node_modules/", "pnpm-lock.yaml"].into_iter().map(String::from).collect());
        rules.insert("yarn".to_string(), vec!["node_modules/", "yarn.lock"].into_iter().map(String::from).collect());
        rules.insert("gradle".to_string(), vec![".gradle/", "build/"].into_iter().map(String::from).collect());
        rules.insert("maven".to_string(), vec!["target/"].into_iter().map(String::from).collect());
        rules.insert("webpack".to_string(), vec!["node_modules/", "dist/"].into_iter().map(String::from).collect());
        rules.insert("composer".to_string(), vec!["vendor/"].into_iter().map(String::from).collect());
        rules.insert("pip".to_string(), vec!["*.py[cod]", "__pycache__/"].into_iter().map(String::from).collect());
        rules.insert("mysql".to_string(), vec!["*.sql", "*.sql.gz"].into_iter().map(String::from).collect());
        rules.insert("mongodb".to_string(), vec!["dump/"].into_iter().map(String::from).collect());
        rules.insert("sqlite".to_string(), vec!["*.sqlite3"].into_iter().map(String::from).collect());
        rules.insert("firebase".to_string(), vec![".firebase/"].into_iter().map(String::from).collect());
        rules.insert("vscode".to_string(), vec![".vscode/"].into_iter().map(String::from).collect());
        rules.insert("idea".to_string(), vec![".idea/"].into_iter().map(String::from).collect());
        rules.insert("visualstudio".to_string(), vec![".vs/", "*.vcxproj", "*.vcxproj.filters", "*.vcxproj.user"].into_iter().map(String::from).collect());
        rules.insert("eclipse".to_string(), vec![".metadata/", ".recommenders/"].into_iter().map(String::from).collect());
        rules.insert("androidstudio".to_string(), vec![".idea/", ".gradle/", "local.properties", "build/", "captures/"].into_iter().map(String::from).collect());
        rules.insert("xcode".to_string(), vec!["build/", "DerivedData/", "*.xcworkspace"].into_iter().map(String::from).collect());
        rules.insert("sublimetext".to_string(), vec!["*.sublime-project", "*.sublime-workspace"].into_iter().map(String::from).collect());
        rules.insert("atom".to_string(), vec!["*.atom"].into_iter().map(String::from).collect());
        rules.insert("vim".to_string(), vec!["*.swp", "*.swo"].into_iter().map(String::from).collect());
        rules.insert("windows".to_string(), vec!["Thumbs.db", "ehthumbs.db", "Desktop.ini"].into_iter().map(String::from).collect());
        rules.insert("linux".to_string(), vec!["*~", ".DS_Store"].into_iter().map(String::from).collect());
        rules.insert("macos".to_string(), vec![".DS_Store", "AppleDouble"].into_iter().map(String::from).collect());
        rules.insert("scss".to_string(), vec![".sass-cache/"].into_iter().map(String::from).collect());
        rules.insert("sass".to_string(), vec![".sass-cache/"].into_iter().map(String::from).collect());
        rules.insert("less".to_string(), vec![".less-cache/"].into_iter().map(String::from).collect());
        rules.insert("jupyter".to_string(), vec![".ipynb_checkpoints/"].into_iter().map(String::from).collect());
        rules.insert("terraform".to_string(), vec![".terraform/"].into_iter().map(String::from).collect());
        
        Self { rules }
    }

    fn get_rules(&self, tech: &str) -> Option<&Vec<String>> {
        self.rules.get(tech)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("No arguments provided. Use --help or -h for usage information.");
        std::process::exit(1);
    }

    let mut content = String::new();
    let mut is_add = false;
    let mut is_remove = false;
    let gitignore_rules = GitignoreRules::new();

    for (index, arg) in args.iter().enumerate().skip(1) {
        match (index, arg.as_str()) {
            (1, "--help") | (1, "-h") => {
                show_help();
                return;
            }
            (1, "--add") | (1, "-a") => {
                if !is_remove {
                    content = read_gitignore_rules();
                }
                is_add = true;
                is_remove = false;
                continue;
            }
            (1, "--remove") | (1, "-r") => {
                if !is_add {
                    content = read_gitignore_rules();
                }
                is_add = false;
                is_remove = true;
                continue;
            }
            (_, arg) if arg == "--help" || arg == "-h" || arg == "--add" || arg == "-a" || arg == "--remove" || arg == "-r" => {
                println!("Options must be before arguments");
                return;
            }
            _ => {}
        }

        if !valid_argument(arg) {
            eprintln!("Invalid argument: {}", arg);
            std::process::exit(1);
        }

        content = if is_add {
            add_gitignore_rule(content, arg, &gitignore_rules)
        } else if is_remove {
            remove_gitignore_rule(content, arg, &gitignore_rules)
        } else {
            generate_gitignore_rule(content, arg, &gitignore_rules)
        };
    }

    content = optimize_rules(content);
    write_in_file(&content);
}

// Функція для перевірки валідності аргументів
fn valid_argument(arg: &str) -> bool {
    // Список дозволених аргументів
    let allowed_args = [
        // Programming languages
        "c", "c++", "csharp", "java", "go", "rust", "python", "php", "ruby", "swift", "kotlin",
        // Frameworks
        "react", "angular", "vue", "node", "django", "tauri", "spring", "rubyonrails", "laravel", "express", "flutter", "svelte", "next", "nuxt", "flask", "asp", "electron",
        // Game engines
        "unity", "unreal", "godot", "renpy",
        // Developer tools
        "docker", "npm", "pnpm", "yarn", "gradle", "maven", "webpack", "composer", "pip",
        // Database managers
        "mysql", "mongodb", "sqlite", "firebase",
        // Code editors & IDE
        "vscode", "idea", "visualstudio", "eclipse", "androidstudio", "xcode", "sublimetext", "atom", "vim",
        // Operating systems
        "windows", "linux", "macos",
        // Other
        "scss", "sass", "less", "jupyter", "terraform"
    ];
    allowed_args.contains(&arg)
}

// Функція, що приймає в себе змінну та аргумент і повертає видозмінену змінну
fn generate_gitignore_rule(mut content: String, arg: &str, rules: &GitignoreRules) -> String {
    content.push_str(&format!("# {}\n", arg));

    if let Some(tech_rules) = rules.get_rules(arg) {
        for rule in tech_rules {
            content.push_str(rule);
            content.push('\n');
        }
    }

    content.push('\n');
    content
}

// Функція шукає в робочій теці файл .gitignore та зчитує його значення у змінну
fn read_gitignore_rules() -> String {
    // let path = Path::new(".gitignore.test");
    let path = Path::new(".gitignore");
    let mut content = String::new();

    if path.exists() {
        let file = File::open(&path).unwrap_or_else(|e| {
            eprintln!("Error opening file: {}", e);
            std::process::exit(1);
        });

        let reader = BufReader::new(file);
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    content.push_str(&line);
                    content.push('\n');
                }
                Err(e) => {
                    eprintln!("Error reading line: {}", e);
                    std::process::exit(1);
                }
            }
        }
    } else {
        println!(".gitignore file not found.");
    }

    content
}

// Функція приймає в себе контент взятий з файлу та додає до нього нові правила не переписуючи старі
fn add_gitignore_rule(mut content: String, arg: &str, rules: &GitignoreRules) -> String {
    content.push_str(&format!("# {}\n", arg));

    if let Some(tech_rules) = rules.get_rules(arg) {
        for rule in tech_rules {
            content.push_str(rule);
            content.push('\n');
        }
    }

    content.push('\n');
    content
}

// Функція приймає в себе контент взятий з файлу та видаляє з нього непотрібні правила не переписуючи старі
fn remove_gitignore_rule(content: String, arg: &str, rules: &GitignoreRules) -> String {
    let mut lines_to_remove = HashSet::new();
    
    // Формуємо коментар, додаючи пробіл після решітки
    let comment_line = format!("# {}", arg);

    if let Some(tech_rules) = rules.get_rules(arg) {
        lines_to_remove.insert(comment_line);
        for rule in tech_rules {
            lines_to_remove.insert(rule.clone());
        }
    }

    content.lines()
           .filter(|line| !lines_to_remove.contains(&line.to_string()))
           .collect::<Vec<_>>()
           .join("\n")
}

// Функція для оптимізації тексту в .gitignore за рахунок видалення повторюваних рядків
fn optimize_rules(content: String) -> String {
    let mut unique_lines = HashSet::new();
    content.lines()
           .filter(|line| unique_lines.insert(line.to_string()))
           .collect::<Vec<_>>()
           .join("\n")
}

// Функція, що приймає в себе змінну і записує цю змінну в файл
fn write_in_file(written_text: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        // .open(".gitignore.test")
        .open(".gitignore")
        .unwrap_or_else(|e| {
            eprintln!("Error creating file: {}", e);
            std::process::exit(1);
        });

    file.write_all(written_text.as_bytes()).unwrap_or_else(|e| {
        eprintln!("Error writing to file: {}", e);
        std::process::exit(1);
    });

    println!("Successfully wrote to .gitignore");
}

// Функція для виводу допомоги
fn show_help() {
    println!("\x1b[1mUsage:\x1b[0m gig [OPTION] [RULES]");
    println!("\n\x1b[1mOptions:\x1b[0m");
    println!("  -h, --help      Show help message");
    println!("  -a, --add       Add a new rule to an existing .gitignore file");
    println!("  -r, --remove    Remove a rule from an existing .gitignore file");
    println!("\n\x1b[1mNote:\x1b[0m");
    println!("  If neither --add nor --remove options are used, the .gitignore\n  file will be completely overwritten.");
    println!("\n\x1b[1mRules:\x1b[0m");
    println!("  \x1b[1mLanguages:\x1b[0m c, c++, csharp, java, go, rust, python, php, ruby,\n  swift, kotlin");
    println!("  \x1b[1mFrameworks:\x1b[0m react, angular, vue, node, django, tauri, spring,\n  rubyonrails, laravel, express, flutter, svelte, next, nuxt, flask,\n  asp, electron");
    println!("  \x1b[1mGame Engines:\x1b[0m unity, unreal, godot, renpy");
    println!("  \x1b[1mDev Tools:\x1b[0m docker, npm, pnpm, yarn, gradle, maven, webpack,\n  composer, pip");
    println!("  \x1b[1mDatabases:\x1b[0m mysql, mongodb, sqllite, firebase");
    println!("  \x1b[1mIDEs/Editors:\x1b[0m vscode, idea, visualstudio, eclipse, androidstudio,\n  xcode, sublimetext, atom, vim");
    println!("  \x1b[1mOS:\x1b[0m windows, linux, macos");
    println!("  \x1b[1mOther:\x1b[0m scss, less, jupiter, terraform");
    println!("\n\x1b[1mExample:\x1b[0m");
    println!("  gig react node vscode");
    println!("  gig -add renpy idea");
    println!("  gig --remove python django");
    // println!("  gig --add ruby rubyonrails --remove python django");
    println!("\n\x1b[1mGiG (.gitignore generator)\x1b[0m is a terminal utility to simplify the");
    println!("creation of .gitignore files for your projects");
}
