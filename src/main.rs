// use std::{env, process};
// use std::fs::File;
// use std::io::Write;
// // use std::io::{BufReader, BufRead};
// // use std::path::Path;

// fn main() {
//     // Отримуємо ітератор аргументів командного рядка
//     let args: Vec<String> = env::args().collect();
//     let mut content: String = String::new();
//     // let mut is_add: bool = false;
//     // let mut is_remove: bool = false;

//     // Перевіряємо чи є аргументи
//     if args.len() < 2 {
//         eprintln!("No arguments provided. Use --help or -h for usage information.");
//         process::exit(1);
//     }

//     // Виводимо аргументи на екран та обробляємо їх
//     for (index, arg) in args.iter().enumerate().skip(1) {
//         // Відображення допомоги 
//         if index == 1 && (arg == "--help" || arg == "-h") {
//             show_help();
//             return;
//         }
//         // else if arg == "--add" || arg == "-a" {
//         //     is_add = true;
//         //     is_remove = false;
//         //     continue;
//         // }
//         // else if arg == "--remove" || arg == "-r" {
//         //     is_add = false;
//         //     is_remove = true;
//         //     continue;
//         // }
//         // if is_add == true {
//         //     if !valid_argument(arg) {
//         //         eprintln!("Invalid argument: {}", arg);
//         //         process::exit(1);
//         //     }
//         //     else {
//         //         println!("Add rule: {}", arg);
//         //     }
//         // }
//         // else if is_remove == true {
//         //     if !valid_argument(arg) {
//         //         eprintln!("Invalid argument: {}", arg);
//         //         process::exit(1);
//         //     }
//         //     else {
//         //         println!("Remove rule: {}", arg);
//         //     }
//         // }

//         if !valid_argument(arg) {
//             eprintln!("Invalid argument: {}", arg);
//             process::exit(1);
//         }

//         content = generate_gitignore_rule(content, arg);
//     }

//     // Оптимізація тексту файлу .gitignore
//     content = optimize_rules(content);
//     // Запис у файл
//     write_in_file(&content);
// }

// // Функція для перевірки валідності аргументів
// fn valid_argument(arg: &str) -> bool {
//     let allowed_args =
//     // Programming languages
//     ["c", "c++", "c#", "java", "go", "rust", "python", "php", "ruby", "swift", "kotlin",
//     // Frameworks
//     "react", "angular", "vue", "node", "django", "tauri", "spring", "rubyonrails", "laravel", "express", "flutter", "svelte", "next", "nuxt", "flask", "asp", "electron",
//     // Game engines
//     "unity", "unreal", "godot", "renpy",
//     // Developer tools
//     "docker", "npm", "pnpm", "yarn", "gradle", "maven", "webpack", "composer", "pip",
//     // Database managers
//     "mysql", "mongodb", "sqllite", "firebase",
//     // Code editors & IDE
//     "vscode", "idea", "visualstudio", "eclipse", "androidstudio", "xcode", "sublimetext", "atom", "vim",
//     // Operating systems
//     "windows", "linux", "macos",
//     // Other
//     "scss", "less", "jupiter", "terraform"];
//     return allowed_args.contains(&arg);
// }

// // Функція, що приймає в себе змінну та аргумент і повертає видозмінену змінну
// fn generate_gitignore_rule(mut content: String, arg: &str) -> String {
//     content.push_str(&format!("# {}\n", arg));
    
//     match arg {
//         "vscode" => {
//             content.push_str(".vscode/*\n");
//             content.push_str(".history*\n");
//             content.push_str("*.vsix\n");
//         },
//         "rust" => {
//             content.push_str("/target\n");
//         },
//         "test" => {
//             content.push_str("test\n");
//             content.push_str("test1\n");
//         },
//         "test1" => {
//             content.push_str("test\n");
//             content.push_str("test3\n");
//         },
//         "test2" => {
//             content.push_str("test1\n");
//             content.push_str("test2\n");
//         },
//         _ => {}
//     }

//     content.push('\n');
//     return content;
// }

// // // Функція шукає в робочій теці файл .gitignore та зчитує його значення у змінну
// // fn read_gitignore_rules() -> String {
// //     let path = Path::new(".gitignore");
// //     let mut content = String::new();

// //     if path.exists() {
// //         // Відкриваємо файл .gitignore для читання
// //         let file = match File::open(&path) {
// //             Ok(file) => file,
// //             Err(e) => {
// //                 eprintln!("Error opening file: {}", e);
// //                 return content;
// //             }
// //         };

