// apps/smith-context/src/main.rs
use anyhow::Result;
use chrono::Local;
use clap::Parser;
use ignore::gitignore::GitignoreBuilder;
use std::collections::{BTreeMap, HashMap};
use std::fmt::Write;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Smith Context Builder
///
/// Собирает весь контекст проекта в один Markdown-файл для передачи LLM.
#[derive(Parser)]
#[command(name = "smith-context")]
#[command(about = "Collect project context into a single Markdown file")]
#[allow(clippy::struct_excessive_bools)] // CLI flags are inherently boolean
struct Cli {
    /// Root directory of the project
    #[arg(short, long, default_value = ".")]
    root: PathBuf,

    /// Output file path
    #[arg(short, long, default_value = "CONTEXT.md")]
    output: PathBuf,

    /// Include git log (last 20 commits)
    #[arg(long)]
    git_log: bool,

    /// Include dependency graph via cargo metadata (Mermaid)
    #[arg(long)]
    graph: bool,

    /// Include graphify-rs artifacts (`graph.json`, `GRAPH_REPORT.md`) if present
    #[arg(long)]
    graphify: bool,

    /// Directory containing graphify-rs artifacts (default: smith-graphify)
    #[arg(long, default_value = DEFAULT_GRAPHIFY_DIR)]
    graphify_dir: String,

    /// Skip graphify-rs build (use existing artifacts)
    #[arg(long)]
    no_graphify_build: bool,

    /// Max file size in bytes (default: 1MB)
    #[arg(long, default_value = "1048576")]
    max_file_size: u64,

    /// Exclude patterns (substring match)
    #[arg(long)]
    exclude: Vec<String>,
}

/// Файлы, которые всегда исключаются из сбора
const ALWAYS_EXCLUDE_FILES: &[&str] =
    &["PROJECT_GUIDE.md", "CONTEXT.md", "graph.html", "Cargo.lock"];

/// Файлы, которые всегда исключаются по директориям
const ALWAYS_EXCLUDE_DIRS: &[&str] = &[
    "target",
    "node_modules",
    "venv",
    "__pycache__",
    ".git",
    ".idea",
    ".vscode",
    "smith-graphify",
];

/// Директория по умолчанию для артефактов graphify-rs
const DEFAULT_GRAPHIFY_DIR: &str = "smith-graphify";

/// Graphify-rs артефакты
struct GraphifyArtifacts {
    graph_json: Option<String>,
    graph_report: Option<String>,
}

/// Представление одного файла проекта
struct FileEntry {
    relative_path: String,
    content: String,
    language: String,
    size: u64,
}

/// Статистика проекта
struct ProjectStats {
    total_files: usize,
    total_lines: usize,
    total_bytes: usize,
    crates: Vec<String>,
    languages: HashMap<String, usize>,
}

/// Узел дерева файлов
struct TreeNode {
    name: String,
    children: BTreeMap<String, TreeNode>,
    is_file: bool,
}

