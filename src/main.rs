mod lexer;
mod parser;
mod ast;
mod values;
mod interpreter;
mod builtins;
mod goose;
mod web;

use clap::{Parser, Subcommand};
use std::fs;
use std::io::{self, Write, BufRead};
use std::path::PathBuf;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const REPO: &str = "konacodes/duck-lang";

#[derive(Parser)]
#[command(name = "goose")]
#[command(about = "The Goose interpreter for Duck-lang", long_about = None)]
#[command(version = VERSION)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a Duck file
    Run {
        /// The .duck file to run
        file: String,
        /// Arguments to pass to the Duck program (accessible via quack-args)
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Check a Duck file for quack issues without running
    Check {
        /// The .duck file to check
        file: String,
    },
    /// Start the interactive REPL
    Repl,
    /// Update goose to the latest version
    Update,
    /// Rollback to a specific version
    Rollback {
        /// Version to rollback to (e.g., v0.1.0)
        version: String,
    },
    /// List available versions
    Versions,
    /// Install a Duck library from GitHub
    Install {
        /// The library to install (format: user/repo)
        library: String,
        /// Version/branch to install (default: main)
        #[arg(default_value = "main")]
        version: String,
    },
    /// List installed libraries
    Libs,
    /// Get wisdom from the goose
    Wisdom,
    /// Watch a file and re-run on changes
    Watch {
        /// The .duck file to watch
        file: String,
        /// Arguments to pass to the Duck program
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Create a new Duck project
    New {
        /// Project name
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Update => update_goose(None),
        Commands::Rollback { version } => update_goose(Some(version)),
        Commands::Versions => list_versions(),
        Commands::Install { library, version } => install_library(&library, &version),
        Commands::Libs => list_libraries(),
        Commands::Wisdom => print_wisdom(),
        Commands::Watch { file, args } => watch_file(&file, args),
        Commands::New { name } => new_project(&name),
        _ => {
            // Print startup message for run/check/repl commands
            println!("{}", goose::startup());

            match cli.command {
                Commands::Run { file, args } => run_file(&file, args),
                Commands::Check { file } => check_file(&file),
                Commands::Repl => run_repl(),
                _ => unreachable!(),
            }
        }
    }
}

fn run_file(path: &str, args: Vec<String>) {
    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => {
            println!("I can't find that file. Are you sure it exists?");
            println!("   Geese have excellent eyesight, you know.");
            return;
        }
    };

    // Lex
    let tokens = match lexer::lex(&source) {
        Ok(t) => t,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    // Parse
    let mut parser = parser::Parser::new(tokens);
    let blocks = match parser.parse() {
        Ok(b) => b,
        Err(errors) => {
            for e in errors {
                println!("{}", e);
            }
            return;
        }
    };

    // Execute with command-line arguments
    let mut interpreter = interpreter::Interpreter::with_args(args);
    if let Err(e) = interpreter.run(blocks) {
        println!("{}", e);
    } else {
        println!("{}", goose::success());
    }

    // Always print rating at the end
    let (score, quip) = goose::rate_code(interpreter.stats());
    println!();
    println!("═══════════════════════════════════════");
    println!("  Goose rated your code: {}/10", score);
    println!("  \"{}\"", quip);
    println!("═══════════════════════════════════════");
}

fn check_file(path: &str) {
    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => {
            println!("I can't find that file. Are you sure it exists?");
            println!("   Geese have excellent eyesight, you know.");
            return;
        }
    };

    // Lex
    let tokens = match lexer::lex(&source) {
        Ok(t) => t,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    // Parse
    let mut parser = parser::Parser::new(tokens);
    let blocks = match parser.parse() {
        Ok(b) => b,
        Err(errors) => {
            for e in errors {
                println!("{}", e);
            }
            return;
        }
    };

    // Check for quack issues (blocks where was_quacked = false)
    let mut quack_issues = Vec::new();
    for block in &blocks {
        if !block.was_quacked {
            quack_issues.push(block.line);
        }
    }

    if quack_issues.is_empty() {
        println!("All blocks are properly quacked! Honk!");
        println!("   Your code passes the vibe check.");
    } else {
        println!("QUACK ALERT! The following lines are missing quack:");
        for line in &quack_issues {
            println!("   Line {}: No quack detected!", line);
        }
        println!();
        println!("Remember: Every block needs a quack to be valid.");
        println!("   {} issue(s) found.", quack_issues.len());
    }
}

fn run_repl() {
    println!("Welcome to the Goose REPL. Type 'exit' to leave.");
    println!("   Don't forget to quack!");
    println!();

    let stdin = io::stdin();
    let mut interpreter = interpreter::Interpreter::new();

    loop {
        print!("duck> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        if stdin.lock().read_line(&mut line).is_err() || line.trim() == "exit" {
            println!("Goodbye! *waddles away*");
            break;
        }

        if line.trim().is_empty() {
            continue;
        }

        // Lex the line
        let tokens = match lexer::lex(line.trim()) {
            Ok(t) => t,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        // Parse the line
        let mut parser = parser::Parser::new(tokens);
        let blocks = match parser.parse() {
            Ok(b) => b,
            Err(errors) => {
                for e in errors {
                    println!("{}", e);
                }
                continue;
            }
        };

        // Execute and provide goose commentary
        for block in blocks {
            match interpreter.run_block(block) {
                Ok(result) => {
                    if let Some(value) = result {
                        println!("=> {}", value);
                    }
                    // Goose comments on the line
                    println!("   {}", goose::repl_comment());
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
    }
}

// =============================================================================
// Update & Version Management
// =============================================================================

fn get_install_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("DUCK_INSTALL_DIR") {
        PathBuf::from(dir)
    } else {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".duck")
    }
}

fn print_goose_ascii() {
    println!();
    println!("                          ___");
    println!("                       .-'   `'.");
    println!("                      /         \\");
    println!("                      |         ;");
    println!("                      |         |           ___.--,");
    println!("                     |          |_.---._ .-'       `,");
    println!("                     /:        ./       ,'          ;");
    println!("                     \\':      :(        |           /");
    println!("                      \\':     :';       ;          /");
    println!("                       \\ \\    / ;      /    ____.--\\");
    println!("                        `.`._.' /    .'  .-\"        |");
    println!("                          `-...-`   /  .-'          /");
    println!("                                 .'  (            /");
    println!("                                /     `-.       .'");
    println!("                               /         `----'`");
    println!("                              (                  ");
    println!("                               `.               /");
    println!("                                 `-._________.-'");
    println!();
}

fn print_update_header() {
    println!("\x1b[36m");
    println!("   ____                        __  __          __      __     ");
    println!("  / ___| ___   ___  ___  ___  | |_| |_ __   __| | __ _| |_ ___");
    println!(" | |  _ / _ \\ / _ \\/ __|/ _ \\ | __| | '_ \\ / _` |/ _` | __/ _ \\");
    println!(" | |_| | (_) | (_) \\__ \\  __/ | |_| | |_) | (_| | (_| | ||  __/");
    println!("  \\____|\\___/ \\___/|___/\\___|  \\__|_| .__/ \\__,_|\\__,_|\\__\\___|");
    println!("                                   |_|                        ");
    println!("\x1b[0m");
}

fn animate_spinner(message: &str, duration_ms: u64) {
    let frames = ['|', '/', '-', '\\'];
    let iterations = duration_ms / 100;
    for i in 0..iterations {
        print!("\r\x1b[36m[{}]\x1b[0m {} ", frames[i as usize % 4], message);
        io::stdout().flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    println!();
}

fn update_goose(target_version: Option<String>) {
    print_update_header();
    print_goose_ascii();

    println!("\x1b[2mThe goose is checking for updates...\x1b[0m");
    println!();

    println!("\x1b[36m[*]\x1b[0m Current version: v{}", VERSION);

    // Determine target version
    let version = match &target_version {
        Some(v) => {
            let v = if v.starts_with('v') { v.clone() } else { format!("v{}", v) };
            println!("\x1b[36m[*]\x1b[0m Target version: {}", v);
            v
        }
        None => {
            animate_spinner("Fetching latest version...", 500);
            match fetch_latest_version() {
                Ok(v) => {
                    println!("\x1b[32m[+]\x1b[0m Latest version: {}", v);
                    v
                }
                Err(e) => {
                    println!("\x1b[31m[x]\x1b[0m Failed to fetch latest version: {}", e);
                    println!();
                    println!("The goose is displeased. Try again later.");
                    return;
                }
            }
        }
    };

    // Check if already on target version
    let current = format!("v{}", VERSION);
    if current == version && target_version.is_none() {
        println!();
        println!("\x1b[32m[+]\x1b[0m Already on the latest version!");
        println!();
        println!("\x1b[2m\"You're already running the finest code. I'm impressed. Barely.\"\x1b[0m");
        return;
    }

    // Detect platform
    let os = detect_os();
    let arch = detect_arch();
    println!("\x1b[36m[*]\x1b[0m Platform: {} ({})", os, arch);

    // Build download URL
    let filename = if os == "windows" {
        format!("goose-{}-{}.exe", os, arch)
    } else {
        format!("goose-{}-{}", os, arch)
    };
    let url = format!(
        "https://github.com/{}/releases/download/{}/{}",
        REPO, version, filename
    );

    println!("\x1b[36m[*]\x1b[0m Downloading from GitHub releases...");
    println!("\x1b[2m{}\x1b[0m", url);

    animate_spinner("Downloading binary...", 800);

    // Download the binary
    match download_binary(&url) {
        Ok(bytes) => {
            println!("\x1b[32m[+]\x1b[0m Download complete ({} bytes)", bytes.len());

            // Get install location
            let install_dir = get_install_dir();
            let bin_dir = install_dir.join("bin");
            let goose_path = bin_dir.join("goose");

            // Create backup of current binary
            if goose_path.exists() {
                let backup_path = bin_dir.join(format!("goose.{}.bak", VERSION));
                if let Err(e) = fs::copy(&goose_path, &backup_path) {
                    println!("\x1b[33m[!]\x1b[0m Could not create backup: {}", e);
                } else {
                    println!("\x1b[32m[+]\x1b[0m Backed up current version to goose.{}.bak", VERSION);
                }
            }

            // Create directories if needed
            if let Err(e) = fs::create_dir_all(&bin_dir) {
                println!("\x1b[31m[x]\x1b[0m Failed to create directory: {}", e);
                return;
            }

            // Remove old binary first (required on Unix - can't overwrite running executable)
            // This works because the running process keeps the inode, but the filename is freed
            if goose_path.exists() {
                if let Err(e) = fs::remove_file(&goose_path) {
                    println!("\x1b[31m[x]\x1b[0m Failed to remove old binary: {}", e);
                    return;
                }
            }

            // Write new binary
            if let Err(e) = fs::write(&goose_path, &bytes) {
                println!("\x1b[31m[x]\x1b[0m Failed to write binary: {}", e);
                return;
            }

            // Make executable on Unix
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Err(e) = fs::set_permissions(&goose_path, fs::Permissions::from_mode(0o755)) {
                    println!("\x1b[33m[!]\x1b[0m Could not set permissions: {}", e);
                }
            }

            // Save version info
            let version_file = install_dir.join(".version");
            let _ = fs::write(&version_file, &version);

            println!();
            println!("\x1b[32m   ___ _   _  ___ ___ ___  ___ ___ \x1b[0m");
            println!("\x1b[32m  / __| | | |/ __/ __/ _ \\/ __/ __|\x1b[0m");
            println!("\x1b[32m  \\__ \\ |_| | (_| (_|  __/\\__ \\__ \\\x1b[0m");
            println!("\x1b[32m  |___/\\__,_|\\___\\___\\___||___/___/\x1b[0m");
            println!();
            println!("\x1b[1mGoose has been updated to {}!\x1b[0m", version);
            println!();
            println!("  Location: {}", goose_path.display());
            println!();
            println!("\x1b[2m\"Another version, another chance for your code to disappoint me.\"\x1b[0m");
        }
        Err(e) => {
            println!("\x1b[31m[x]\x1b[0m Download failed: {}", e);
            println!();
            println!("The goose could not fetch the binary.");
            println!("Make sure the version exists: {}", version);
        }
    }
}

fn list_versions() {
    print_update_header();

    println!("\x1b[36m[*]\x1b[0m Current version: v{}", VERSION);
    println!();

    animate_spinner("Fetching available versions...", 600);

    match fetch_versions() {
        Ok(versions) => {
            println!("\x1b[32m[+]\x1b[0m Available versions:");
            println!();

            for (i, v) in versions.iter().take(10).enumerate() {
                let marker = if v == &format!("v{}", VERSION) {
                    " <-- current"
                } else {
                    ""
                };
                println!("    {}. {}{}", i + 1, v, marker);
            }

            if versions.len() > 10 {
                println!("    ... and {} more", versions.len() - 10);
            }

            println!();
            println!("To rollback: goose rollback <version>");
            println!("To update:   goose update");
        }
        Err(e) => {
            println!("\x1b[31m[x]\x1b[0m Failed to fetch versions: {}", e);
        }
    }
}

fn detect_os() -> &'static str {
    #[cfg(target_os = "linux")]
    return "linux";
    #[cfg(target_os = "macos")]
    return "macos";
    #[cfg(target_os = "windows")]
    return "windows";
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    return "unknown";
}

fn detect_arch() -> &'static str {
    #[cfg(target_arch = "x86_64")]
    return "x86_64";
    #[cfg(target_arch = "aarch64")]
    return "aarch64";
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    return "unknown";
}

fn fetch_latest_version() -> Result<String, String> {
    let url = format!("https://api.github.com/repos/{}/releases/latest", REPO);

    let client = reqwest::blocking::Client::builder()
        .user_agent("goose-updater")
        .build()
        .map_err(|e| e.to_string())?;

    let response = client
        .get(&url)
        .send()
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err(format!("HTTP {}", response.status()));
    }

    let json: serde_json::Value = response.json().map_err(|e| e.to_string())?;

    json["tag_name"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| "No tag_name in response".to_string())
}