// //         // Створюємо буферний рідер для читання рядків з файлу
// //         let reader = BufReader::new(file);
// //         for line in reader.lines() {
// //             match line {
// //                 Ok(line) => content.push_str(&line),
// //                 Err(e) => {
// //                     eprintln!("Error reading line: {}", e);
// //                     return content;
// //                 }
// //             }
// //             content.push('\n');
// //         }
// //     } else {
// //         println!(".gitignore file not found.");
// //     }

// //     println!("{}", content);
// //     return content;
// // }


// // // Функція приймає в себе контент взятий з файлу та додає до нього нові правила не переписуючи старі
// // fn add_gitignore_rule(mut content: String, arg: &str) -> String {
// //     content.push_str(&format!("# {}\n", arg));
    
// //     match arg {
// //         "vscode" => {
// //             content.push_str(".vscode/*\n");
// //             content.push_str(".history*\n");
// //             content.push_str("*.vsix\n");
// //         },
// //         "rust" => {
// //             content.push_str("/target\n");
// //         },
// //         _ => {}
// //     }

// //     content.push('\n');
// //     return content;
// // }

// // // // Функція приймає в себе контент взятий з файлу та вилучає з нього вказані правила не переписуючи старі, але перевіряючи, чи не були старі правила оптимізовані за рахунок стирання повторюваних правил, і якщо було стерто, то дописує правило що повторювалось в нове місце
// // fn remove_gitignore_rule(mut content: String, arg: &str) -> String {
// //     content.push_str(&format!("# {}\n", arg));
    
// //     match arg {
// //         "vscode" => {
// //             content.push_str(".vscode/*\n");
// //             content.push_str(".history*\n");
// //             content.push_str("*.vsix\n");
// //         },
// //         "rust" => {
// //             content.push_str("/target\n");
// //         },
// //         _ => {}
// //     }

// //     content.push('\n');
// //     return content;
// // }

// // Функція для оптимізації тексту в .gitignore за рахунок видалення повторюваних рядків
// fn optimize_rules(content: String) -> String {
//     let mut unique_lines: std::collections::HashSet<String> = std::collections::HashSet::new();
//     let mut optimized_content = String::new();

//     for line in content.lines() {
//         if unique_lines.insert(line.to_string()) {
//             optimized_content.push_str(line);
//             optimized_content.push('\n');
//         }
//     }

//     return optimized_content;
// }

// // Функція, що приймає в себе змінну і записує цю змінну в файл
// fn write_in_file(written_text: &str) {
//     // Відкриваємо файл .gitignore для запису
//     let mut file = match File::create(".gitignore") {
//         Ok(file) => file,
//         Err(e) => {
//             eprintln!("Error creating file: {}", e);
//             return;
//         },
//     };

//     // Записуємо вміст у файл
//     match file.write_all(written_text.as_bytes()) {
//         Ok(_) => println!("Successfully wrote to .gitignore"),
//         Err(e) => eprintln!("Error writing to file: {}", e),
//     };
// }

// // Функція для виводу допомоги
// fn show_help() {
//     println!("\x1b[1mUsage:\x1b[0m gig [OPTIONS] [RULES]");
//     print!("\n");
//     println!("\x1b[1mOptions:\x1b[0m");
//     println!("  -h, --help      Show help message");
//     // println!("  -a, --add       Add a new rule to an existing .gitignore file");
//     // println!("  -r, --remove    Remove a rule from an existing .gitignore file");
//     // print!("\n");
//     // println!("\x1b[1mNote:\x1b[0m");
//     // println!("  If neither --add nor --remove options are used, the .gitignore\n  file will be completely overwritten.");
//     print!("\n");
//     println!("\x1b[1mRules:\x1b[0m");
//     println!("  List of .gitignore rules to be added or removed");
//     print!("\n");
//     println!("\x1b[1mGiG (.gitignore generator)\x1b[0m is a terminal utility to simplify the\ncreation of .gitignore files");
// }


use std::{env, process};
use std::fs::File;
use std::io::Write;
// use std::io::{BufReader, BufRead};
// use std::path::Path;

