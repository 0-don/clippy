use serde::{Deserialize, Serialize};
use std::{
    io::{BufRead, BufReader, IsTerminal, Read, Write},
    net::{Ipv4Addr, TcpStream},
    path::PathBuf,
    time::Duration,
};

const CLI_FILE_NAME: &str = "cli.json";
const COMMANDS: [&str; 12] = [
    "list", "search", "get", "copy", "add", "edit", "rename", "star", "unstar", "delete", "run",
    "help",
];

const HELP: &str = "Usage: clippy <command> [arguments]

Talks to the running Clippy. Index 1 is the newest clipboard.

  list [-n COUNT] [--json]          show recent clipboards (default 10)
  search QUERY [-n COUNT] [--json]  search clipboards
  get [INDEX|ID]                    print a clipboard (default 1)
  copy [INDEX|ID]                   put a clipboard back on the system clipboard
  add [TEXT]                        copy text, or stdin when TEXT is missing
  edit INDEX|ID [TEXT]              replace a text clipboard, opens $EDITOR without TEXT
  rename INDEX|ID [NAME]            name a clipboard, no NAME removes the name
  star INDEX|ID, unstar INDEX|ID    add to or remove from favorites
  delete INDEX|ID                   delete a clipboard
  run NAME                          run a custom command as if its hotkey was pressed
  help                              show this help
";

#[derive(Serialize, Deserialize)]
struct Endpoint {
    port: u16,
    token: String,
}

