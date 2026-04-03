use anyhow::Result;
use std::process::Command;
use std::fs;
use std::path::Path;
use rand::seq::SliceRandom;

pub struct BypassConfig {
    pub proxy: Option<String>,
    pub dpi_bypass: bool,
}

impl BypassConfig {
    pub fn new() -> Self {
        Self {
            proxy: None,
            dpi_bypass: false,
        }
    }

    pub fn with_proxy(mut self, proxy: String) -> Self {
        self.proxy = Some(proxy);
        self
    }

    pub fn with_dpi_bypass(mut self) -> Self {
        self.dpi_bypass = true;
        self
    }
}

pub fn load_proxies() -> Vec<String> {
    let paths = vec![
        "proxies.txt",
        "/usr/local/share/media-cli/proxies.txt",
        "~/.config/media-cli/proxies.txt",
    ];
    
    for path_str in paths {
        let path = if path_str.starts_with('~') {
            if let Some(home) = std::env::var_os("HOME") {
                Path::new(&home).join(&path_str[2..])
            } else {
                continue;
            }
        } else {
            Path::new(path_str).to_path_buf()
        };
        
        if let Ok(content) = fs::read_to_string(&path) {
            let proxies: Vec<String> = content
                .lines()
                .filter(|line| !line.trim().is_empty() && !line.trim().starts_with('#'))
                .map(|line| {
                    let line = line.trim();
                    if line.starts_with("socks5://") {
                        line.to_string()
                    } else {
                        format!("socks5://{}", line)
                    }
                })
                .collect();
            
            if !proxies.is_empty() {
                return proxies;
            }
        }
    }
    
    vec![]
}

pub fn test_proxy(proxy: &str) -> bool {
    println!("\x1B[1;90m  Testing {}...\x1B[0m", proxy);
    
    let output = Command::new("curl")
        .args([
            "-x", proxy,
            "-s", "-o", "/dev/null",
            "-w", "%{http_code}",
            "--max-time", "5",
            "https://www.youtube.com"
        ])
        .output();
    
    match output {
        Ok(out) => {
            let code = String::from_utf8_lossy(&out.stdout);
            code.starts_with('2') || code.starts_with('3')
        }
        Err(_) => false,
    }
}

pub fn find_working_proxy() -> Option<String> {
    let mut proxies = load_proxies();
    
    if proxies.is_empty() {
        println!("\x1B[1;33m⚠\x1B[0m  proxies.txt not found or empty");
        return None;
    }
    
    println!("\x1B[1;36m🔍 Loaded {} proxies\x1B[0m", proxies.len());
    
    let mut rng = rand::thread_rng();
    proxies.shuffle(&mut rng);
    
    for proxy in proxies.iter().take(5) {
        if test_proxy(proxy) {
            println!("\x1B[1;32m✓\x1B[0m Working proxy: {}", proxy);
            return Some(proxy.clone());
        }
    }
    
    None
}

pub fn check_access(url: &str) -> bool {
    let output = Command::new("curl")
        .args(["-s", "-o", "/dev/null", "-w", "%{http_code}", "--max-time", "5", url])
        .output();
    
    match output {
        Ok(out) => {
            let code = String::from_utf8_lossy(&out.stdout);
            code.starts_with('2') || code.starts_with('3')
        }
        Err(_) => false,
    }
}

pub fn detect_os() -> String {
    std::env::consts::OS.to_string()
}

pub fn start_dpi_bypass() -> Result<()> {
    let os = detect_os();
    
    match os.as_str() {
        "linux" => {
            println!("\x1B[1;33m⚡ Starting DPI bypass (zapret)...\x1B[0m");
            
            if Command::new("which").arg("zapret").output().is_ok() {
                std::thread::spawn(|| {
                    let _ = Command::new("zapret").spawn();
                });
                println!("\x1B[1;32m✓\x1B[0m zapret started");
            } else {
                println!("\x1B[1;33m⚠\x1B[0m  zapret not installed");
                println!("\x1B[1;90m   Install: https://github.com/bol-van/zapret\x1B[0m");
            }
        }
        "windows" => {
            println!("\x1B[1;33m⚡ Starting DPI bypass (GoodbyeDPI)...\x1B[0m");
            println!("\x1B[1;33m⚠\x1B[0m  Download GoodbyeDPI: https://github.com/ValdikSS/GoodbyeDPI");
        }
        _ => {
            println!("\x1B[1;33m⚠\x1B[0m  DPI bypass not supported on {}", os);
        }
    }
    
    Ok(())
}