fn main() {
    // Отримуємо ітератор аргументів командного рядка
    let args: Vec<String> = env::args().collect();

    // Перевіряємо чи є аргументи
    if args.len() < 2 {
        eprintln!("No arguments provided. Use --help or -h for usage information.");
        process::exit(1);
    }

    // Обробляємо аргументи
    let mut content = String::new();
    // let mut is_add: bool = false;
    // let mut is_remove: bool = false;

    for (index, arg) in args.iter().enumerate().skip(1) {
        if index == 1 && (arg == "--help" || arg == "-h") {
            show_help();
            return;
        }
        // else if arg == "--add" || arg == "-a" {
        //     is_add = true;
        //     is_remove = false;
        //     continue;
        // }
        // else if arg == "--remove" || arg == "-r" {
        //     is_add = false;
        //     is_remove = true;
        //     continue;
        // }

        if !valid_argument(arg) {
            eprintln!("Invalid argument: {}", arg);
            process::exit(1);
        }

        // if is_add {
        //     content = add_gitignore_rule(content, arg);
        // } else if is_remove {
        //     content = remove_gitignore_rule(content, arg);
        // } else {
        content = generate_gitignore_rule(content, arg);
        // }
    }

    // Оптимізація тексту файлу .gitignore
    content = optimize_rules(content);
    // Запис у файл
    write_in_file(&content);
}

// Функція для перевірки валідності аргументів
fn valid_argument(arg: &str) -> bool {
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
        "mysql", "mongodb", "sqllite", "firebase",
        // Code editors & IDE
        "vscode", "idea", "visualstudio", "eclipse", "androidstudio", "xcode", "sublimetext", "atom", "vim",
        // Operating systems
        "windows", "linux", "macos",
        // Other
        "scss", "less", "jupiter", "terraform"
    ];
    return allowed_args.contains(&arg);
}