#[derive(Serialize, Deserialize)]
struct Request {
    token: String,
    command: String,
    args: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Response {
    ok: bool,
    output: String,
}

/// Same folder Tauri uses as app data dir for the "clippy" identifier, resolved
/// without a running app so the client can find the server.
fn cli_file() -> Option<PathBuf> {
    let dir = if cfg!(debug_assertions) {
        std::env::current_dir().ok()?.parent()?.to_path_buf()
    } else {
        dirs::data_dir()?.join("clippy")
    };
    Some(dir.join(CLI_FILE_NAME))
}

/// Handles `clippy <command>` and returns the exit code, or None to start the app.
pub fn run_client(args: &[String]) -> Option<i32> {
    let command = args.first()?.as_str();
    let command = match command {
        "-h" | "--help" => "help",
        other if COMMANDS.contains(&other) => other,
        _ => return None,
    };

    #[cfg(windows)]
    unsafe {
        // Release builds use the GUI subsystem and have no console of their own.
        windows_sys::Win32::System::Console::AttachConsole(
            windows_sys::Win32::System::Console::ATTACH_PARENT_PROCESS,
        );
    }

    if command == "help" {
        print!("{HELP}");
        return Some(0);
    }

    let mut args: Vec<String> = args[1..].to_vec();
    let missing_text = match command {
        "add" => args.is_empty(),
        "edit" => args.len() == 1,
        _ => false,
    };
    if missing_text {
        if !std::io::stdin().is_terminal() {
            let mut text = String::new();
            if std::io::stdin().read_to_string(&mut text).is_ok() {
                args.push(text);
            }
        } else if command == "edit" {
            match edit_in_editor(&args[0]) {
                Ok(Some(text)) => args.push(text),
                Ok(None) => {
                    eprintln!("unchanged");
                    return Some(0);
                }
                Err(e) => {
                    eprintln!("{e}");
                    return Some(1);
                }
            }
        }
    }

    match send(command, args) {
        Ok(response) => {
            if response.ok {
                if !response.output.is_empty() {
                    println!("{}", response.output);
                }
                Some(0)
            } else {
                eprintln!("{}", response.output);
                Some(1)
            }
        }
        Err(e) => {
            eprintln!("clippy is not running ({e})");
            Some(2)
        }
    }
}

/// Opens the current text in the user's editor; None when nothing changed.
fn edit_in_editor(selector: &str) -> Result<Option<String>, String> {
    let current = match send("get", vec![selector.to_string()]) {
        Ok(response) if response.ok => response.output,
        Ok(response) => return Err(response.output),
        Err(e) => return Err(format!("clippy is not running ({e})")),
    };
    let path = std::env::temp_dir().join(format!("clippy-edit-{}.txt", std::process::id()));
    std::fs::write(&path, &current).map_err(|e| e.to_string())?;

    let editor = std::env::var("VISUAL")
        .or_else(|_| std::env::var("EDITOR"))
        .unwrap_or_else(|_| if cfg!(windows) { "notepad" } else { "vi" }.to_string());
    let mut parts = editor.split_whitespace();
    let program = parts.next().unwrap_or("vi");
    let status = std::process::Command::new(program)
        .args(parts)
        .arg(&path)
        .status()
        .map_err(|e| format!("could not start {program}: {e}"));
    let edited = std::fs::read_to_string(&path);
    let _ = std::fs::remove_file(&path);

    if !status?.success() {
        return Err(format!("{program} exited with an error"));
    }
    let edited = edited.map_err(|e| e.to_string())?;
    Ok((edited != current).then_some(edited))
}

fn send(command: &str, args: Vec<String>) -> Result<Response, String> {
    let path = cli_file().ok_or("no data directory")?;
    let endpoint: Endpoint = std::fs::read_to_string(&path)
        .ok()
        .and_then(|json| serde_json::from_str(&json).ok())
        .ok_or("no cli.json")?;

    let mut stream =
        TcpStream::connect((Ipv4Addr::LOCALHOST, endpoint.port)).map_err(|e| e.to_string())?;
    stream
        .set_read_timeout(Some(Duration::from_secs(60)))
        .map_err(|e| e.to_string())?;

    let request = Request {
        token: endpoint.token,
        command: command.to_string(),
        args,
    };
    let mut line = serde_json::to_string(&request).map_err(|e| e.to_string())?;
    line.push('\n');
    stream
        .write_all(line.as_bytes())
        .map_err(|e| e.to_string())?;

    let mut reply = String::new();
    BufReader::new(stream)
        .read_line(&mut reply)
        .map_err(|e| e.to_string())?;
    serde_json::from_str(&reply).map_err(|e| e.to_string())
}

pub mod server {
    use super::{cli_file, Endpoint, Request, Response, HELP};
    use crate::{
        commands::clipboard::{
            delete_clipboard, get_clipboard, get_clipboards, rename_clipboard, star_clipboard,
            update_clipboard_content,
        },
        prelude::*,
        service::{
            cipher::is_encryption_key_set,
            clipboard::{copy_clipboard_from_id, init_clipboards},
            custom_command::run_command_hotkey,
            settings::get_global_settings,
        },
        tao::{connection::db, global::get_app},
    };
    use common::types::{
        enums::ClipboardType,
        orm_query::FullClipboardDto,
        types::{CommandError, CustomCommand},
    };
    use entity::clipboard;
    use ring::rand::{SecureRandom, SystemRandom};
    use sea_orm::{prelude::Uuid, EntityTrait, QueryOrder, QuerySelect};
    use serde_json::json;
    use tauri::Manager;
    use tauri_plugin_clipboard::Clipboard;
    use tokio::{
        io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
        net::TcpListener,
    };

    const MAX_REQUEST_BYTES: u64 = 16 * 1024 * 1024;
    const PREVIEW_CHARS: usize = 80;

    pub fn setup_cli_server() {
        tauri::async_runtime::spawn(async {
            if let Err(e) = serve().await {
                printlog!("cli: server stopped: {}", e);
            }
        });
    }

