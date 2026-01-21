use crate::errors::GitError;

pub struct GitIgnore {
    pub patterns: Vec<String>,
}

impl GitIgnore {
    pub fn new() -> Self {
        GitIgnore {
            patterns: Vec::new(),
        }
    }

    pub fn read_patterns(&mut self, file_path: &str) -> Result<(), GitError> {
        let content = std::fs::read_to_string(file_path)?;
        self.parse(&content);
        Ok(())
    }

    fn parse(&mut self, content: &str) {
        for line in content.lines() {
            let raw_line = line.trim_end_matches('\r');
            if raw_line.trim().is_empty() {
                continue;
            }
            if raw_line.starts_with('#') {
                continue;
            }

            let mut cleaned = trim_trailing_spaces(raw_line);
            if cleaned.is_empty() {
                continue;
            }
            if cleaned.starts_with("\\#") || cleaned.starts_with("\\!") {
                cleaned.remove(0);
            }
            if !cleaned.is_empty() {
                self.patterns.push(cleaned);
            }
        }
    }

    pub fn matches(&self, file_path: &str) -> bool {
        let (path, is_dir) = normalize_path(file_path);
        if path.is_empty() {
            return false;
        }

        let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        if !segments.is_empty() {
            let parent_len = if is_dir {
                segments.len().saturating_sub(1)
            } else {
                segments.len().saturating_sub(1)
            };

            for i in 1..=parent_len {
                let parent = segments[..i].join("/");
                if matches_path(&self.patterns, &parent, true) {
                    return true;
                }
            }
        }

        matches_path(&self.patterns, &path, is_dir)
    }
}

fn normalize_path(file_path: &str) -> (String, bool) {
    let mut path = file_path.replace('\\', "/");
    let is_dir = path.ends_with('/');

    while path.starts_with("./") {
        path = path[2..].to_string();
    }

    if path.starts_with('/') {
        path = path[1..].to_string();
    }

    let mut cleaned = String::new();
    for part in path.split('/').filter(|s| !s.is_empty()) {
        if !cleaned.is_empty() {
            cleaned.push('/');
        }
        cleaned.push_str(part);
    }

    (cleaned, is_dir)
}

fn trim_trailing_spaces(line: &str) -> String {
    let mut chars: Vec<char> = line.chars().collect();
    let mut end = chars.len();

    while end > 0 && chars[end - 1].is_whitespace() {
        end -= 1;
    }

    if end == chars.len() {
        return line.to_string();
    }

    let mut escaped_space = false;
    if end > 0 && chars[end - 1] == '\\' {
        let mut count = 1;
        let mut idx = end - 1;
        while idx > 0 && chars[idx - 1] == '\\' {
            count += 1;
            idx -= 1;
        }
        if count % 2 == 1 {
            escaped_space = true;
            chars.remove(end - 1);
            end -= 1;
        }
    }

    let mut result: String = chars[..end].iter().collect();
    if escaped_space {
        result.push(' ');
    }
    result
}

fn matches_path(patterns: &[String], path: &str, is_dir: bool) -> bool {
    let mut ignored = false;
    for raw_pattern in patterns {
        if let Some(pattern) = ParsedPattern::from(raw_pattern) {
            if pattern.matches(path, is_dir) {
                ignored = !pattern.negated;
            }
        }
    }
    ignored
}

#[derive(Debug, Clone)]
struct ParsedPattern {
    pattern: String,
    negated: bool,
    dir_only: bool,
    has_slash: bool,
}

impl ParsedPattern {
    fn from(raw: &str) -> Option<Self> {
        let mut line = raw.trim_end_matches('\r').to_string();
        if line.trim().is_empty() {
            return None;
        }

        let mut negated = false;
        if line.starts_with("\\#") || line.starts_with("\\!") {
            line.remove(0);
        } else if line.starts_with('!') {
            negated = true;
            line.remove(0);
        }

        if line.is_empty() {
            return None;
        }

        let dir_only = ends_with_unescaped_slash(&line);
        if dir_only {
            line.pop();
        }

        if line.starts_with('/') {
            line.remove(0);
        }

        let has_slash = line.contains('/');

        Some(ParsedPattern {
            pattern: line,
            negated,
            dir_only,
            has_slash,
        })
    }

    fn matches(&self, path: &str, is_dir: bool) -> bool {
        if self.pattern.is_empty() {
            return false;
        }

        let path_segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

        if self.has_slash {
            let pattern_segments: Vec<String> = self
                .pattern
                .split('/')
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect();

            if self.dir_only {
                return match_segments_prefix(&pattern_segments, &path_segments, is_dir);
            }
            return match_segments_full(&pattern_segments, &path_segments);
        }

        if self.dir_only {
            return match_dir_components(&self.pattern, &path_segments, is_dir);
        }

        match_any_component(&self.pattern, &path_segments)
    }
}

