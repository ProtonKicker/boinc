use std::{
    fs,
    io::{self, Read, Write},
    net::{Ipv4Addr, SocketAddrV4, TcpStream},
    path::{Path, PathBuf},
    sync::OnceLock,
    time::Duration,
};

use quick_xml::{events::Event, Reader};

#[derive(Debug)]
pub enum BoincRpcError {
    Io(String),
    Protocol(String),
    Unauthorized,
    Unsupported(String),
}

impl std::fmt::Display for BoincRpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoincRpcError::Io(msg) => write!(f, "I/O error: {msg}"),
            BoincRpcError::Protocol(msg) => write!(f, "BOINC RPC protocol error: {msg}"),
            BoincRpcError::Unauthorized => write!(f, "BOINC RPC unauthorized"),
            BoincRpcError::Unsupported(msg) => write!(f, "Unsupported: {msg}"),
        }
    }
}

impl std::error::Error for BoincRpcError {}

impl From<io::Error> for BoincRpcError {
    fn from(value: io::Error) -> Self {
        Self::Io(value.to_string())
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BoincTask {
    pub name: Option<String>,
    pub wu_name: Option<String>,
    pub project_url: Option<String>,
    pub state: Option<i32>,
    pub ready_to_report: Option<bool>,
    pub got_server_ack: Option<bool>,
    pub received_time: Option<f64>,
    pub report_deadline: Option<f64>,
    pub active_task: Option<bool>,
    pub active_task_state: Option<i32>,
    pub fraction_done: Option<f64>,
    pub elapsed_time: Option<f64>,
    pub estimated_cpu_time_remaining: Option<f64>,
}

pub fn get_results(active_only: bool) -> Result<Vec<BoincTask>, BoincRpcError> {
    let password = read_gui_rpc_password()?;
    let mut stream = connect_local_boinc()?;
    gui_rpc_auth(&mut stream, &password)?;

    let request = format!(
        "<get_results>\n<active_only>{}</active_only>\n</get_results>\n",
        if active_only { 1 } else { 0 }
    );
    let reply = do_rpc(&mut stream, &request)?;
    parse_get_results_reply(&reply)
}

fn connect_local_boinc() -> Result<TcpStream, BoincRpcError> {
    let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 31416);
    let stream = TcpStream::connect_timeout(&addr.into(), Duration::from_secs(2))?;
    stream.set_nodelay(true)?;
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    stream.set_write_timeout(Some(Duration::from_secs(5)))?;
    Ok(stream)
}

fn do_rpc(stream: &mut TcpStream, inner_xml: &str) -> Result<String, BoincRpcError> {
    let request = format!(
        "<boinc_gui_rpc_request>\n{}\n</boinc_gui_rpc_request>\n\u{3}",
        inner_xml.trim_end()
    );
    stream.write_all(request.as_bytes())?;
    stream.flush()?;

    let mut buf = Vec::new();
    let mut chunk = [0u8; 8192];
    loop {
        let n = stream.read(&mut chunk)?;
        if n == 0 {
            return Err(BoincRpcError::Protocol(
                "connection closed before terminator".to_string(),
            ));
        }
        if let Some(pos) = chunk[..n].iter().position(|b| *b == 0x03) {
            buf.extend_from_slice(&chunk[..pos]);
            break;
        }
        buf.extend_from_slice(&chunk[..n]);
        if buf.len() > 20 * 1024 * 1024 {
            return Err(BoincRpcError::Protocol("reply too large".to_string()));
        }
    }

    Ok(String::from_utf8_lossy(&buf).to_string())
}

fn gui_rpc_auth(stream: &mut TcpStream, password: &str) -> Result<(), BoincRpcError> {
    let reply = do_rpc(stream, "<auth1/>\n")?;
    if xml_has_unauthorized(&reply) {
        return Err(BoincRpcError::Unauthorized);
    }

    let nonce = xml_first_text(&reply, b"nonce")
        .ok_or_else(|| BoincRpcError::Protocol("nonce not found".to_string()))?;
    let digest = md5::compute(format!("{nonce}{password}").as_bytes());
    let nonce_hash = format!("{:x}", digest);

    let auth2 = format!(
        "<auth2>\n<nonce_hash>{}</nonce_hash>\n</auth2>\n",
        nonce_hash
    );
    let reply = do_rpc(stream, &auth2)?;
    if xml_has_unauthorized(&reply) {
        return Err(BoincRpcError::Unauthorized);
    }

    let authorized = xml_first_text(&reply, b"authorized")
        .map(|s| s == "1" || s.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    if authorized {
        Ok(())
    } else {
        Err(BoincRpcError::Unauthorized)
    }
}

fn xml_has_unauthorized(xml: &str) -> bool {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Empty(e)) | Ok(Event::Start(e)) => {
                if e.name().as_ref() == b"unauthorized" {
                    return true;
                }
            }
            Ok(Event::Eof) => return false,
            Err(_) => return false,
            _ => {}
        }
        buf.clear();
    }
}

