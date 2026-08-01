use crate::engine::CheckResult;
use crate::results::parse_checks::parse_checks;
use hypernode::{HyperNode, Node, PropValue};
use std::fs;
use std::path::PathBuf;

/// Write a minimal PDF summary of the ResultsNode in the working directory.
pub fn export_results_pdf(results: &Node) -> std::io::Result<PathBuf> {
    let wall_id = match results.get_prop("wall_id") {
        Some(PropValue::I64(v)) => *v,
        _ => 0,
    };
    let path = PathBuf::from(format!("results_wall_{wall_id}.pdf"));
    let checks = parse_checks(results);
    let governing = match results.get_prop("governing") {
        Some(PropValue::Text(s)) => s.as_str(),
        _ => "—",
    };
    let code = match results.get_prop("code_ref") {
        Some(PropValue::Text(s)) => s.as_str(),
        _ => "ACI 318",
    };
    let overall = match results.get_prop("overall_pass") {
        Some(PropValue::Bool(true)) => "PASS",
        Some(PropValue::Bool(false)) => "FAIL",
        _ => "—",
    };
    let ts = match results.get_prop("run_timestamp") {
        Some(PropValue::I64(v)) => v.to_string(),
        _ => "0".into(),
    };

    let mut lines = vec![
        "Innovator - Wall Analysis Results".into(),
        format!("Wall id: {wall_id}"),
        format!("Status: {overall}"),
        format!("Governing: {governing}"),
        format!("Code: {code}"),
        format!("Run timestamp: {ts}"),
        String::new(),
        format!(
            "{:<28} {:>10} {:>10} {:>8} {:>6}",
            "Check", "Demand", "Capacity", "Ratio", "Pass"
        ),
        "-".repeat(68),
    ];
    for c in &checks {
        lines.push(format_check_line(c));
    }
    write_simple_pdf(&path, &lines.join("\n"))?;
    Ok(path)
}

fn format_check_line(c: &CheckResult) -> String {
    if c.informational {
        format!(
            "{:<28} {:>10.3} {:>10} {:>8} {:>6}",
            c.name, c.demand, "-", "-", "info"
        )
    } else {
        format!(
            "{:<28} {:>10.3} {:>10.3} {:>8.3} {:>6}",
            c.name,
            c.demand,
            c.capacity,
            c.ratio,
            if c.pass { "PASS" } else { "FAIL" }
        )
    }
}

fn write_simple_pdf(path: &PathBuf, text: &str) -> std::io::Result<()> {
    let mut content = String::from("BT /F1 10 Tf 50 750 Td 12 TL\n");
    for line in text.lines() {
        let escaped = line
            .replace('\\', "\\\\")
            .replace('(', "\\(")
            .replace(')', "\\)");
        content.push_str(&format!("({escaped}) '\n"));
    }
    content.push_str("ET");

    let stream = format!("<< /Length {} >>\nstream\n{content}\nendstream", content.len());
    let objects = [
        "<< /Type /Catalog /Pages 2 0 R >>".to_string(),
        "<< /Type /Pages /Kids [3 0 R] /Count 1 >>".to_string(),
        "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Contents 4 0 R /Resources << /Font << /F1 5 0 R >> >> >>".to_string(),
        stream,
        "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>".to_string(),
    ];

    let mut buf: Vec<u8> = Vec::new();
    buf.extend_from_slice(b"%PDF-1.4\n");
    let mut offsets = Vec::with_capacity(objects.len() + 1);
    offsets.push(0);
    for (i, obj) in objects.iter().enumerate() {
        offsets.push(buf.len() as u32);
        buf.extend_from_slice(format!("{} 0 obj\n", i + 1).as_bytes());
        buf.extend_from_slice(obj.as_bytes());
        buf.extend_from_slice(b"\nendobj\n");
    }
    let xref_pos = buf.len();
    buf.extend_from_slice(format!("xref\n0 {}\n0000000000 65535 f \n", objects.len() + 1).as_bytes());
    for off in offsets.iter().skip(1) {
        buf.extend_from_slice(format!("{off:010} 00000 n \n").as_bytes());
    }
    buf.extend_from_slice(
        format!(
            "trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{xref_pos}\n%%EOF\n",
            objects.len() + 1
        )
        .as_bytes(),
    );
    fs::write(path, buf)
}