fn fetch_versions() -> Result<Vec<String>, String> {
    let url = format!("https://api.github.com/repos/{}/releases", REPO);

    let client = reqwest::blocking::Client::builder()
        .user_agent("goose-updater")
        .build()
        .map_err(|e| e.to_string())?;

    let response = client
        .get(&url)
        .send()
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err(format!("HTTP {}", response.status()));
    }

    let json: serde_json::Value = response.json().map_err(|e| e.to_string())?;

    let versions: Vec<String> = json
        .as_array()
        .ok_or_else(|| "Expected array".to_string())?
        .iter()
        .filter_map(|release| release["tag_name"].as_str().map(|s| s.to_string()))
        .collect();

    Ok(versions)
}

fn download_binary(url: &str) -> Result<Vec<u8>, String> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("goose-updater")
        .build()
        .map_err(|e| e.to_string())?;

    let response = client
        .get(url)
        .send()
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err(format!("HTTP {}", response.status()));
    }

    response.bytes().map(|b| b.to_vec()).map_err(|e| e.to_string())
}

// =============================================================================
// Library Management
// =============================================================================

fn get_libs_dir() -> PathBuf {
    get_install_dir().join("libs")
}

fn print_install_header() {
    println!("\x1b[36m");
    println!("   ____                         _____           _        _ _ ");
    println!("  / ___| ___   ___  ___  ___   |_   _|         | |      | | |");
    println!(" | |  _ / _ \\ / _ \\/ __|/ _ \\    | |  _ __  ___| |_ __ _| | |");
    println!(" | |_| | (_) | (_) \\__ \\  __/    | | | '_ \\/ __| __/ _` | | |");
    println!("  \\____|\\___/ \\___/|___/\\___|   |___/| | | \\__ \\ || (_| | | |");
    println!("                                     |_| |_|___/\\__\\__,_|_|_|");
    println!("\x1b[0m");
}

