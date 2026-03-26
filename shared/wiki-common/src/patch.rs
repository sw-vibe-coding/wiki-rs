use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct PatchRequest {
    pub etag: String,
    pub ops: Vec<PatchOp>,
}

#[derive(Deserialize)]
#[serde(tag = "op")]
pub enum PatchOp {
    #[serde(rename = "replace")]
    Replace {
        #[serde(rename = "match")]
        match_str: String,
        replace: String,
    },
    #[serde(rename = "append_after")]
    AppendAfter {
        #[serde(rename = "match")]
        match_str: String,
        text: String,
    },
    #[serde(rename = "insert_before")]
    InsertBefore {
        #[serde(rename = "match")]
        match_str: String,
        text: String,
    },
    #[serde(rename = "delete")]
    Delete {
        #[serde(rename = "match")]
        match_str: String,
    },
}

#[derive(Debug, Serialize)]
pub struct PatchError {
    pub op_index: usize,
    pub match_str: String,
}

/// Apply patch operations sequentially to content.
/// Each op works on the result of the previous op.
/// Returns the patched content, or an error if a match string was not found.
pub fn apply_ops(content: &str, ops: &[PatchOp]) -> Result<String, PatchError> {
    let mut result = content.to_string();
    for (i, op) in ops.iter().enumerate() {
        result = apply_one(&result, i, op)?;
    }
    Ok(result)
}

fn apply_one(content: &str, index: usize, op: &PatchOp) -> Result<String, PatchError> {
    match op {
        PatchOp::Replace { match_str, replace } => {
            if !content.contains(match_str.as_str()) {
                return Err(not_found(index, match_str));
            }
            Ok(content.replacen(match_str, replace, 1))
        }
        PatchOp::AppendAfter { match_str, text } => {
            let lines: Vec<&str> = content.lines().collect();
            let pos = lines
                .iter()
                .position(|l| l.contains(match_str.as_str()))
                .ok_or_else(|| not_found(index, match_str))?;
            let mut out = Vec::with_capacity(lines.len() + 1);
            out.extend_from_slice(&lines[..=pos]);
            out.push(text);
            out.extend_from_slice(&lines[pos + 1..]);
            Ok(join_preserving_trailing(content, &out))
        }
        PatchOp::InsertBefore { match_str, text } => {
            let lines: Vec<&str> = content.lines().collect();
            let pos = lines
                .iter()
                .position(|l| l.contains(match_str.as_str()))
                .ok_or_else(|| not_found(index, match_str))?;
            let mut out = Vec::with_capacity(lines.len() + 1);
            out.extend_from_slice(&lines[..pos]);
            out.push(text);
            out.extend_from_slice(&lines[pos..]);
            Ok(join_preserving_trailing(content, &out))
        }
        PatchOp::Delete { match_str } => {
            if !content.contains(match_str.as_str()) {
                return Err(not_found(index, match_str));
            }
            Ok(content.replacen(match_str, "", 1))
        }
    }
}

fn not_found(op_index: usize, match_str: &str) -> PatchError {
    PatchError {
        op_index,
        match_str: match_str.to_string(),
    }
}

fn join_preserving_trailing(original: &str, lines: &[&str]) -> String {
    let mut joined = lines.join("\n");
    if original.ends_with('\n') {
        joined.push('\n');
    }
    joined
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_op_works() {
        let content = "| REQ-003 | REQUESTED |\n| REQ-004 | DONE |";
        let ops = vec![PatchOp::Replace {
            match_str: "| REQ-003 | REQUESTED |".to_string(),
            replace: "| REQ-003 | IN PROGRESS |".to_string(),
        }];
        let result = apply_ops(content, &ops).unwrap();
        assert_eq!(result, "| REQ-003 | IN PROGRESS |\n| REQ-004 | DONE |");
    }

    #[test]
    fn replace_only_first_occurrence() {
        let content = "AAA\nAAA\nBBB";
        let ops = vec![PatchOp::Replace {
            match_str: "AAA".to_string(),
            replace: "ZZZ".to_string(),
        }];
        let result = apply_ops(content, &ops).unwrap();
        assert_eq!(result, "ZZZ\nAAA\nBBB");
    }

    #[test]
    fn append_after_inserts_on_next_line() {
        let content = "## Requirements\n| REQ-001 | DONE |";
        let ops = vec![PatchOp::AppendAfter {
            match_str: "## Requirements".to_string(),
            text: "| REQ-002 | NEW |".to_string(),
        }];
        let result = apply_ops(content, &ops).unwrap();
        assert_eq!(
            result,
            "## Requirements\n| REQ-002 | NEW |\n| REQ-001 | DONE |"
        );
    }

    #[test]
    fn insert_before_inserts_above_line() {
        let content = "## Header\n## Footer";
        let ops = vec![PatchOp::InsertBefore {
            match_str: "## Footer".to_string(),
            text: "## Notes".to_string(),
        }];
        let result = apply_ops(content, &ops).unwrap();
        assert_eq!(result, "## Header\n## Notes\n## Footer");
    }

    #[test]
    fn delete_removes_match() {
        let content = "line1\ndelete-me\nline3";
        let ops = vec![PatchOp::Delete {
            match_str: "delete-me\n".to_string(),
        }];
        let result = apply_ops(content, &ops).unwrap();
        assert_eq!(result, "line1\nline3");
    }

    #[test]
    fn match_not_found_returns_error() {
        let content = "hello world";
        let ops = vec![PatchOp::Replace {
            match_str: "no such text".to_string(),
            replace: "x".to_string(),
        }];
        let err = apply_ops(content, &ops).unwrap_err();
        assert_eq!(err.op_index, 0);
        assert_eq!(err.match_str, "no such text");
    }

    #[test]
    fn ops_apply_sequentially() {
        let content = "A\nB\nC";
        let ops = vec![
            PatchOp::Replace {
                match_str: "A".to_string(),
                replace: "X".to_string(),
            },
            PatchOp::Replace {
                match_str: "X".to_string(),
                replace: "Y".to_string(),
            },
        ];
        let result = apply_ops(content, &ops).unwrap();
        assert_eq!(result, "Y\nB\nC");
    }

    #[test]
    fn preserves_trailing_newline() {
        let content = "line1\nline2\n";
        let ops = vec![PatchOp::AppendAfter {
            match_str: "line1".to_string(),
            text: "inserted".to_string(),
        }];
        let result = apply_ops(content, &ops).unwrap();
        assert!(result.ends_with('\n'));
        assert_eq!(result, "line1\ninserted\nline2\n");
    }
}
