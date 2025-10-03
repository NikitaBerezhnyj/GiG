use clap::{Parser, Subcommand};
use inquire::MultiSelect;
use std::collections::{HashMap, HashSet};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

#[derive(Parser)]
#[command(
    name = "gig",
    version = "2.0",
    about = ".gitignore file generator",
    long_about = "CLI tool to quickly generate or modify .gitignore files for your projects.\nSupports both command-line arguments and interactive mode.",
    after_help = "Examples:\n  gig                          # Interactive mode\n  gig rust node vscode         # Quick generation\n  gig add python django        # Add rules\n  gig remove react node        # Remove rules"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Add rules to existing .gitignore")]
    Add {
        #[arg(help = "Technologies to add (e.g., rust, node, vscode)")]
        technologies: Vec<String>,
        #[arg(short, long, help = "Interactive selection mode")]
        interactive: bool,
    },
    #[command(about = "Remove rules from .gitignore")]
    Remove {
        #[arg(help = "Technologies to remove")]
        technologies: Vec<String>,
        #[arg(short, long, help = "Interactive selection mode")]
        interactive: bool,
    },
    #[command(about = "Generate new .gitignore (overwrites existing)")]
    Generate {
        #[arg(help = "Technologies to include")]
        technologies: Vec<String>,
        #[arg(short, long, help = "Interactive selection mode")]
        interactive: bool,
    },
}

#[derive(Debug, Clone)]
struct Category {
    name: &'static str,
    items: Vec<&'static str>,
}

struct GitignoreRules {
    rules: HashMap<String, Vec<String>>,
    categories: Vec<Category>,
}