    async fn serve() -> Result<(), String> {
        let listener = TcpListener::bind(("127.0.0.1", 0))
            .await
            .map_err(|e| e.to_string())?;
        let port = listener.local_addr().map_err(|e| e.to_string())?.port();

        let mut bytes = [0u8; 32];
        SystemRandom::new()
            .fill(&mut bytes)
            .map_err(|_| "no randomness".to_string())?;
        let token: String = bytes.iter().map(|b| format!("{b:02x}")).collect();

        let path = cli_file().ok_or("no data directory")?;
        let json = serde_json::to_string(&Endpoint {
            port,
            token: token.clone(),
        })
        .map_err(|e| e.to_string())?;
        write_private(&path, json.as_bytes()).map_err(|e| e.to_string())?;
        printlog!("cli: listening on 127.0.0.1:{}", port);

        loop {
            let Ok((stream, _)) = listener.accept().await else {
                continue;
            };
            let token = token.clone();
            tauri::async_runtime::spawn(async move {
                let (read, mut write) = stream.into_split();
                let mut line = String::new();
                let mut reader =
                    BufReader::new(tokio::io::AsyncReadExt::take(read, MAX_REQUEST_BYTES));
                if reader.read_line(&mut line).await.is_err() {
                    return;
                }
                let response = match serde_json::from_str::<Request>(&line) {
                    Ok(request) if constant_eq(&request.token, &token) => {
                        handle(&request.command, &request.args).await
                    }
                    _ => fail("unauthorized"),
                };
                if let Ok(mut reply) = serde_json::to_string(&response) {
                    reply.push('\n');
                    let _ = write.write_all(reply.as_bytes()).await;
                }
            });
        }
    }

    fn write_private(path: &std::path::Path, bytes: &[u8]) -> std::io::Result<()> {
        use std::io::Write;
        let mut options = std::fs::OpenOptions::new();
        options.write(true).create(true).truncate(true);
        #[cfg(unix)]
        std::os::unix::fs::OpenOptionsExt::mode(&mut options, 0o600);
        options.open(path)?.write_all(bytes)
    }

    fn constant_eq(a: &str, b: &str) -> bool {
        a.len() == b.len()
            && a.bytes()
                .zip(b.bytes())
                .fold(0u8, |acc, (x, y)| acc | (x ^ y))
                == 0
    }

    fn ok(output: impl Into<String>) -> Response {
        Response {
            ok: true,
            output: output.into(),
        }
    }

    fn fail(output: impl Into<String>) -> Response {
        Response {
            ok: false,
            output: output.into(),
        }
    }

    fn is_locked() -> bool {
        get_global_settings().encryption && !is_encryption_key_set()
    }

    async fn handle(command: &str, args: &[String]) -> Response {
        let reads_history = !matches!(command, "add" | "run" | "help");
        if reads_history && is_locked() {
            return fail("clippy is locked, enter the password in the app first");
        }
        let options = Options::parse(args);
        match command {
            "list" => list(None, &options).await,
            "search" if options.positional.is_empty() => {
                fail("usage: clippy search QUERY [-n COUNT] [--json]")
            }
            "search" => list(Some(options.positional.join(" ")), &options).await,
            "get" => get(options.positional.first()).await,
            "copy" => copy(options.positional.first()).await,
            "add" => add(args),
            "edit" => edit(&options.positional).await,
            "rename" => rename(&options.positional).await,
            "star" => star(options.positional.first(), true).await,
            "unstar" => star(options.positional.first(), false).await,
            "delete" => delete(options.positional.first()).await,
            "run" => run(args).await,
            _ => ok(HELP.trim_end()),
        }
    }

    struct Options {
        positional: Vec<String>,
        count: usize,
        json: bool,
    }

    impl Options {
        fn parse(args: &[String]) -> Self {
            let mut options = Options {
                positional: Vec::new(),
                count: 10,
                json: false,
            };
            let mut args = args.iter();
            while let Some(arg) = args.next() {
                match arg.as_str() {
                    "--json" => options.json = true,
                    "-n" => {
                        if let Some(n) = args.next().and_then(|n| n.parse::<usize>().ok()) {
                            options.count = n.clamp(1, 1000);
                        }
                    }
                    _ => options.positional.push(arg.clone()),
                }
            }
            options
        }
    }

