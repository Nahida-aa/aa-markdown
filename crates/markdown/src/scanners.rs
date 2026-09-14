use crate::types::HeadingLevel;

pub fn scan_atx_heading(line: &str) -> Option<HeadingLevel> {
    let level = line.bytes().take_while(|&i| i == b'#').count();
    if level == 0 {
        return None;
    }
    if !matches!(line.as_bytes().get(level), None | Some(b' ') | Some(b'\t')) {
        return None;
    }
    HeadingLevel::try_from(level).ok()
}

/// 这一行是否打断正在累积的段落？
///
/// 目前只有标题会打断段落。
pub fn is_paragraph_interrupt(line: &str) -> bool {
    scan_atx_heading(line).is_some()
}