pub fn auto_bypass() -> Result<Option<String>> {
    println!("\x1B[1;36m🔍 Checking YouTube access...\x1B[0m");
    
    if check_access("https://www.youtube.com") {
        println!("\x1B[1;32m✓\x1B[0m Access available, bypass not needed");
        return Ok(None);
    }
    
    println!("\x1B[1;31m✕\x1B[0m YouTube is blocked");
    println!("\x1B[1;33m⚡ Searching for working proxy...\x1B[0m");
    
    if let Some(proxy) = find_working_proxy() {
        return Ok(Some(proxy));
    }
    
    println!("\x1B[1;31m✕\x1B[0m No working proxies found");
    println!("\x1B[1;36mℹ\x1B[0m  Add your SOCKS5 proxies to proxies.txt");
    
    Ok(None)
}

pub fn get_common_proxies() -> Vec<(&'static str, &'static str)> {
    vec![
        ("SOCKS5 (Shadowsocks)", "socks5://127.0.0.1:1080"),
        ("HTTP (Clash)", "http://127.0.0.1:7890"),
        ("SOCKS5 (v2ray)", "socks5://127.0.0.1:10808"),
        ("HTTP (Xray)", "http://127.0.0.1:10809"),
    ]
}

pub fn show_bypass_help() {
    println!("\n\x1B[1;36m╭─ Bypass Restrictions ──────────────────────────────╮\x1B[0m");
    println!("\x1B[1;36m│\x1B[0m                                                    \x1B[1;36m│\x1B[0m");
    println!("\x1B[1;36m│\x1B[0m  \x1B[1;33m1. Auto mode:\x1B[0m                                  \x1B[1;36m│\x1B[0m");
    println!("\x1B[1;36m│\x1B[0m     media-cli --proxy auto yt \"query\"            \x1B[1;36m│\x1B[0m");
    println!("\x1B[1;36m│\x1B[0m                                                    \x1B[1;36m│\x1B[0m");
    println!("\x1B[1;36m│\x1B[0m  \x1B[1;33m2. DPI bypass (zapret/GoodbyeDPI):\x1B[0m           \x1B[1;36m│\x1B[0m");
    println!("\x1B[1;36m│\x1B[0m     Requires separate installation                \x1B[1;36m│\x1B[0m");
    println!("\x1B[1;36m│\x1B[0m                                                    \x1B[1;36m│\x1B[0m");
    println!("\x1B[1;36m│\x1B[0m  \x1B[1;33m3. Proxy/VPN:\x1B[0m                                 \x1B[1;36m│\x1B[0m");
    println!("\x1B[1;36m│\x1B[0m     media-cli --proxy socks5://127.0.0.1:1080    \x1B[1;36m│\x1B[0m");
    println!("\x1B[1;36m│\x1B[0m                                                    \x1B[1;36m│\x1B[0m");
    println!("\x1B[1;36m│\x1B[0m  \x1B[1;90mCommon proxies:\x1B[0m                            \x1B[1;36m│\x1B[0m");
    for (name, addr) in get_common_proxies() {
        println!("\x1B[1;36m│\x1B[0m    \x1B[1;90m{:<20}\x1B[0m {}  \x1B[1;36m│\x1B[0m", name, addr);
    }
    println!("\x1B[1;36m│\x1B[0m                                                    \x1B[1;36m│\x1B[0m");
    println!("\x1B[1;36m╰────────────────────────────────────────────────────╯\x1B[0m\n");
}