/// Контекст для генерации Markdown-отчёта.
/// Инкапсулирует все данные, необходимые для форматирования,
/// чтобы избежать избыточного количества аргументов функции.
struct MarkdownContext<'a> {
    tree: &'a str,
    files: &'a [FileEntry],
    stats: &'a ProjectStats,
    env_info: &'a str,
    workspace_cargo: Option<&'a str>,
    graph: Option<&'a str>,
    graphify: Option<&'a GraphifyArtifacts>,
    git_log: Option<&'a str>,
    todos: &'a [(String, usize, String)],
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    println!("🔍 Collecting project context from: {}", cli.root.display());

    // 1. Собираем файлы
    let files = collect_files(&cli)?;
    println!("   📄 Found {} files", files.len());

    // 2. Строим дерево
    let tree = build_tree(&files);

    // 3. Считаем статистику
    let stats = build_stats(&files);

    // 4. Собираем TODO/FIXME
    let todos = collect_todos(&files);

    // 5. Опционально: граф зависимостей через cargo metadata
    let graph = if cli.graph {
        println!("   🔗 Building dependency graph (cargo metadata)...");
        Some(build_graph(&cli.root)?)
    } else {
        None
    };

    // 6. Опционально: graphify-rs артефакты
    let graphify = if cli.graphify {
        // 6a. Сначала запускаем сборку (если не пропущено)
        if cli.no_graphify_build {
            println!("   ⏭️  Skipping graphify-rs build (--no-graphify-build)");
        } else {
            let build_ok = run_graphify_build(&cli.root, &cli.graphify_dir);
            if !build_ok {
                println!("      ⚠ Continuing with existing artifacts (if any)");
            }
        }

        // 6b. Загружаем артефакты
        println!(
            "   🧠 Loading graphify-rs artifacts from: {}",
            cli.graphify_dir
        );
        Some(load_graphify_artifacts(&cli.root, &cli.graphify_dir)?)
    } else {
        None
    };

    // 7. Опционально: git log
    let git_log = if cli.git_log {
        println!("   📜 Fetching git log...");
        Some(build_git_log(&cli.root)?)
    } else {
        None
    };

    // 8. Environment info
    let env_info = build_env_info()?;

    // 9. Workspace Cargo.toml
    let workspace_cargo = read_workspace_cargo(&cli.root)?;

    // 10. Форматируем в Markdown
    let md_ctx = MarkdownContext {
        tree: &tree,
        files: &files,
        stats: &stats,
        env_info: &env_info,
        workspace_cargo: workspace_cargo.as_deref(),
        graph: graph.as_deref(),
        graphify: graphify.as_ref(),
        git_log: git_log.as_deref(),
        todos: &todos,
    };
    let markdown = format_markdown(&md_ctx);

    // 11. Записываем в файл
    std::fs::write(&cli.output, &markdown)?;

    println!("\n✅ Context collected to: {}", cli.output.display());
    println!("   📊 Files:  {}", stats.total_files);
    println!("   📝 Lines:  {}", stats.total_lines);
    println!(
        "   💾 Size:   {} bytes ({} KB)",
        markdown.len(),
        markdown.len() / 1024
    );
    println!("   📦 Crates: {}", stats.crates.len());

    Ok(())
}

/// Проверяет, является ли файл в списке всегда исключаемых
fn is_always_excluded_file(path: &Path) -> bool {
    let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    ALWAYS_EXCLUDE_FILES.contains(&file_name)
}

/// Проверяет, является ли директория всегда исключаемой
fn is_always_excluded_dir(name: &str) -> bool {
    ALWAYS_EXCLUDE_DIRS.contains(&name)
}

/// Рекурсивно собирает все текстовые файлы проекта
fn collect_files(cli: &Cli) -> Result<Vec<FileEntry>> {
    // Строим gitignore
    let gitignore_path = cli.root.join(".gitignore");
    let mut builder = GitignoreBuilder::new(&cli.root);
    if gitignore_path.exists() {
        builder.add(gitignore_path);
    }
    let gitignore = builder.build()?;

    let mut files = Vec::new();

    for entry in WalkDir::new(&cli.root).into_iter().filter_entry(|e| {
        let path = e.path();
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        // Пропускаем скрытые директории (кроме корня)
        if name.starts_with('.') && name != "." {
            return false;
        }

        // Пропускаем стандартные build-директории
        if is_always_excluded_dir(name) {
            return false;
        }

        // Пропускаем пользовательские excludes
        for pattern in &cli.exclude {
            if path.to_str().is_some_and(|p| p.contains(pattern)) {
                return false;
            }
        }

        true
    }) {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        // Пропускаем всегда исключаемые файлы
        if is_always_excluded_file(path) {
            continue;
        }

        // Проверяем gitignore
        if gitignore.matched(path, false).is_ignore() {
            continue;
        }

        // Проверяем размер файла
        let metadata = entry.metadata()?;
        if metadata.len() > cli.max_file_size {
            continue;
        }

        // Пропускаем бинарные файлы
        let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if is_binary_extension(extension) {
            continue;
        }

        // Читаем содержимое (пропускаем если не UTF-8)
        let Ok(content) = std::fs::read_to_string(path) else {
            continue;
        };

        // Нормализуем путь (Windows backslashes → forward slashes)
        let relative_path = path
            .strip_prefix(&cli.root)?
            .to_string_lossy()
            .replace('\\', "/");

        let language = detect_language(path).to_string();

        files.push(FileEntry {
            relative_path,
            content,
            language,
            size: metadata.len(),
        });
    }

    // Сортируем для консистентного вывода
    files.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));

    Ok(files)
}