    async fn list(search: Option<String>, options: &Options) -> Response {
        let limit = options.count;

        let mut clipboards: Vec<FullClipboardDto> = Vec::new();
        while clipboards.len() < limit {
            let page =
                match get_clipboards(Some(clipboards.len() as u64), search.clone(), None, None)
                    .await
                {
                    Ok(page) => page,
                    Err(CommandError::Error(e)) => return fail(e),
                };
            let done = !page.has_more || page.clipboards.is_empty();
            clipboards.extend(page.clipboards);
            if done {
                break;
            }
        }
        clipboards.truncate(limit);

        if options.json {
            let rows: Vec<_> = clipboards
                .iter()
                .enumerate()
                .map(|(i, c)| {
                    json!({
                        "index": i + 1,
                        "id": c.clipboard.id,
                        "types": c.clipboard.types,
                        "star": c.clipboard.star,
                        "created_at": c.clipboard.created_at,
                        "preview": preview(c),
                    })
                })
                .collect();
            return ok(serde_json::to_string_pretty(&rows).unwrap_or_default());
        }

        let lines: Vec<String> = clipboards
            .iter()
            .enumerate()
            .map(|(i, c)| format!("{:>4}  {:<5}  {}", i + 1, kind(c), preview(c)))
            .collect();
        ok(lines.join("\n"))
    }

    fn kind(clipboard: &FullClipboardDto) -> &'static str {
        if clipboard.image.is_some() {
            "image"
        } else if !clipboard.files.is_empty() {
            "file"
        } else if clipboard.text.is_some() {
            "text"
        } else if clipboard.html.is_some() {
            "html"
        } else {
            "rtf"
        }
    }

    fn preview(clipboard: &FullClipboardDto) -> String {
        if let Some(image) = &clipboard.image {
            return format!("[image {}x{}]", image.width, image.height);
        }
        if !clipboard.files.is_empty() {
            let names: Vec<&str> = clipboard.files.iter().map(|f| f.name.as_str()).collect();
            return format!("[file {}]", names.join(", "));
        }
        let text = full_text(clipboard).unwrap_or_default();
        let line: String = text.split_whitespace().collect::<Vec<_>>().join(" ");
        if line.chars().count() > PREVIEW_CHARS {
            format!(
                "{}...",
                line.chars().take(PREVIEW_CHARS).collect::<String>()
            )
        } else {
            line
        }
    }

    fn full_text(clipboard: &FullClipboardDto) -> Option<String> {
        clipboard
            .text
            .as_ref()
            .map(|t| t.data.clone())
            .or_else(|| clipboard.html.as_ref().map(|h| h.data.clone()))
            .or_else(|| clipboard.rtf.as_ref().map(|r| r.data.clone()))
    }

    /// Accepts a clipboard id or a 1 based index into the history, newest first.
    async fn resolve(selector: Option<&String>) -> Result<clipboard::Model, String> {
        let selector = selector.map(String::as_str).unwrap_or("1");
        let found = match Uuid::parse_str(selector) {
            Ok(id) => clipboard::Entity::find_by_id(id).one(db()).await,
            Err(_) => {
                let index: u64 = selector
                    .parse()
                    .ok()
                    .filter(|i| *i >= 1)
                    .ok_or(format!("not an index or id: {selector}"))?;
                clipboard::Entity::find()
                    .order_by_desc(clipboard::Column::Id)
                    .offset(index - 1)
                    .limit(1)
                    .one(db())
                    .await
            }
        }
        .map_err(|e| e.to_string())?;
        found.ok_or(format!("no clipboard {selector}"))
    }