fn xml_first_text(xml: &str, tag: &[u8]) -> Option<String> {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);
    let mut buf = Vec::new();
    let mut in_tag = false;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                if e.name().as_ref() == tag {
                    in_tag = true;
                }
            }
            Ok(Event::End(e)) => {
                if e.name().as_ref() == tag {
                    in_tag = false;
                }
            }
            Ok(Event::Text(t)) => {
                if in_tag {
                    if let Ok(text) = t.unescape() {
                        return Some(text.to_string());
                    }
                }
            }
            Ok(Event::Eof) => return None,
            Err(_) => return None,
            _ => {}
        }
        buf.clear();
    }
}

fn parse_get_results_reply(xml: &str) -> Result<Vec<BoincTask>, BoincRpcError> {
    if xml_has_unauthorized(xml) {
        return Err(BoincRpcError::Unauthorized);
    }

    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);
    let mut buf = Vec::new();

    let mut tasks: Vec<BoincTask> = Vec::new();
    let mut current: Option<BoincTask> = None;
    let mut current_field: Option<Vec<u8>> = None;
    let mut in_result = false;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let name = e.name().as_ref().to_vec();
                if name == b"result" {
                    in_result = true;
                    current = Some(BoincTask {
                        name: None,
                        wu_name: None,
                        project_url: None,
                        state: None,
                        ready_to_report: None,
                        got_server_ack: None,
                        received_time: None,
                        report_deadline: None,
                        active_task: None,
                        active_task_state: None,
                        fraction_done: None,
                        elapsed_time: None,
                        estimated_cpu_time_remaining: None,
                    });
                    current_field = None;
                } else if in_result {
                    current_field = Some(name);
                }
            }
            Ok(Event::End(e)) => {
                if e.name().as_ref() == b"result" {
                    in_result = false;
                    if let Some(task) = current.take() {
                        tasks.push(task);
                    }
                    current_field = None;
                } else if in_result {
                    current_field = None;
                }
            }
            Ok(Event::Text(t)) => {
                if !in_result {
                    buf.clear();
                    continue;
                }
                let Some(field) = current_field.as_deref() else {
                    buf.clear();
                    continue;
                };
                let Ok(text) = t.unescape() else {
                    buf.clear();
                    continue;
                };
                let text = text.as_ref();
                if let Some(task) = current.as_mut() {
                    match field {
                        b"name" => task.name = Some(text.to_string()),
                        b"wu_name" => task.wu_name = Some(text.to_string()),
                        b"project_url" => task.project_url = Some(text.to_string()),
                        b"state" => task.state = text.parse::<i32>().ok(),
                        b"ready_to_report" => task.ready_to_report = parse_bool(text),
                        b"got_server_ack" => task.got_server_ack = parse_bool(text),
                        b"received_time" => task.received_time = text.parse::<f64>().ok(),
                        b"report_deadline" => task.report_deadline = text.parse::<f64>().ok(),
                        b"active_task" => task.active_task = parse_bool(text),
                        b"active_task_state" => task.active_task_state = text.parse::<i32>().ok(),
                        b"fraction_done" => task.fraction_done = text.parse::<f64>().ok(),
                        b"elapsed_time" => task.elapsed_time = text.parse::<f64>().ok(),
                        b"estimated_cpu_time_remaining" => {
                            task.estimated_cpu_time_remaining = text.parse::<f64>().ok()
                        }
                        _ => {}
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                return Err(BoincRpcError::Protocol(format!(
                    "XML parse error: {e}"
                )))
            }
            _ => {}
        }
        buf.clear();
    }

    Ok(tasks)
}