fn ends_with_unescaped_slash(value: &str) -> bool {
    if !value.ends_with('/') {
        return false;
    }

    let mut backslashes = 0;
    for ch in value.chars().rev().skip(1) {
        if ch == '\\' {
            backslashes += 1;
        } else {
            break;
        }
    }

    backslashes % 2 == 0
}

fn match_any_component(pattern: &str, segments: &[&str]) -> bool {
    segments
        .iter()
        .any(|segment| glob_match_segment(pattern, segment))
}

fn match_dir_components(pattern: &str, segments: &[&str], is_dir: bool) -> bool {
    if segments.is_empty() {
        return false;
    }

    let last_index = if is_dir {
        segments.len()
    } else {
        segments.len().saturating_sub(1)
    };

    segments
        .iter()
        .take(last_index)
        .any(|segment| glob_match_segment(pattern, segment))
}

fn match_segments_full(pattern: &[String], path: &[&str]) -> bool {
    let mut memo = vec![vec![None; path.len() + 1]; pattern.len() + 1];
    match_segments_full_impl(pattern, path, 0, 0, &mut memo)
}

fn match_segments_full_impl(
    pattern: &[String],
    path: &[&str],
    p_idx: usize,
    s_idx: usize,
    memo: &mut Vec<Vec<Option<bool>>>,
) -> bool {
    if let Some(result) = memo[p_idx][s_idx] {
        return result;
    }

    let result = if p_idx == pattern.len() {
        s_idx == path.len()
    } else if pattern[p_idx] == "**" {
        let mut matched = match_segments_full_impl(pattern, path, p_idx + 1, s_idx, memo);
        if !matched && s_idx < path.len() {
            matched = match_segments_full_impl(pattern, path, p_idx, s_idx + 1, memo);
        }
        matched
    } else if s_idx >= path.len() {
        false
    } else if glob_match_segment(&pattern[p_idx], path[s_idx]) {
        match_segments_full_impl(pattern, path, p_idx + 1, s_idx + 1, memo)
    } else {
        false
    };

    memo[p_idx][s_idx] = Some(result);
    result
}

fn match_segments_prefix(pattern: &[String], path: &[&str], is_dir: bool) -> bool {
    let mut memo = vec![vec![None; path.len() + 1]; pattern.len() + 1];
    match_segments_prefix_impl(pattern, path, is_dir, 0, 0, &mut memo)
}

fn match_segments_prefix_impl(
    pattern: &[String],
    path: &[&str],
    is_dir: bool,
    p_idx: usize,
    s_idx: usize,
    memo: &mut Vec<Vec<Option<bool>>>,
) -> bool {
    if let Some(result) = memo[p_idx][s_idx] {
        return result;
    }

    let result = if p_idx == pattern.len() {
        s_idx < path.len() || (is_dir && s_idx == path.len())
    } else if pattern[p_idx] == "**" {
        let mut matched = match_segments_prefix_impl(pattern, path, is_dir, p_idx + 1, s_idx, memo);
        if !matched && s_idx < path.len() {
            matched = match_segments_prefix_impl(pattern, path, is_dir, p_idx, s_idx + 1, memo);
        }
        matched
    } else if s_idx >= path.len() {
        false
    } else if glob_match_segment(&pattern[p_idx], path[s_idx]) {
        match_segments_prefix_impl(pattern, path, is_dir, p_idx + 1, s_idx + 1, memo)
    } else {
        false
    };

    memo[p_idx][s_idx] = Some(result);
    result
}

fn glob_match_segment(pattern: &str, text: &str) -> bool {
    let p: Vec<char> = pattern.chars().collect();
    let t: Vec<char> = text.chars().collect();
    let mut memo = vec![vec![None; t.len() + 1]; p.len() + 1];
    glob_match_impl(&p, &t, 0, 0, &mut memo)
}

fn glob_match_impl(
    pattern: &[char],
    text: &[char],
    p_idx: usize,
    t_idx: usize,
    memo: &mut Vec<Vec<Option<bool>>>,
) -> bool {
    if let Some(result) = memo[p_idx][t_idx] {
        return result;
    }

    let result = if p_idx == pattern.len() {
        t_idx == text.len()
    } else {
        match pattern[p_idx] {
            '*' => {
                let mut matched = glob_match_impl(pattern, text, p_idx + 1, t_idx, memo);
                if !matched && t_idx < text.len() {
                    matched = glob_match_impl(pattern, text, p_idx, t_idx + 1, memo);
                }
                matched
            }
            '?' => t_idx < text.len() && glob_match_impl(pattern, text, p_idx + 1, t_idx + 1, memo),
            '[' => match_char_class(pattern, text, p_idx, t_idx, memo),
            '\\' => {
                if p_idx + 1 >= pattern.len() {
                    t_idx < text.len()
                        && text[t_idx] == '\\'
                        && glob_match_impl(pattern, text, p_idx + 1, t_idx + 1, memo)
                } else if t_idx < text.len() && text[t_idx] == pattern[p_idx + 1] {
                    glob_match_impl(pattern, text, p_idx + 2, t_idx + 1, memo)
                } else {
                    false
                }
            }
            ch => {
                t_idx < text.len()
                    && text[t_idx] == ch
                    && glob_match_impl(pattern, text, p_idx + 1, t_idx + 1, memo)
            }
        }
    };

    memo[p_idx][t_idx] = Some(result);
    result
}