// Функція, що приймає в себе змінну та аргумент і повертає видозмінену змінну
fn generate_gitignore_rule(mut content: String, arg: &str) -> String {
    content.push_str(&format!("# {}\n", arg));
    
    match arg {
        "c" => {
            content.push_str("*.o\n");
            content.push_str("*.obj\n");
            content.push_str("*.so\n");
            content.push_str("*.exe\n");
        },
        "c++" => {
            content.push_str("*.o\n");
            content.push_str("*.obj\n");
            content.push_str("*.so\n");
            content.push_str("*.exe\n");
        },
        "csharp" => {
            content.push_str("*.dll\n");
            content.push_str("*.exe\n");
            content.push_str("*.pdb\n");
            content.push_str("*.user\n");
            content.push_str("*.cache\n");
            content.push_str("*.mdb\n");
        },
        "java" => {
            content.push_str("/bin/\n");
            content.push_str("/build/\n");
            content.push_str("*.class\n");
        },
        "kotlin" => {
            content.push_str("/build/\n");
        },
        "swift" => {
            content.push_str("build/\n");
            content.push_str(".build/\n");
            content.push_str("Packages/\n");
        },
        "rust" => {
            content.push_str("/target/\n");
        },
        "python" => {
            content.push_str("__pycache__/\n");
            content.push_str("*.py[cod]\n");
        },
        "go" => {
            content.push_str("/bin/\n");
            content.push_str("/pkg/\n");
        },
        "ruby" => {
            content.push_str("/.bundle/\n");
            content.push_str("/vendor/bundle/\n");
            content.push_str("/log/\n");
            content.push_str("/tmp/\n");
        },
        "php" => {
            content.push_str("vendor/\n");
        },
        "react" => {
            content.push_str("node_modules/\n");
            content.push_str("build/\n");
        },
        "angular" => {
            content.push_str("node_modules/\n");
            content.push_str("dist/\n");
        },
        "vue" => {
            content.push_str("node_modules/\n");
            content.push_str("dist/\n");
        },
        "svelte" => {
            content.push_str("node_modules/\n");
            content.push_str("public/build/\n");
        },
        "node" => {
            content.push_str("node_modules/\n");
        },
        "next" => {
            content.push_str(".next/\n");
            content.push_str("node_modules/\n");
        },
        "nuxt" => {
            content.push_str(".nuxt/\n");
            content.push_str("node_modules/\n");
            content.push_str("dist/\n");
        },
        "express" => {
            content.push_str("node_modules/\n");
        },
        "django" => {
            content.push_str("*.log\n");
            content.push_str("*.pot\n");
            content.push_str("*.pyc\n");
            content.push_str("__pycache__/\n");
            content.push_str("db.sqlite3\n");
            content.push_str("/media/\n");
            content.push_str("/static/\n");
        },
        "tauri" => {
            content.push_str("/src-tauri/target/\n");
            content.push_str("/src-tauri/.bundle/\n");
            content.push_str("/src-tauri/.parcel-cache/\n");
        },
        "electron" => {
            content.push_str("node_modules/\n");
            content.push_str("dist/\n");
        },
        "spring" => {
            content.push_str("bin/\n");
            content.push_str("logs/\n");
            content.push_str("*.log\n");
            content.push_str("target/\n");
        },
        "rubyonrails" => {
            content.push_str("/.bundle/\n");
            content.push_str("/vendor/bundle/\n");
            content.push_str("/log/\n");
            content.push_str("/tmp/\n");
        },
        "laravel" => {
            content.push_str("/vendor/\n");
            content.push_str("/node_modules/\n");
        },
        "flutter" => {
            content.push_str(".flutter-plugins\n");
            content.push_str(".flutter-plugins-dependencies\n");
            content.push_str(".packages\n");
            content.push_str(".dart_tool/\n");
            content.push_str(".pub-cache/\n");
            content.push_str("build/\n");
        },
        "flask" => {
            content.push_str("instance/\n");
            content.push_str("*.pyc\n");
            content.push_str("__pycache__/\n");
        },
        "asp.net" => {
            content.push_str("bin/\n");
            content.push_str("obj/\n");
        },
        "unity" => {
            content.push_str("Library/\n");
            content.push_str("Temp/\n");
            content.push_str("Obj/\n");
            content.push_str("Build/\n");
            content.push_str("Builds/\n");
            content.push_str("Logs/\n");
            content.push_str("MemoryCaptures/\n");
        },
        "unreal" => {
            content.push_str("*.ncb\n");
            content.push_str("*.sdf\n");
            content.push_str("*.pch\n");
            content.push_str("Binaries/\n");
            content.push_str("DerivedDataCache/\n");
            content.push_str("Intermediate/\n");
            content.push_str("Saved/\n");
        },
        "godot" => {
            content.push_str("export/\n");
        },
        "renpy" => {
            content.push_str("cache/\n");
            content.push_str("saves/\n");
            content.push_str("*.rpyc\n");
            content.push_str("*.rpymc\n");
            content.push_str("*.rpyb\n");
            content.push_str("*.rpy\n");
            content.push_str("*.pyo\n");
        },
        "docker" => {
            content.push_str(".dockerignore\n");
        },
        "npm" => {
            content.push_str("node_modules/\n");
        },
        "pnpm" => {
            content.push_str("node_modules/\n");
            content.push_str("pnpm-lock.yaml\n");
        },
        "yarn" => {
            content.push_str("node_modules/\n");
            content.push_str("yarn.lock\n");
        },
        "gradle" => {
            content.push_str(".gradle/\n");
            content.push_str("build/\n");
        },
        "maven" => {
            content.push_str("target/\n");
        },
        "webpack" => {
            content.push_str("node_modules/\n");
            content.push_str("dist/\n");
        },
        "composer" => {
            content.push_str("vendor/\n");
        },
        "pip" => {
            content.push_str("*.py[cod]\n");
            content.push_str("__pycache__/\n");
        },
        "mysql" => {
            content.push_str("*.sql\n");
            content.push_str("*.sql.gz\n");
        },
        "mongodb" => {
            content.push_str("dump/\n");
        },
        "sqlite" => {
            content.push_str("*.sqlite3\n");
        },
        "firebase" => {
            content.push_str(".firebase/\n");
        },
        "vscode" => {
            content.push_str(".vscode/\n");
        },
        "idea" => {
            content.push_str(".idea/\n");
        },
        "visualstudio" => {
            content.push_str(".vs/\n");
            content.push_str("*.vcxproj\n");
            content.push_str("*.vcxproj.filters\n");
            content.push_str("*.vcxproj.user\n");
        },
        "eclipse" => {
            content.push_str(".metadata/\n");
            content.push_str(".recommenders/\n");
        },
        "androidstudio" => {
            content.push_str(".idea/\n");
            content.push_str(".gradle/\n");
            content.push_str("local.properties\n");
            content.push_str("build/\n");
            content.push_str("captures/\n");
        },
        "xcode" => {
            content.push_str("build/\n");
            content.push_str("DerivedData/\n");
            content.push_str("*.xcworkspace\n");
        },
        "sublimetext" => {
            content.push_str("*.sublime-project\n");
            content.push_str("*.sublime-workspace\n");
        },
        "atom" => {
            content.push_str("*.atom\n");
        },
        "vim" => {
            content.push_str("*.swp\n");
            content.push_str("*.swo\n");
        },
        "windows" => {
            content.push_str("Thumbs.db\n");
            content.push_str("ehthumbs.db\n");
            content.push_str("Desktop.ini\n");
        },
        "linux" => {
            content.push_str("*~\n");
            content.push_str(".DS_Store\n");
        },
        "macos" => {
            content.push_str(".DS_Store\n");
            content.push_str("AppleDouble\n");
        },
        "scss" | "sass" => {
            content.push_str(".sass-cache/\n");
        },
        "less" => {
            content.push_str(".less-cache/\n");
        },
        "jupyter" => {
            content.push_str(".ipynb_checkpoints/\n");
        },
        "terraform" => {
            content.push_str(".terraform/\n");
        },
        _ => {}
    }

    content.push('\n');
    return content;
}

