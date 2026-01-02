mod lexer;
mod parser;
mod ast;
mod values;
mod interpreter;
mod builtins;
mod goose;

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
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Update => update_goose(None),
        Commands::Rollback { version } => update_goose(Some(version)),
        Commands::Versions => list_versions(),
        Commands::Install { library, version } => install_library(&library, &version),
        Commands::Libs => list_libraries(),
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
                if metadata_path.exists() {
                    println!("\x1b[32m[+]\x1b[0m Found metadata.dm");

                    // Parse metadata to show info
                    if let Ok(metadata) = fs::read_to_string(&metadata_path) {
                        for line in metadata.lines() {
                            let line = line.trim();
                            if line.starts_with("description:") {
                                let desc = line.trim_start_matches("description:").trim().trim_matches('\'');
                                println!("\x1b[2m    {}\x1b[0m", desc);
                            }
                        }
                    }
                } else {
                    println!("\x1b[33m[!]\x1b[0m No metadata.dm found - using default lib.duck");
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