/// Определяет язык программирования по расширению файла
fn detect_language(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("rs") => "rust",
        Some("toml" | "lock") => "toml",
        Some("yaml" | "yml") => "yaml",
        Some("json") => "json",
        Some("md") => "markdown",
        Some("sh" | "bash") => "bash",
        Some("ps1") => "powershell",
        Some("py") => "python",
        Some("js") => "javascript",
        Some("ts") => "typescript",
        Some("html") => "html",
        Some("css") => "css",
        Some("sql") => "sql",
        Some("xml") => "xml",
        Some("ini" | "cfg") => "ini",
        Some("dockerfile") => "dockerfile",
        Some("gitignore") => "gitignore",
        _ => "text",
    }
}

/// Проверяет, является ли расширение бинарным
fn is_binary_extension(ext: &str) -> bool {
    matches!(
        ext,
        // Исполняемые файлы
        "exe"
            | "dll"
            | "so"
            | "dylib"
            | "bin"
            | "obj"
            | "o"
            | "a"
            | "lib"
            | "pdb"
            // Изображения
            | "png"
            | "jpg"
            | "jpeg"
            | "gif"
            | "bmp"
            | "ico"
            | "svg"
            | "webp"
            | "tiff"
            // Аудио/видео
            | "mp3"
            | "mp4"
            | "avi"
            | "mov"
            | "wav"
            | "flac"
            | "mkv"
            | "webm"
            // Архивы
            | "zip"
            | "tar"
            | "gz"
            | "bz2"
            | "7z"
            | "rar"
            | "xz"
            // Документы
            | "pdf"
            | "doc"
            | "docx"
            | "xls"
            | "xlsx"
            | "ppt"
            | "pptx"
            // WASM
            | "wasm"
    )
}

/// Строит ASCII-дерево файлов
fn build_tree(files: &[FileEntry]) -> String {
    let mut root = TreeNode {
        name: ".".to_string(),
        children: BTreeMap::new(),
        is_file: false,
    };

    // Строим дерево из путей
    for file in files {
        let parts: Vec<&str> = file.relative_path.split('/').collect();
        let mut current = &mut root;

        for (i, part) in parts.iter().enumerate() {
            let is_last = i == parts.len() - 1;
            current = current
                .children
                .entry(part.to_string())
                .or_insert_with(|| TreeNode {
                    name: part.to_string(),
                    children: BTreeMap::new(),
                    is_file: is_last,
                });
        }
    }

    // Рендерим дерево в строку
    let mut output = String::new();
    output.push_str("```\n");
    output.push_str(".\n");
    render_tree(&root, "", &mut output);
    output.push_str("```\n");

    output
}

/// Рекурсивно рендерит узел дерева
fn render_tree(node: &TreeNode, prefix: &str, output: &mut String) {
    let children: Vec<_> = node.children.values().collect();
    for (i, child) in children.iter().enumerate() {
        let is_last = i == children.len() - 1;
        let connector = if is_last { "└── " } else { "├── " };
        let _ = writeln!(output, "{prefix}{connector}{}", child.name);

        if !child.is_file {
            let new_prefix = if is_last {
                format!("{prefix}    ")
            } else {
                format!("{prefix}│   ")
            };
            render_tree(child, &new_prefix, output);
        }
    }
}