fn parse_bool(s: &str) -> Option<bool> {
    match s {
        "1" | "true" | "TRUE" | "True" => Some(true),
        "0" | "false" | "FALSE" | "False" => Some(false),
        _ => None,
    }
}

fn read_gui_rpc_password() -> Result<String, BoincRpcError> {
    #[cfg(target_os = "android")]
    {
        return Err(BoincRpcError::Unsupported(
            "reading gui_rpc_auth.cfg is not supported on Android".to_string(),
        ));
    }

    #[cfg(not(target_os = "android"))]
    {
        static PASSWORD: OnceLock<String> = OnceLock::new();
        if let Some(pwd) = PASSWORD.get() {
            return Ok(pwd.clone());
        }

        let candidates = gui_rpc_password_candidates();
        for path in candidates {
            if let Ok(contents) = fs::read_to_string(&path) {
                let pwd = contents.trim().to_string();
                if !pwd.is_empty() {
                    let _ = PASSWORD.set(pwd.clone());
                    return Ok(pwd);
                }
            }
        }

        Err(BoincRpcError::Io(
            "could not read gui_rpc_auth.cfg from BOINC data directory".to_string(),
        ))
    }
}

#[cfg(not(target_os = "android"))]
fn gui_rpc_password_candidates() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    if let Ok(dir) = std::env::var("BOINC_DATA_DIR") {
        paths.push(Path::new(&dir).join("gui_rpc_auth.cfg"));
    }

    paths.push(std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")).join("gui_rpc_auth.cfg"));

    #[cfg(target_os = "windows")]
    {
        if let Ok(program_data) = std::env::var("ProgramData") {
            paths.push(Path::new(&program_data).join("BOINC").join("gui_rpc_auth.cfg"));
        }
        paths.push(PathBuf::from(r"C:\ProgramData\BOINC\gui_rpc_auth.cfg"));
    }

    #[cfg(target_os = "macos")]
    {
        paths.push(PathBuf::from(
            "/Library/Application Support/BOINC Data/gui_rpc_auth.cfg",
        ));
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        paths.push(PathBuf::from("/var/lib/boinc-client/gui_rpc_auth.cfg"));
        paths.push(PathBuf::from("/var/lib/boinc/gui_rpc_auth.cfg"));
    }

    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_results() {
        let xml = r#"<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<boinc_gui_rpc_reply>
<results>
  <result>
    <name>task_1</name>
    <wu_name>wu_1</wu_name>
    <project_url>https://example.invalid/</project_url>
    <state>2</state>
    <ready_to_report>0</ready_to_report>
    <got_server_ack>1</got_server_ack>
    <received_time>1700000000.0</received_time>
    <report_deadline>1700003600.0</report_deadline>
    <active_task>1</active_task>
    <active_task_state>1</active_task_state>
    <fraction_done>0.25</fraction_done>
    <elapsed_time>120.0</elapsed_time>
    <estimated_cpu_time_remaining>360.0</estimated_cpu_time_remaining>
  </result>
</results>
</boinc_gui_rpc_reply>
\u0003"#;

        let tasks = parse_get_results_reply(xml).unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].name.as_deref(), Some("task_1"));
        assert_eq!(tasks[0].state, Some(2));
        assert_eq!(tasks[0].fraction_done, Some(0.25));
        assert_eq!(tasks[0].got_server_ack, Some(true));
    }
}