fn match_char_class(
    pattern: &[char],
    text: &[char],
    p_idx: usize,
    t_idx: usize,
    memo: &mut Vec<Vec<Option<bool>>>,
) -> bool {
    if t_idx >= text.len() {
        return false;
    }

    let mut end = p_idx + 1;
    while end < pattern.len() && pattern[end] != ']' {
        if pattern[end] == '\\' && end + 1 < pattern.len() {
            end += 2;
        } else {
            end += 1;
        }
    }

    if end >= pattern.len() {
        return text[t_idx] == '[' && glob_match_impl(pattern, text, p_idx + 1, t_idx + 1, memo);
    }

    let class = &pattern[p_idx + 1..end];
    let mut idx = 0;
    let mut negated = false;
    if idx < class.len() && (class[idx] == '!' || class[idx] == '^') {
        negated = true;
        idx += 1;
    }

    let mut matched = false;
    let mut prev: Option<char> = None;
    while idx < class.len() {
        let ch = if class[idx] == '\\' && idx + 1 < class.len() {
            idx += 1;
            class[idx]
        } else {
            class[idx]
        };

        if ch == '-' && prev.is_some() && idx + 1 < class.len() && class[idx + 1] != ']' {
            let start = prev.unwrap();
            let mut end_ch = class[idx + 1];
            if end_ch == '\\' && idx + 2 < class.len() {
                end_ch = class[idx + 2];
                idx += 1;
            }
            if (start <= text[t_idx] && text[t_idx] <= end_ch)
                || (end_ch <= text[t_idx] && text[t_idx] <= start)
            {
                matched = true;
            }
            idx += 1;
            prev = Some(end_ch);
        } else {
            if text[t_idx] == ch {
                matched = true;
            }
            prev = Some(ch);
        }
        idx += 1;
    }

    let class_match = if negated { !matched } else { matched };
    if class_match {
        glob_match_impl(pattern, text, end + 1, t_idx + 1, memo)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_gitignore(patterns: &str) -> GitIgnore {
        let mut gi = GitIgnore::new();
        gi.parse(patterns);
        gi
    }

    #[test]
    fn matches_simple_globs() {
        let gi = build_gitignore("*.log\nfile?.txt\n[ab].tmp\n");

        assert!(gi.matches("debug.log"));
        assert!(gi.matches("file1.txt"));
        assert!(gi.matches("a.tmp"));
        assert!(!gi.matches("file10.txt"));
        assert!(!gi.matches("c.tmp"));
    }

    #[test]
    fn matches_directory_only_patterns() {
        let gi = build_gitignore("build/\nlogs/\n");

        assert!(gi.matches("build/"));
        assert!(gi.matches("build/output.o"));
        assert!(gi.matches("logs/debug.log"));
        assert!(!gi.matches("build.log"));
    }

    #[test]
    fn matches_anchored_patterns() {
        let gi = build_gitignore("/temp\n");

        assert!(gi.matches("temp"));
        assert!(!gi.matches("dir/temp"));
    }

    #[test]
    fn matches_double_star_patterns() {
        let gi = build_gitignore("**/foo\nabc/**\na/**/b\n");

        assert!(gi.matches("foo"));
        assert!(gi.matches("dir/foo"));
        assert!(gi.matches("abc/one/two.txt"));
        assert!(gi.matches("a/b"));
        assert!(gi.matches("a/x/y/b"));
        assert!(!gi.matches("a/x/y/c"));
    }

    #[test]
    fn matches_negation_patterns() {
        let gi = build_gitignore("*.log\n!important.log\n");

        assert!(gi.matches("debug.log"));
        assert!(!gi.matches("important.log"));
    }

    #[test]
    fn parent_directory_exclusion_blocks_reinclude() {
        let gi = build_gitignore("build/\n!build/main.js\n");

        assert!(gi.matches("build/main.js"));
        assert!(gi.matches("build/other.js"));
    }

    #[test]
    fn reinclude_within_directory_when_parent_not_ignored() {
        let gi = build_gitignore("build/*\n!build/main.js\n");

        assert!(!gi.matches("build/main.js"));
        assert!(gi.matches("build/other.js"));
    }
}