/// Считает статистику проекта
fn build_stats(files: &[FileEntry]) -> ProjectStats {
    let mut stats = ProjectStats {
        total_files: files.len(),
        total_lines: 0,
        total_bytes: 0,
        crates: Vec::new(),
        languages: HashMap::new(),
    };

    for file in files {
        stats.total_lines += file.content.lines().count();
        // Для утилиты сбора контекста потеря точности на 32-бит допустима.
        // Файлы >4GB всё равно фильтруются через max_file_size.
        #[allow(clippy::cast_possible_truncation)]
        {
            stats.total_bytes += file.size as usize;
        }

        *stats.languages.entry(file.language.clone()).or_insert(0) += 1;

        // Извлекаем имена крейтов из Cargo.toml
        if file.relative_path.ends_with("Cargo.toml") {
            if let Some(name) = extract_crate_name(&file.content) {
                stats.crates.push(name);
            }
        }
    }

    stats
}

/// Извлекает имя crate из содержимого Cargo.toml
fn extract_crate_name(cargo_toml: &str) -> Option<String> {
    for line in cargo_toml.lines() {
        let line = line.trim();
        if line.starts_with("name") {
            if let Some(name) = line.split('=').nth(1) {
                let name = name.trim().trim_matches('"');
                return Some(name.to_string());
            }
        }
    }
    None
}

/// Собирает все TODO/FIXME/XXX комментарии
fn collect_todos(files: &[FileEntry]) -> Vec<(String, usize, String)> {
    let mut todos = Vec::new();

    for file in files {
        for (line_num, line) in file.content.lines().enumerate() {
            let line_upper = line.to_uppercase();
            if line_upper.contains("TODO")
                || line_upper.contains("FIXME")
                || line_upper.contains("XXX")
                || line_upper.contains("HACK")
            {
                todos.push((
                    file.relative_path.clone(),
                    line_num + 1,
                    line.trim().to_string(),
                ));
            }
        }
    }

    todos
}

/// Загружает graphify-rs артефакты из указанной директории
fn load_graphify_artifacts(root: &Path, graphify_dir: &str) -> Result<GraphifyArtifacts> {
    let graphify_path = root.join(graphify_dir);
    let graph_json_path = graphify_path.join("graph.json");
    let graph_report_path = graphify_path.join("GRAPH_REPORT.md");

    // Проверяем, существует ли директория
    if !graphify_path.exists() {
        println!("      ⚠ Directory '{graphify_dir}' not found");
        return Ok(GraphifyArtifacts {
            graph_json: None,
            graph_report: None,
        });
    }

    let graph_json = if graph_json_path.exists() {
        let content = std::fs::read_to_string(graph_json_path)?;
        println!("      ✓ Found {graphify_dir}/graph.json");
        Some(content)
    } else {
        println!("      ⚠ {graphify_dir}/graph.json not found");
        None
    };

    let graph_report = if graph_report_path.exists() {
        let content = std::fs::read_to_string(graph_report_path)?;
        println!("      ✓ Found {graphify_dir}/GRAPH_REPORT.md");
        Some(content)
    } else {
        println!("      ⚠ {graphify_dir}/GRAPH_REPORT.md not found");
        None
    };

    Ok(GraphifyArtifacts {
        graph_json,
        graph_report,
    })
}

/// Запускает `graphify-rs build` для генерации артефактов.
///
/// Возвращает `true` если сборка прошла успешно,
/// `false` если graphify-rs не установлен или сборка пропущена.
fn run_graphify_build(root: &Path, graphify_dir: &str) -> bool {
    println!("   🔨 Running: graphify-rs build --no-llm --output ./{graphify_dir}");

    // Проверяем, установлен ли graphify-rs
    let check = std::process::Command::new("graphify-rs")
        .arg("--version")
        .output();

    match check {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            println!("      ✓ Found graphify-rs ({})", version.trim());
        }
        Ok(_) => {
            println!("      ⚠ graphify-rs returned error on --version, skipping build");
            return false;
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            println!("      ⚠ graphify-rs not found in PATH");
            println!("        Install with: cargo install graphify-rs");
            println!("        Skipping build, will use existing artifacts if present");
            return false;
        }
        Err(e) => {
            println!("      ⚠ Failed to check graphify-rs: {e}");
            return false;
        }
    }

    // Запускаем сборку
    let start = std::time::Instant::now();

    let status = std::process::Command::new("graphify-rs")
        .arg("build")
        .arg("--no-llm")
        .arg("--output")
        .arg(format!("./{graphify_dir}"))
        .current_dir(root)
        .status();

    match status {
        Ok(s) if s.success() => {
            let elapsed = start.elapsed();
            println!(
                "      ✓ Graph built successfully in {:.2}s",
                elapsed.as_secs_f64()
            );
            true
        }
        Ok(s) => {
            println!(
                "      ⚠ graphify-rs build failed with exit code: {:?}",
                s.code()
            );
            false
        }
        Err(e) => {
            println!("      ⚠ Failed to run graphify-rs build: {e}");
            false
        }
    }
}

