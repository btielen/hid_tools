struct Report {
    size: usize,
    raw: Vec<u8>
}

struct ParsedReport {
    report_id: Option<u32>,
    field: Vec<Field>
}

/// A discrete section of data within a report.
struct Field {
    usage_page: u16,
    usage: u16,
    value: i32
}