// // Функція шукає в робочій теці файл .gitignore та зчитує його значення у змінну
// fn read_gitignore_rules() -> String {
//     let path = Path::new(".gitignore");
//     let mut content = String::new();

//     if path.exists() {
//         // Відкриваємо файл .gitignore для читання
//         let file = match File::open(&path) {
//             Ok(file) => file,
//             Err(e) => {
//                 eprintln!("Error opening file: {}", e);
//                 return content;
//             }
//         };

//         // Створюємо буферний рідер для читання рядків з файлу
//         let reader = BufReader::new(file);
//         for line in reader.lines() {
//             match line {
//                 Ok(line) => content.push_str(&line),
//                 Err(e) => {
//                     eprintln!("Error reading line: {}", e);
//                     return content;
//                 }
//             }
//             content.push('\n');
//         }
//     } else {
//         println!(".gitignore file not found.");
//     }

//     println!("{}", content);
//     return content;
// }

// // Функція приймає в себе контент взятий з файлу та додає до нього нові правила не переписуючи старі
// fn add_gitignore_rule(mut content: String, arg: &str) -> String {
//     content.push_str(&format!("# {}\n", arg));
    
//     match arg {
//         "vscode" => {
//             content.push_str(".vscode/*\n");
//             content.push_str(".history*\n");
//             content.push_str("*.vsix\n");
//         },
//         "rust" => {
//             content.push_str("/target\n");
//         },
//         _ => {}
//     }

//     content.push('\n');
//     return content;
// }

// // // Функція приймає в себе контент взятий з файлу та вилучає з нього вказані правила не переписуючи старі, але перевіряючи, чи не були старі правила оптимізовані за рахунок стирання повторюваних правил, і якщо було стерто, то дописує правило що повторювалось в нове місце
// fn remove_gitignore_rule(mut content: String, arg: &str) -> String {
//     content.push_str(&format!("# {}\n", arg));
    
//     match arg {
//         "vscode" => {
//             content.push_str(".vscode/*\n");
//             content.push_str(".history*\n");
//             content.push_str("*.vsix\n");
//         },
//         "rust" => {
//             content.push_str("/target\n");
//         },
//         _ => {}
//     }

//     content.push('\n');
//     return content;
// }

// Функція для оптимізації тексту в .gitignore за рахунок видалення повторюваних рядків
fn optimize_rules(content: String) -> String {
    let mut unique_lines = std::collections::HashSet::new();
    let mut optimized_content = String::new();

    for line in content.lines() {
        if unique_lines.insert(line.to_string()) {
            optimized_content.push_str(line);
            optimized_content.push('\n');
        }
    }

    return optimized_content;
}

// Функція, що приймає в себе змінну і записує цю змінну в файл
fn write_in_file(written_text: &str) {
    // Відкриваємо файл .gitignore для запису
    let mut file = match File::create(".gitignore") {
    // let mut file = match File::create(".gitignore.test") {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error creating file: {}", e);
            return;
        },
    };

    // Записуємо вміст у файл
    match file.write_all(written_text.as_bytes()) {
        Ok(_) => println!("Successfully wrote to .gitignore"),
        Err(e) => eprintln!("Error writing to file: {}", e),
    };
}

// Функція для виводу допомоги
fn show_help() {
    println!("\x1b[1mUsage:\x1b[0m gig [OPTIONS] [RULES]");
    println!("\n\x1b[1mOptions:\x1b[0m");
    println!("  -h, --help      Show help message");
    //  // println!("  -a, --add       Add a new rule to an existing .gitignore file");
    //  // println!("  -r, --remove    Remove a rule from an existing .gitignore file");
    //  // println!("\n\x1b[1mNote:\x1b[0m");
    //  // println!("  If neither --add nor --remove options are used, the .gitignore\n  file will be completely overwritten.");
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
    println!("\n\x1b[1mGiG (.gitignore generator)\x1b[0m is a terminal utility to simplify the");
    println!("creation of .gitignore files");
}