/// Строит граф зависимостей через cargo metadata
fn build_graph(root: &Path) -> Result<String> {
    let output = std::process::Command::new("cargo")
        .arg("metadata")
        .arg("--format-version=1")
        .arg("--no-deps")
        .current_dir(root)
        .output()?;

    if !output.status.success() {
        return Ok("⚠️ Failed to run cargo metadata\n".to_string());
    }

    let stdout = String::from_utf8(output.stdout)?;
    let metadata: serde_json::Value = serde_json::from_str(&stdout)?;

    let mut graph = String::new();
    graph.push_str("```mermaid\n");
    graph.push_str("graph TD\n");

    // Рендерим узлы (workspace crates)
    if let Some(packages) = metadata["packages"].as_array() {
        for package in packages {
            let name = package["name"].as_str().unwrap_or("unknown");
            let version = package["version"].as_str().unwrap_or("0.0.0");
            let _ = writeln!(
                graph,
                "    {}[\"{} v{}\"]",
                name.replace('-', "_"),
                name,
                version
            );
        }

        // Рендерим рёбра (зависимости между workspace crates)
        for package in packages {
            let name = package["name"].as_str().unwrap_or("unknown");
            if let Some(deps) = package["dependencies"].as_array() {
                for dep in deps {
                    let dep_name = dep["name"].as_str().unwrap_or("unknown");
                    // Показываем только зависимости между workspace crates
                    let is_workspace_dep = packages
                        .iter()
                        .any(|p| p["name"].as_str() == Some(dep_name));
                    if is_workspace_dep {
                        let _ = writeln!(
                            graph,
                            "    {} --> {}",
                            name.replace('-', "_"),
                            dep_name.replace('-', "_")
                        );
                    }
                }
            }
        }
    }

    graph.push_str("```\n");

    Ok(graph)
}

/// Получает последние git-коммиты
fn build_git_log(root: &Path) -> Result<String> {
    let output = std::process::Command::new("git")
        .arg("log")
        .arg("--oneline")
        .arg("-20")
        .current_dir(root)
        .output()?;

    if !output.status.success() {
        return Ok("⚠️ Git not available or not a git repository\n".to_string());
    }

    let stdout = String::from_utf8(output.stdout)?;

    let mut log = String::new();
    log.push_str("```\n");
    log.push_str(&stdout);
    log.push_str("```\n");

    Ok(log)
}

/// Собирает информацию об окружении
fn build_env_info() -> Result<String> {
    let rustc_output = std::process::Command::new("rustc")
        .arg("--version")
        .output()?;

    let rust_version = if rustc_output.status.success() {
        String::from_utf8(rustc_output.stdout)?.trim().to_string()
    } else {
        "unknown".to_string()
    };

    let mut info = String::new();
    let _ = writeln!(info, "- **Rust:** {rust_version}");
    let _ = writeln!(
        info,
        "- **OS:** {} {}",
        std::env::consts::OS,
        std::env::consts::ARCH
    );
    let _ = writeln!(info, "- **Family:** {}", std::env::consts::FAMILY);

    Ok(info)
}

/// Читает корневой Cargo.toml
fn read_workspace_cargo(root: &Path) -> Result<Option<String>> {
    let cargo_path = root.join("Cargo.toml");
    if cargo_path.exists() {
        let content = std::fs::read_to_string(cargo_path)?;
        Ok(Some(content))
    } else {
        Ok(None)
    }
}