fn install_library(library: &str, version: &str) {
    print_install_header();

    // Parse library format: user/repo
    let parts: Vec<&str> = library.split('/').collect();
    if parts.len() != 2 {
        println!("\x1b[31m[x]\x1b[0m Invalid library format. Use: user/repo");
        println!("    Example: goose install konacodes/discord v0.1.0");
        return;
    }

    let user = parts[0];
    let repo = parts[1];

    println!("\x1b[36m[*]\x1b[0m Installing {} @ {}", library, version);
    println!();

    // Create libs directory
    let libs_dir = get_libs_dir();
    let lib_path = libs_dir.join(user).join(repo).join(version);

    if lib_path.exists() {
        println!("\x1b[33m[!]\x1b[0m Library already installed at:");
        println!("    {}", lib_path.display());
        println!();
        println!("To reinstall, remove the directory first:");
        println!("    rm -rf \"{}\"", lib_path.display());
        return;
    }

    if let Err(e) = fs::create_dir_all(&lib_path) {
        println!("\x1b[31m[x]\x1b[0m Failed to create directory: {}", e);
        return;
    }

    // Clone the repository
    let git_url = format!("https://github.com/{}/{}.git", user, repo);
    println!("\x1b[36m[*]\x1b[0m Cloning from GitHub...");
    println!("\x1b[2m{}\x1b[0m", git_url);
    println!();

    animate_spinner("Fetching library...", 500);

    // Use git clone with depth 1 for faster cloning
    let output = std::process::Command::new("git")
        .args(["clone", "--depth", "1", "--branch", version, &git_url, lib_path.to_str().unwrap()])
        .output();

    match output {
        Ok(result) => {
            if result.status.success() {
                // Check for metadata.dm
                let metadata_path = lib_path.join("metadata.dm");
                let mut dependencies: Vec<(String, String)> = Vec::new();

                if metadata_path.exists() {
                    println!("\x1b[32m[+]\x1b[0m Found metadata.dm");

                    // Parse metadata to show info and collect dependencies
                    if let Ok(metadata) = fs::read_to_string(&metadata_path) {
                        let mut in_dependencies = false;

                        for line in metadata.lines() {
                            let line = line.trim();

                            // Check for section headers
                            if line.starts_with('[') && line.ends_with(']') {
                                in_dependencies = line == "[dependencies]";
                                continue;
                            }

                            if line.starts_with("description:") {
                                let desc = line.trim_start_matches("description:").trim().trim_matches('\'');
                                println!("\x1b[2m    {}\x1b[0m", desc);
                            }

                            // Parse dependency lines (format: user/repo vX.Y.Z)
                            if in_dependencies && !line.is_empty() && !line.starts_with("--") {
                                let parts: Vec<&str> = line.split_whitespace().collect();
                                if parts.len() >= 2 {
                                    dependencies.push((parts[0].to_string(), parts[1].to_string()));
                                }
                            }
                        }
                    }
                } else {
                    println!("\x1b[33m[!]\x1b[0m No metadata.dm found - using default lib.duck");
                }

                // Install dependencies if any
                if !dependencies.is_empty() {
                    println!();
                    println!("\x1b[36m[*]\x1b[0m Installing {} dependencies...", dependencies.len());
                    for (dep_lib, dep_version) in &dependencies {
                        println!("\x1b[2m    -> {} @ {}\x1b[0m", dep_lib, dep_version);
                    }
                    println!();

                    for (dep_lib, dep_version) in dependencies {
                        // Check if dependency is already installed
                        let dep_parts: Vec<&str> = dep_lib.split('/').collect();
                        if dep_parts.len() == 2 {
                            let dep_path = get_libs_dir()
                                .join(dep_parts[0])
                                .join(dep_parts[1])
                                .join(&dep_version);

                            if !dep_path.exists() {
                                install_library(&dep_lib, &dep_version);
                            } else {
                                println!("\x1b[32m[+]\x1b[0m Dependency {} @ {} already installed", dep_lib, dep_version);
                            }
                        }
                    }
                    println!();
                }

                println!();
                println!("\x1b[32m   ___ _   _  ___ ___ ___  ___ ___ \x1b[0m");
                println!("\x1b[32m  / __| | | |/ __/ __/ _ \\/ __/ __|\x1b[0m");
                println!("\x1b[32m  \\__ \\ |_| | (_| (_|  __/\\__ \\__ \\\x1b[0m");
                println!("\x1b[32m  |___/\\__,_|\\___\\___\\___||___/___/\x1b[0m");
                println!();
                println!("\x1b[1mLibrary installed successfully!\x1b[0m");
                println!();
                println!("  Location: {}", lib_path.display());
                println!();
                println!("Usage in your Duck code:");
                println!("  \x1b[33mquack [migrate \"git+{}/{}\" as {}]\x1b[0m", user, repo, repo);
                println!();
                println!("\x1b[2m\"Another library to ignore. How delightful.\"\x1b[0m");
            } else {
                let stderr = String::from_utf8_lossy(&result.stderr);
                if stderr.contains("not find remote branch") || stderr.contains("Could not find remote branch") {
                    println!("\x1b[31m[x]\x1b[0m Branch/version '{}' not found", version);
                    println!("    Try: goose install {} main", library);
                } else {
                    println!("\x1b[31m[x]\x1b[0m Failed to clone repository");
                    println!("\x1b[2m{}\x1b[0m", stderr);
                }
                // Clean up failed directory
                let _ = fs::remove_dir_all(&lib_path);
            }
        }
        Err(e) => {
            println!("\x1b[31m[x]\x1b[0m Failed to run git: {}", e);
            println!("    Make sure git is installed and in your PATH");
        }
    }
}