    async fn get(selector: Option<&String>) -> Response {
        let model = match resolve(selector).await {
            Ok(model) => model,
            Err(e) => return fail(e),
        };
        let clipboard = match get_clipboard(model.id).await {
            Ok(clipboard) => clipboard,
            Err(CommandError::Error(e)) => return fail(e),
        };
        match full_text(&clipboard) {
            Some(text) => ok(text),
            None => ok(preview(&clipboard)),
        }
    }

    async fn copy(selector: Option<&String>) -> Response {
        let model = match resolve(selector).await {
            Ok(model) => model,
            Err(e) => return fail(e),
        };
        let kind = serde_json::from_value::<Vec<ClipboardType>>(model.types.clone())
            .ok()
            .and_then(|types| types.into_iter().next())
            .unwrap_or(ClipboardType::Text);
        match copy_clipboard_from_id(model.id, kind).await {
            Ok(true) => ok(""),
            Ok(false) => fail("could not copy this clipboard"),
            Err(e) => fail(e.to_string()),
        }
    }

    /// Write commands need an explicit target, defaulting to the newest is too easy to regret.
    async fn target(selector: Option<&String>, usage: &str) -> Result<clipboard::Model, Response> {
        match selector {
            Some(selector) => resolve(Some(selector)).await.map_err(fail),
            None => Err(fail(format!("usage: clippy {usage}"))),
        }
    }

    async fn edit(positional: &[String]) -> Response {
        let model = match target(positional.first(), "edit INDEX|ID [TEXT]").await {
            Ok(model) => model,
            Err(response) => return response,
        };
        let text = positional[1..].join(" ");
        let result = update_clipboard_content(model.id, model.name.clone(), text).await;
        match result {
            Ok(_) => {
                init_clipboards();
                ok("")
            }
            Err(CommandError::Error(e)) => fail(e),
        }
    }

    async fn rename(positional: &[String]) -> Response {
        let model = match target(positional.first(), "rename INDEX|ID [NAME]").await {
            Ok(model) => model,
            Err(response) => return response,
        };
        let name = positional[1..].join(" ");
        let name = (!name.trim().is_empty()).then_some(name);
        match rename_clipboard(model.id, name).await {
            Ok(_) => {
                init_clipboards();
                ok("")
            }
            Err(CommandError::Error(e)) => fail(e),
        }
    }

    async fn star(selector: Option<&String>, starred: bool) -> Response {
        let usage = if starred {
            "star INDEX|ID"
        } else {
            "unstar INDEX|ID"
        };
        let model = match target(selector, usage).await {
            Ok(model) => model,
            Err(response) => return response,
        };
        match star_clipboard(model.id, starred).await {
            Ok(_) => {
                init_clipboards();
                ok("")
            }
            Err(CommandError::Error(e)) => fail(e),
        }
    }

    async fn delete(selector: Option<&String>) -> Response {
        let model = match target(selector, "delete INDEX|ID").await {
            Ok(model) => model,
            Err(response) => return response,
        };
        match delete_clipboard(model.id).await {
            Ok(()) => {
                init_clipboards();
                ok("")
            }
            Err(CommandError::Error(e)) => fail(e),
        }
    }

    fn add(args: &[String]) -> Response {
        if args.is_empty() {
            return fail("usage: clippy add TEXT, or pipe text into clippy add");
        }
        let text = args.join(" ");
        match get_app().state::<Clipboard>().write_text(text) {
            Ok(_) => ok(""),
            Err(e) => fail(e),
        }
    }

    async fn run(args: &[String]) -> Response {
        let name = args.join(" ");
        if name.is_empty() {
            return fail("usage: clippy run NAME");
        }
        let command = CustomCommand::from_json_value(&get_global_settings().commands)
            .into_iter()
            .find(|command| command.enabled && command.name.eq_ignore_ascii_case(&name));
        match command {
            Some(command) => {
                run_command_hotkey(command).await;
                ok("")
            }
            None => fail(format!("no enabled command named {name}")),
        }
    }
}