/// Форматирует собранный контекст проекта в Markdown.
fn format_markdown(ctx: &MarkdownContext) -> String {
    let mut md = String::new();

    // Header
    md.push_str("# 📦 Project Context\n\n");
    let _ = write!(
        md,
        "*Generated on {}*\n\n",
        Local::now().format("%Y-%m-%d %H:%M:%S")
    );

    // Statistics
    md.push_str("## 📊 Statistics\n\n");
    let _ = writeln!(md, "- **Total files:** {}", ctx.stats.total_files);
    let _ = writeln!(md, "- **Total lines:** {}", ctx.stats.total_lines);
    let _ = writeln!(md, "- **Total size:** {} bytes", ctx.stats.total_bytes);
    let _ = writeln!(md, "- **Crates:** {}", ctx.stats.crates.len());
    md.push('\n');

    // Environment
    md.push_str("## 🖥️ Environment\n\n");
    md.push_str(ctx.env_info);
    md.push('\n');

    // Workspace crates
    if !ctx.stats.crates.is_empty() {
        md.push_str("## 📦 Workspace Crates\n\n");
        for crate_name in &ctx.stats.crates {
            let _ = writeln!(md, "- `{crate_name}`");
        }
        md.push('\n');
    }

    // Languages
    if !ctx.stats.languages.is_empty() {
        md.push_str("## 🗣️ Languages\n\n");
        let mut langs: Vec<_> = ctx.stats.languages.iter().collect();
        langs.sort_by(|a, b| b.1.cmp(a.1));
        for (lang, count) in langs {
            let _ = writeln!(md, "- **{lang}:** {count} files");
        }
        md.push('\n');
    }

    // Workspace Cargo.toml
    if let Some(cargo) = ctx.workspace_cargo {
        md.push_str("## 📋 Workspace Cargo.toml\n\n");
        md.push_str("```toml\n");
        md.push_str(cargo);
        if !cargo.ends_with('\n') {
            md.push('\n');
        }
        md.push_str("```\n\n");
    }

    // Project structure
    md.push_str("## 🌳 Project Structure\n\n");
    md.push_str(ctx.tree);
    md.push('\n');

    // Dependency graph (cargo metadata)
    if let Some(graph) = ctx.graph {
        md.push_str("## 🔗 Dependency Graph (Cargo Metadata)\n\n");
        md.push_str(graph);
        md.push('\n');
    }

    // Graphify-rs artifacts
    if let Some(artifacts) = ctx.graphify {
        md.push_str("## 🧠 Knowledge Graph (graphify-rs)\n\n");

        if let Some(report) = &artifacts.graph_report {
            md.push_str("### 📊 Graph Analysis Report\n\n");
            md.push_str(report);
            if !report.ends_with('\n') {
                md.push('\n');
            }
            md.push('\n');
        }

        if let Some(json) = &artifacts.graph_json {
            md.push_str("### 🔗 Graph Data (JSON)\n\n");
            md.push_str("```json\n");
            md.push_str(json);
            if !json.ends_with('\n') {
                md.push('\n');
            }
            md.push_str("```\n\n");
        }
    }

    // Git log
    if let Some(git_log) = ctx.git_log {
        md.push_str("## 📜 Recent Commits\n\n");
        md.push_str(git_log);
        md.push('\n');
    }

    // TODOs
    if !ctx.todos.is_empty() {
        md.push_str("## 📝 TODOs and FIXMEs\n\n");
        for (file, line, content) in ctx.todos {
            let _ = writeln!(md, "- **{file}:{line}** — `{content}`");
        }
        md.push('\n');
    }

    // Source files
    md.push_str("## 📄 Source Files\n\n");

    for file in ctx.files {
        let _ = write!(md, "### `{}`\n\n", file.relative_path);
        let _ = writeln!(md, "```{}", file.language);
        md.push_str(&file.content);
        if !file.content.ends_with('\n') {
            md.push('\n');
        }
        md.push_str("```\n\n");
    }

    md
}