fn list_libraries() {
    println!();
    println!("\x1b[36m[*]\x1b[0m Installed Duck Libraries");
    println!("\x1b[36m{}\x1b[0m", "=".repeat(40));
    println!();

    let libs_dir = get_libs_dir();

    if !libs_dir.exists() {
        println!("  \x1b[2mNo libraries installed yet.\x1b[0m");
        println!();
        println!("  Install one with:");
        println!("    goose install user/repo version");
        println!();
        return;
    }

    let mut found_any = false;

    // Iterate through user directories
    if let Ok(users) = fs::read_dir(&libs_dir) {
        for user_entry in users.flatten() {
            if !user_entry.path().is_dir() {
                continue;
            }
            let user_name = user_entry.file_name().to_string_lossy().to_string();

            // Iterate through repo directories
            if let Ok(repos) = fs::read_dir(user_entry.path()) {
                for repo_entry in repos.flatten() {
                    if !repo_entry.path().is_dir() {
                        continue;
                    }
                    let repo_name = repo_entry.file_name().to_string_lossy().to_string();

                    // Iterate through version directories
                    if let Ok(versions) = fs::read_dir(repo_entry.path()) {
                        for version_entry in versions.flatten() {
                            if !version_entry.path().is_dir() {
                                continue;
                            }
                            let version = version_entry.file_name().to_string_lossy().to_string();

                            found_any = true;
                            println!("  \x1b[32m{}/{}\x1b[0m @ \x1b[33m{}\x1b[0m", user_name, repo_name, version);

                            // Try to read description from metadata.dm
                            let metadata_path = version_entry.path().join("metadata.dm");
                            if let Ok(metadata) = fs::read_to_string(metadata_path) {
                                for line in metadata.lines() {
                                    let line = line.trim();
                                    if line.starts_with("description:") {
                                        let desc = line.trim_start_matches("description:").trim().trim_matches('\'');
                                        println!("    \x1b[2m{}\x1b[0m", desc);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if !found_any {
        println!("  \x1b[2mNo libraries installed yet.\x1b[0m");
        println!();
        println!("  Install one with:");
        println!("    goose install user/repo version");
    }

    println!();
}

fn print_wisdom() {
    use std::time::{SystemTime, UNIX_EPOCH};

    // Embed wisdom at compile time
    const WISDOM: &str = include_str!("../assets/wisdom.txt");

    // Parse quotes (skip comments and empty lines)
    let quotes: Vec<&str> = WISDOM
        .lines()
        .filter(|line| !line.starts_with('#') && !line.trim().is_empty())
        .collect();

    // Pick a random quote using time-based randomness
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let seed = duration.subsec_nanos() as usize;
    let idx = seed % quotes.len();
    let quote = quotes[idx];

    // Print with goose flair
    let emojis = ["🪿", "🦆", "🦢", ">o)", "~(o>"];
    let emoji = emojis[seed % emojis.len()];

    println!();
    println!("  {} \x1b[3m\"{}\"\x1b[0m", emoji, quote);
    println!();
    println!("    \x1b[2m— The Goose\x1b[0m");
    println!();
}

fn watch_file(path: &str, args: Vec<String>) {
    use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher, Event, EventKind};
    use std::sync::mpsc::channel;
    use std::time::{Duration, Instant};
    use std::path::Path;

    let path = Path::new(path);
    if !path.exists() {
        println!("\x1b[31m✗\x1b[0m File not found: {}", path.display());
        return;
    }

    let file_name = path.file_name().unwrap_or_default().to_string_lossy();
    let parent = path.parent().unwrap_or(Path::new("."));
    let canonical_path = path.canonicalize().unwrap_or(path.to_path_buf());

    // Clear screen and print header
    fn print_header(file_name: &str, run_count: usize) {
        print!("\x1b[2J\x1b[H"); // Clear screen and move cursor to top
        println!();
        println!("  \x1b[36m┌─────────────────────────────────────────┐\x1b[0m");
        println!("  \x1b[36m│\x1b[0m  🪿 \x1b[1mGOOSE WATCH\x1b[0m                         \x1b[36m│\x1b[0m");
        println!("  \x1b[36m│\x1b[0m  \x1b[2mWatching for changes...\x1b[0m               \x1b[36m│\x1b[0m");
        println!("  \x1b[36m└─────────────────────────────────────────┘\x1b[0m");
        println!();
        println!("  \x1b[33m▶\x1b[0m \x1b[1m{}\x1b[0m \x1b[2m(run #{})\x1b[0m", file_name, run_count);
        println!("  \x1b[2m{}\x1b[0m", "─".repeat(43));
        println!();
    }

    fn print_reload_banner(file_name: &str) {
        println!();
        println!("  \x1b[2m{}\x1b[0m", "─".repeat(43));
        println!("  \x1b[35m⟳\x1b[0m \x1b[1mChange detected!\x1b[0m Reloading \x1b[33m{}\x1b[0m...", file_name);
        println!();
        std::thread::sleep(Duration::from_millis(100));
    }

    fn run_duck_file(path: &Path, args: &[String]) -> bool {
        let source = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                println!("  \x1b[31m✗ Error reading file:\x1b[0m {}", e);
                return false;
            }
        };

        // Lex
        let tokens = match crate::lexer::lex(&source) {
            Ok(t) => t,
            Err(e) => {
                println!("  \x1b[31m✗ Lexer error:\x1b[0m {}", e);
                return false;
            }
        };

        // Parse
        let mut parser = crate::parser::Parser::new(tokens);
        let blocks = match parser.parse() {
            Ok(b) => b,
            Err(errors) => {
                for e in errors {
                    println!("  \x1b[31m✗ Parser error:\x1b[0m {}", e);
                }
                return false;
            }
        };

        // Run
        let mut interp = crate::interpreter::Interpreter::with_args(args.to_vec());

        match interp.run(blocks) {
            Ok(_) => true,
            Err(e) => {
                println!("  \x1b[31m✗ Runtime error:\x1b[0m {}", e);
                false
            }
        }
    }

    // Set up file watcher
    let (tx, rx) = channel();
    let mut watcher = match RecommendedWatcher::new(
        move |res: Result<Event, _>| {
            if let Ok(event) = res {
                let _ = tx.send(event);
            }
        },
        Config::default(),
    ) {
        Ok(w) => w,
        Err(e) => {
            println!("\x1b[31m✗\x1b[0m Failed to create watcher: {}", e);
            return;
        }
    };

    if let Err(e) = watcher.watch(parent, RecursiveMode::NonRecursive) {
        println!("\x1b[31m✗\x1b[0m Failed to watch directory: {}", e);
        return;
    }

    let mut run_count = 1;
    let mut last_run = Instant::now();
    let debounce = Duration::from_millis(200);

    // Initial run
    print_header(&file_name, run_count);
    let start = Instant::now();
    let success = run_duck_file(path, &args);
    let elapsed = start.elapsed();

    println!();
    if success {
        println!("  \x1b[32m✓\x1b[0m \x1b[2mCompleted in {:.2?}\x1b[0m", elapsed);
    }
    println!();
    println!("  \x1b[2mPress Ctrl+C to stop watching\x1b[0m");

    // Watch loop
    loop {
        match rx.recv() {
            Ok(event) => {
                // Check if this event is for our file
                let dominated = event.paths.iter().any(|p| {
                    p.canonicalize().unwrap_or(p.clone()) == canonical_path
                });

                if !dominated {
                    continue;
                }

                // Only react to modify events
                if !matches!(event.kind, EventKind::Modify(_)) {
                    continue;
                }

                // Debounce rapid events
                if last_run.elapsed() < debounce {
                    continue;
                }
                last_run = Instant::now();

                run_count += 1;
                print_reload_banner(&file_name);
                print_header(&file_name, run_count);

                let start = Instant::now();
                let success = run_duck_file(path, &args);
                let elapsed = start.elapsed();

                println!();
                if success {
                    println!("  \x1b[32m✓\x1b[0m \x1b[2mCompleted in {:.2?}\x1b[0m", elapsed);
                }
                println!();
                println!("  \x1b[2mPress Ctrl+C to stop watching\x1b[0m");
            }
            Err(_) => break,
        }
    }
}

fn new_project(name: &str) {
    use std::path::Path;

    let project_dir = Path::new(name);

    // Check if directory already exists
    if project_dir.exists() {
        println!("\x1b[31m✗\x1b[0m Directory '{}' already exists!", name);
        return;
    }

    // Create project structure
    println!();
    println!("  🪿 \x1b[1mCreating new Duck project:\x1b[0m {}", name);
    println!();

    // Create directories
    if let Err(e) = fs::create_dir_all(project_dir.join("src")) {
        println!("\x1b[31m✗\x1b[0m Failed to create directory: {}", e);
        return;
    }
    println!("  \x1b[32m✓\x1b[0m Created {}/", name);
    println!("  \x1b[32m✓\x1b[0m Created {}/src/", name);

    // Create main.duck
    let main_content = format!(r#"-- {} - A Duck Project
-- Run with: goose run src/main.duck

quack [print "Hello from {}!"]
quack [print ""]
quack [print "Welcome to your new Duck project."]
quack [print "Start coding and don't forget to quack!"]
"#, name, name);

    if let Err(e) = fs::write(project_dir.join("src/main.duck"), main_content) {
        println!("\x1b[31m✗\x1b[0m Failed to create main.duck: {}", e);
        return;
    }
    println!("  \x1b[32m✓\x1b[0m Created {}/src/main.duck", name);

    // Create README.md
    let readme_content = format!(r#"# {}

A Duck project. The goose approves.

## Run

```bash
goose run src/main.duck
```

## Watch (auto-reload on changes)

```bash
goose watch src/main.duck
```

## About Duck

Duck is a programming language where every code block must be preceded by `quack` to execute.
The interpreter is named Goose. They have a complicated relationship.

Learn more: https://github.com/konacodes/duck-lang
"#, name);

    if let Err(e) = fs::write(project_dir.join("README.md"), readme_content) {
        println!("\x1b[31m✗\x1b[0m Failed to create README.md: {}", e);
        return;
    }
    println!("  \x1b[32m✓\x1b[0m Created {}/README.md", name);

    // Create .gitignore
    let gitignore_content = "# Duck project ignores\n*.log\n.DS_Store\n";
    if let Err(e) = fs::write(project_dir.join(".gitignore"), gitignore_content) {
        println!("\x1b[31m✗\x1b[0m Failed to create .gitignore: {}", e);
        return;
    }
    println!("  \x1b[32m✓\x1b[0m Created {}/.gitignore", name);

    println!();
    println!("  \x1b[36m┌─────────────────────────────────────────┐\x1b[0m");
    println!("  \x1b[36m│\x1b[0m  \x1b[32m✓ Project created successfully!\x1b[0m       \x1b[36m│\x1b[0m");
    println!("  \x1b[36m└─────────────────────────────────────────┘\x1b[0m");
    println!();
    println!("  Next steps:");
    println!("    \x1b[33mcd {}\x1b[0m", name);
    println!("    \x1b[33mgoose run src/main.duck\x1b[0m");
    println!();
    println!("  Or watch for changes:");
    println!("    \x1b[33mgoose watch src/main.duck\x1b[0m");
    println!();
    println!("  Happy quacking! 🦆");
    println!();
}