impl GitignoreRules {
    fn new() -> Self {
        let mut rules = HashMap::new();

        // Programming languages
        rules.insert(
            "c".to_string(),
            vec!["*.o", "*.obj", "*.so", "*.exe"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        rules.insert(
            "c++".to_string(),
            vec!["*.o", "*.obj", "*.so", "*.exe"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        rules.insert(
            "csharp".to_string(),
            vec!["*.dll", "*.exe", "*.pdb", "*.user", "*.cache", "*.mdb"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        rules.insert(
            "java".to_string(),
            vec!["/bin/", "/build/", "*.class"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        rules.insert(
            "kotlin".to_string(),
            vec!["/build/"].iter().map(|s| s.to_string()).collect(),
        );
        rules.insert(
            "swift".to_string(),
            vec!["build/", ".build/", "Packages/"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        rules.insert(
            "rust".to_string(),
            vec!["/target/"].iter().map(|s| s.to_string()).collect(),
        );
        rules.insert(
            "python".to_string(),
            vec!["__pycache__/", "*.py[cod]"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        rules.insert(
            "go".to_string(),
            vec!["/bin/", "/pkg/"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        rules.insert(
            "ruby".to_string(),
            vec!["/.bundle/", "/vendor/bundle/", "/log/", "/tmp/"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        rules.insert(
            "php".to_string(),
            vec!["vendor/"].iter().map(|s| s.to_string()).collect(),
        );

        // Frameworks
        rules.insert(
            "react".to_string(),
            vec!["node_modules/", "build/"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        rules.insert(
            "angular".to_string(),
            vec!["node_modules/", "dist/"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        rules.insert(
            "vue".to_string(),
            vec!["node_modules/", "dist/"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        rules.insert(
            "svelte".to_string(),
            vec!["node_modules/", "public/build/"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        rules.insert(
            "node".to_string(),
            vec!["node_modules/"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        rules.insert(
            "next".to_string(),
            vec![".next/", "node_modules/"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        rules.insert(
            "nuxt".to_string(),
            vec![".nuxt/", "node_modules/", "dist/"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        rules.insert(
            "express".to_string(),
            vec!["node_modules/"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        rules.insert(
            "django".to_string(),
            vec![
                "*.log",
                "*.pot",
                "*.pyc",
                "__pycache__/",
                "db.sqlite3",
                "/media/",
                "/static/",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        );
        rules.insert(
            "tauri".to_string(),
            vec![
                "/src-tauri/target/",
                "/src-tauri/.bundle/",
                "/src-tauri/.parcel-cache/",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        );
        rules.insert(
            "electron".to_string(),
            vec!["node_modules/", "dist/"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        rules.insert(
            "spring".to_string(),
            vec!["bin/", "logs/", "*.log", "target/"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        rules.insert(
            "rubyonrails".to_string(),
            vec!["/.bundle/", "/vendor/bundle/", "/log/", "/tmp/"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        rules.insert(
            "laravel".to_string(),
            vec!["/vendor/", "/node_modules/"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        rules.insert(
            "flutter".to_string(),
            vec![
                ".flutter-plugins",
                ".flutter-plugins-dependencies",
                ".packages",
                ".dart_tool/",
                ".pub-cache/",
                "build/",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        );
        rules.insert(
            "flask".to_string(),
            vec!["instance/", "*.pyc", "__pycache__/"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        rules.insert(
            "asp.net".to_string(),
            vec!["bin/", "obj/"].iter().map(|s| s.to_string()).collect(),
        );

        // Game engines
        rules.insert(
            "unity".to_string(),
            vec![
                "Library/",
                "Temp/",
                "Obj/",
                "Build/",
                "Builds/",
                "Logs/",
                "MemoryCaptures/",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        );
        rules.insert(
            "unreal".to_string(),
            vec![
                "*.ncb",
                "*.sdf",
                "*.pch",
                "Binaries/",
                "DerivedDataCache/",
                "Intermediate/",
                "Saved/",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        );
        rules.insert(
            "godot".to_string(),
            vec!["export/"].iter().map(|s| s.to_string()).collect(),
        );
        rules.insert(
            "renpy".to_string(),
            vec![
                "cache/", "saves/", "*.rpyc", "*.rpymc", "*.rpyb", "*.rpy", "*.pyo",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        );

        // Developer tools
        rules.insert(
            "docker".to_string(),
            vec![".dockerignore"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        rules.insert(
            "npm".to_string(),
            vec!["node_modules/"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        rules.insert(
            "pnpm".to_string(),
            vec!["node_modules/", "pnpm-lock.yaml"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        rules.insert(
            "yarn".to_string(),
            vec!["node_modules/", "yarn.lock"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        rules.insert(
            "gradle".to_string(),
            vec![".gradle/", "build/"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        rules.insert(
            "maven".to_string(),
            vec!["target/"].iter().map(|s| s.to_string()).collect(),
        );
        rules.insert(
            "webpack".to_string(),
            vec!["node_modules/", "dist/"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        rules.insert(
            "composer".to_string(),
            vec!["vendor/"].iter().map(|s| s.to_string()).collect(),
        );
        rules.insert(
            "pip".to_string(),
            vec!["*.py[cod]", "__pycache__/"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );

        // Database managers
        rules.insert(
            "mysql".to_string(),
            vec!["*.sql", "*.sql.gz"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        rules.insert(
            "mongodb".to_string(),
            vec!["dump/"].iter().map(|s| s.to_string()).collect(),
        );
        rules.insert(
            "sqlite".to_string(),
            vec!["*.sqlite3"].iter().map(|s| s.to_string()).collect(),
        );
        rules.insert(
            "firebase".to_string(),
            vec![".firebase/"].iter().map(|s| s.to_string()).collect(),
        );

        // IDEs
        rules.insert(
            "vscode".to_string(),
            vec![".vscode/"].iter().map(|s| s.to_string()).collect(),
        );
        rules.insert(
            "idea".to_string(),
            vec![".idea/"].iter().map(|s| s.to_string()).collect(),
        );
        rules.insert(
            "visualstudio".to_string(),
            vec![".vs/", "*.vcxproj", "*.vcxproj.filters", "*.vcxproj.user"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        rules.insert(
            "eclipse".to_string(),
            vec![".metadata/", ".recommenders/"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        rules.insert(
            "androidstudio".to_string(),
            vec![
                ".idea/",
                ".gradle/",
                "local.properties",
                "build/",
                "captures/",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        );
        rules.insert(
            "xcode".to_string(),
            vec!["build/", "DerivedData/", "*.xcworkspace"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        rules.insert(
            "sublimetext".to_string(),
            vec!["*.sublime-project", "*.sublime-workspace"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        rules.insert(
            "atom".to_string(),
            vec!["*.atom"].iter().map(|s| s.to_string()).collect(),
        );
        rules.insert(
            "vim".to_string(),
            vec!["*.swp", "*.swo"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );

        // Operating systems
        rules.insert(
            "windows".to_string(),
            vec!["Thumbs.db", "ehthumbs.db", "Desktop.ini"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        rules.insert(
            "linux".to_string(),
            vec!["*~", ".DS_Store"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        rules.insert(
            "macos".to_string(),
            vec![".DS_Store", "AppleDouble"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );

        // Other
        rules.insert(
            "scss".to_string(),
            vec![".sass-cache/"].iter().map(|s| s.to_string()).collect(),
        );
        rules.insert(
            "sass".to_string(),
            vec![".sass-cache/"].iter().map(|s| s.to_string()).collect(),
        );
        rules.insert(
            "less".to_string(),
            vec![".less-cache/"].iter().map(|s| s.to_string()).collect(),
        );
        rules.insert(
            "jupyter".to_string(),
            vec![".ipynb_checkpoints/"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        rules.insert(
            "terraform".to_string(),
            vec![".terraform/"].iter().map(|s| s.to_string()).collect(),
        );

        let categories = vec![
            Category {
                name: "Programming Languages",
                items: vec![
                    "c", "c++", "csharp", "java", "kotlin", "swift", "rust", "python", "go",
                    "ruby", "php",
                ],
            },
            Category {
                name: "Frameworks",
                items: vec![
                    "react",
                    "angular",
                    "vue",
                    "svelte",
                    "node",
                    "next",
                    "nuxt",
                    "express",
                    "django",
                    "tauri",
                    "electron",
                    "spring",
                    "rubyonrails",
                    "laravel",
                    "flutter",
                    "flask",
                    "asp.net",
                ],
            },
            Category {
                name: "Game Engines",
                items: vec!["unity", "unreal", "godot", "renpy"],
            },
            Category {
                name: "Developer Tools",
                items: vec![
                    "docker", "npm", "pnpm", "yarn", "gradle", "maven", "webpack", "composer",
                    "pip",
                ],
            },
            Category {
                name: "Databases",
                items: vec!["mysql", "mongodb", "sqlite", "firebase"],
            },
            Category {
                name: "IDEs & Editors",
                items: vec![
                    "vscode",
                    "idea",
                    "visualstudio",
                    "eclipse",
                    "androidstudio",
                    "xcode",
                    "sublimetext",
                    "atom",
                    "vim",
                ],
            },
            Category {
                name: "Operating Systems",
                items: vec!["windows", "linux", "macos"],
            },
            Category {
                name: "Other",
                items: vec!["scss", "sass", "less", "jupyter", "terraform"],
            },
        ];

        Self { rules, categories }
    }

    fn get_rules(&self, tech: &str) -> Option<&Vec<String>> {
        self.rules.get(tech)
    }

    fn is_valid(&self, tech: &str) -> bool {
        self.rules.contains_key(tech)
    }
}

fn main() {
    let cli = Cli::parse();
    let gitignore_rules = GitignoreRules::new();

    match cli.command {
        Some(Commands::Add {
            technologies,
            interactive,
        }) => {
            let techs = if interactive || technologies.is_empty() {
                interactive_select(&gitignore_rules, "Select technologies to ADD:")
            } else {
                validate_technologies(&technologies, &gitignore_rules)
            };

            if !techs.is_empty() {
                let mut content = read_gitignore_rules();
                for tech in &techs {
                    content = add_gitignore_rule(content, tech, &gitignore_rules);
                }
                content = optimize_rules(content);
                write_in_file(&content);
                println!(
                    "✓ Successfully added {} technologies to .gitignore",
                    techs.len()
                );
            }
        }
        Some(Commands::Remove {
            technologies,
            interactive,
        }) => {
            let techs = if interactive || technologies.is_empty() {
                interactive_select(&gitignore_rules, "Select technologies to REMOVE:")
            } else {
                validate_technologies(&technologies, &gitignore_rules)
            };

            if !techs.is_empty() {
                let mut content = read_gitignore_rules();
                for tech in &techs {
                    content = remove_gitignore_rule(content, tech, &gitignore_rules);
                }
                content = optimize_rules(content);
                write_in_file(&content);
                println!(
                    "✓ Successfully removed {} technologies from .gitignore",
                    techs.len()
                );
            }
        }
        Some(Commands::Generate {
            technologies,
            interactive,
        }) => {
            let techs = if interactive || technologies.is_empty() {
                interactive_select(&gitignore_rules, "Select technologies to include:")
            } else {
                validate_technologies(&technologies, &gitignore_rules)
            };

            if !techs.is_empty() {
                let mut content = String::new();
                for tech in &techs {
                    content = generate_gitignore_rule(content, tech, &gitignore_rules);
                }
                content = optimize_rules(content);
                write_in_file(&content);
                println!(
                    "✓ Successfully generated .gitignore with {} technologies",
                    techs.len()
                );
            }
        }
        None => {
            let techs = interactive_select(&gitignore_rules, "Select technologies for .gitignore:");
            if !techs.is_empty() {
                let mut content = String::new();
                for tech in &techs {
                    content = generate_gitignore_rule(content, tech, &gitignore_rules);
                }
                content = optimize_rules(content);
                write_in_file(&content);
                println!(
                    "✓ Successfully generated .gitignore with {} technologies",
                    techs.len()
                );
            }
        }
    }
}

fn interactive_select(rules: &GitignoreRules, prompt: &str) -> Vec<String> {
    println!("\n{}", prompt);
    let mut selected = Vec::new();

    for category in &rules.categories {
        let options: Vec<String> = category.items.iter().map(|s| s.to_string()).collect();

        match MultiSelect::new(&format!("{}:", category.name), options).prompt() {
            Ok(choices) => {
                selected.extend(choices);
            }
            Err(_) => {
                println!("Selection cancelled for {}", category.name);
            }
        }
    }

    if !selected.is_empty() {
        println!("\n📋 Selected: {}", selected.join(", "));
    } else {
        println!("\n⚠ No technologies selected");
    }

    selected
}

fn validate_technologies(technologies: &[String], rules: &GitignoreRules) -> Vec<String> {
    let mut valid = Vec::new();
    let mut invalid = Vec::new();

    for tech in technologies {
        if rules.is_valid(tech) {
            valid.push(tech.clone());
        } else {
            invalid.push(tech.clone());
        }
    }

    if !invalid.is_empty() {
        eprintln!("⚠ Invalid technologies: {}", invalid.join(", "));
        eprintln!("Run 'gig --help' to see available technologies");
    }

    valid
}

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

fn read_gitignore_rules() -> String {
    let path = Path::new(".gitignore");
    let mut content = String::new();

    if path.exists() {
        let file = File::open(path).unwrap_or_else(|e| {
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
    }

    content
}

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

fn remove_gitignore_rule(content: String, arg: &str, rules: &GitignoreRules) -> String {
    let mut lines_to_remove = HashSet::new();
    let comment_line = format!("# {}", arg);

    if let Some(tech_rules) = rules.get_rules(arg) {
        lines_to_remove.insert(comment_line);
        for rule in tech_rules {
            lines_to_remove.insert(rule.clone());
        }
    }

    content
        .lines()
        .filter(|line| !lines_to_remove.contains(&line.to_string()))
        .collect::<Vec<_>>()
        .join("\n")
}

fn optimize_rules(content: String) -> String {
    let mut unique_lines = HashSet::new();
    content
        .lines()
        .filter(|line| unique_lines.insert(line.to_string()))
        .collect::<Vec<_>>()
        .join("\n")
}

fn write_in_file(written_text: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(".gitignore")
        .unwrap_or_else(|e| {
            eprintln!("Error creating file: {}", e);
            std::process::exit(1);
        });

    file.write_all(written_text.as_bytes()).unwrap_or_else(|e| {
        eprintln!("Error writing to file: {}", e);
        std::process::exit(1);
    });
}
