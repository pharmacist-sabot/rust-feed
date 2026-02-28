use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

// Let's assume the script is always run from the root of `rust-feed`.
const ROOT_SRC_DIR: &str = "src";

#[derive(Debug)]
struct Category {
    name: &'static str,
    folder: &'static str,
}

const CATEGORIES: &[Category] = &[
    Category {
        name: "Case Study",
        folder: "case-study",
    },
    Category {
        name: "Deep Dive",
        folder: "deep-dive",
    },
    Category {
        name: "Rust Blockchain",
        folder: "rust-blockchain",
    },
    Category {
        name: "Rust Core",
        folder: "rust-core",
    },
    Category {
        name: "Rust Crates",
        folder: "rust-crates",
    },
    Category {
        name: "Rust Games",
        folder: "rust-games",
    },
    Category {
        name: "Rust Hacker",
        folder: "rust-hacker",
    },
    Category {
        name: "Rust Project",
        folder: "rust-project",
    },
    Category {
        name: "Rust Research",
        folder: "rust-research",
    },
    Category {
        name: "Rust Tools",
        folder: "rust-tools",
    },
    Category {
        name: "Rust Update",
        folder: "rust-update",
    },
    Category {
        name: "Rust Web",
        folder: "rust-web",
    },
];

fn extract_title(filepath: &Path) -> String {
    if let Ok(file) = fs::File::open(filepath) {
        let reader = io::BufReader::new(file);
        for line in reader.lines().map_while(Result::ok) {
            if let Some(stripped) = line.strip_prefix("# ") {
                return stripped.trim().to_string();
            }
        }
    }
    // Fallback to filename
    filepath
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned()
}

fn generate_index_content(category_name: &str, articles: &[(String, String)]) -> String {
    let mut content = format!("# {}\n\n", category_name);
    if articles.is_empty() {
        content.push_str("ยังไม่มีบทความในหมวดหมู่นี้\n");
    } else {
        for (title, filename) in articles {
            content.push_str(&format!("- [{title}](./{filename})\n"));
        }
    }
    content
}

type CategoriesData<'a> = &'a [(&'a str, &'a str, Vec<(String, String)>)];

fn generate_summary_content(categories_data: CategoriesData) -> String {
    let mut content = String::from(
        "<!-- markdownlint-disable MD025 -->\n\n# Summary\n\n[Introduction](./README.md)\n\n",
    );

    for (category_name, folder_name, articles) in categories_data {
        content.push_str(&format!("# {}\n\n", category_name));
        content.push_str(&format!("- [{category_name}](./{folder_name}/index.md)\n"));

        for (title, filename) in articles {
            content.push_str(&format!("  - [{title}](./{folder_name}/{filename})\n"));
        }
    }
    content
}

fn main() {
    let src_path = Path::new(ROOT_SRC_DIR);
    if !src_path.exists() {
        eprintln!(
            "Error: {} directory not found. Please run this command from the root of the project.",
            ROOT_SRC_DIR
        );
        std::process::exit(1);
    }

    let mut categories_data = Vec::new();

    for category in CATEGORIES {
        let folder_path = src_path.join(category.folder);

        // Create folder if it doesn't exist
        if !folder_path.exists() {
            if let Err(e) = fs::create_dir_all(&folder_path) {
                eprintln!(
                    "Failed to create directory {}: {}",
                    folder_path.display(),
                    e
                );
                continue;
            }
        }

        let mut articles = Vec::new();

        if let Ok(entries) = fs::read_dir(&folder_path) {
            let mut files: Vec<_> = entries.filter_map(|e| e.ok()).collect();
            // Sort files by name to ensure consistent ordering
            files.sort_by_key(|e| e.file_name());

            for entry in files {
                let path = entry.path();
                if path.is_file() {
                    let file_name = entry.file_name().to_string_lossy().into_owned();
                    if file_name.ends_with(".md") && file_name != "index.md" {
                        let title = extract_title(&path);
                        articles.push((title, file_name));
                    }
                }
            }
        }

        let index_content = generate_index_content(category.name, &articles);
        let index_path = folder_path.join("index.md");
        if let Err(e) = fs::write(&index_path, index_content) {
            eprintln!("Failed to write {}: {}", index_path.display(), e);
        }

        categories_data.push((category.name, category.folder, articles));
    }

    // Generate SUMMARY.md
    let summary_content = generate_summary_content(&categories_data);
    let summary_path = src_path.join("SUMMARY.md");
    if let Err(e) = fs::write(&summary_path, summary_content) {
        eprintln!("Failed to write {}: {}", summary_path.display(), e);
    } else {
        println!("Successfully generated index.md files and updated SUMMARY.md using Rust");
    }
}
