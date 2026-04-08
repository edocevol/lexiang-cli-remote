use serde_json::Value;

pub fn print_table(value: &Value) {
    let data = value.get("data").unwrap_or(value);

    match data {
        Value::Array(arr) => print_array_table(arr),
        Value::Object(obj) => {
            for (key, val) in obj {
                if let Value::Array(arr) = val {
                    if !arr.is_empty() {
                        println!("{}:", key);
                        print_array_table(arr);
                        return;
                    }
                }
            }
            for (key, val) in obj {
                println!("{}: {}", key, format_value(key, val));
            }
        }
        _ => println!("{}", format_value("", data)),
    }
}

fn print_array_table(arr: &[Value]) {
    if arr.is_empty() {
        println!("(empty)");
        return;
    }

    if let Some(Value::Object(first)) = arr.first() {
        let columns: Vec<&str> = first.keys().map(std::string::String::as_str).collect();

        let mut widths: Vec<usize> = columns.iter().map(|c| c.len()).collect();
        for item in arr {
            if let Value::Object(obj) = item {
                for (i, col) in columns.iter().enumerate() {
                    let val_len = format_value(col, obj.get(*col).unwrap_or(&Value::Null)).len();
                    if val_len > widths[i] {
                        widths[i] = val_len.min(40);
                    }
                }
            }
        }

        let header: Vec<String> = columns
            .iter()
            .zip(&widths)
            .map(|(c, w)| format!("{:<width$}", c, width = *w))
            .collect();
        println!("{}", header.join(" | "));
        println!(
            "{}",
            widths
                .iter()
                .map(|w| "-".repeat(*w))
                .collect::<Vec<_>>()
                .join("-+-")
        );

        for item in arr {
            if let Value::Object(obj) = item {
                let row: Vec<String> = columns
                    .iter()
                    .zip(&widths)
                    .map(|(col, w)| {
                        let val = format_value(col, obj.get(*col).unwrap_or(&Value::Null));
                        if val.len() > *w {
                            format!("{:.width$}", val, width = w - 1).to_string() + "…"
                        } else {
                            format!("{:<width$}", val, width = *w)
                        }
                    })
                    .collect();
                println!("{}", row.join(" | "));
            }
        }
    } else {
        for item in arr {
            println!("{}", format_value("", item));
        }
    }
}

pub fn print_csv(value: &Value) {
    let data = value.get("data").unwrap_or(value);

    if let Value::Object(obj) = data {
        for (_, val) in obj {
            if let Value::Array(arr) = val {
                if !arr.is_empty() {
                    if let Some(Value::Object(first)) = arr.first() {
                        let columns: Vec<&str> =
                            first.keys().map(std::string::String::as_str).collect();
                        println!("{}", columns.join(","));

                        for item in arr {
                            if let Value::Object(obj) = item {
                                let row: Vec<String> = columns
                                    .iter()
                                    .map(|col| {
                                        let val = format_value(
                                            col,
                                            obj.get(*col).unwrap_or(&Value::Null),
                                        );
                                        if val.contains(',') || val.contains('"') {
                                            format!("\"{}\"", val.replace('"', "\"\""))
                                        } else {
                                            val
                                        }
                                    })
                                    .collect();
                                println!("{}", row.join(","));
                            }
                        }
                        return;
                    }
                }
            }
        }
    }

    println!("{}", serde_json::to_string(value).unwrap_or_default());
}

pub fn print_markdown(value: &Value) {
    let data = value.get("data").unwrap_or(value);

    if let Value::Object(obj) = data {
        for (key, val) in obj {
            if let Value::Array(arr) = val {
                if !arr.is_empty() {
                    println!("## {}\n", key);
                    if let Some(Value::Object(first)) = arr.first() {
                        let columns: Vec<&str> =
                            first.keys().map(std::string::String::as_str).collect();
                        println!("| {} |", columns.join(" | "));
                        println!(
                            "| {} |",
                            columns
                                .iter()
                                .map(|_| "---")
                                .collect::<Vec<_>>()
                                .join(" | ")
                        );

                        for item in arr {
                            if let Value::Object(obj) = item {
                                let row: Vec<String> = columns
                                    .iter()
                                    .map(|col| {
                                        format_value(col, obj.get(*col).unwrap_or(&Value::Null))
                                    })
                                    .collect();
                                println!("| {} |", row.join(" | "));
                            }
                        }
                        return;
                    }
                }
            }
        }
    }

    println!(
        "```json\n{}\n```",
        serde_json::to_string_pretty(value).unwrap_or_default()
    );
}

fn format_value(key: &str, value: &Value) -> String {
    match value {
        Value::Null => String::new(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => {
            if key.ends_with("_at") {
                if let Some(n) = n.as_i64() {
                    // Go zero-value time (0001-01-01T00:00:00Z) or negative
                    if n <= 0 {
                        return "-".to_string();
                    }
                    // Millisecond timestamps (13 digits, > 10_000_000_000)
                    if n > 10_000_000_000 {
                        if let Some(dt) = chrono::DateTime::from_timestamp(
                            n / 1000,
                            ((n % 1000) * 1_000_000) as u32,
                        ) {
                            return dt
                                .with_timezone(&chrono::Local)
                                .format("%Y-%m-%d %H:%M:%S")
                                .to_string();
                        }
                    }
                    // Second timestamps (10 digits)
                    if n > 0 {
                        if let Some(dt) = chrono::DateTime::from_timestamp(n, 0) {
                            return dt
                                .with_timezone(&chrono::Local)
                                .format("%Y-%m-%d %H:%M:%S")
                                .to_string();
                        }
                    }
                }
            }
            n.to_string()
        }
        Value::String(s) => s.clone(),
        Value::Array(arr) => {
            let items: Vec<String> = arr.iter().map(|v| format_value("", v)).collect();
            format!("[{}]", items.join(", "))
        }
        Value::Object(obj) => {
            if let Some(name) = obj.get("name").and_then(|v| v.as_str()) {
                name.to_string()
            } else if let Some(id) = obj.get("id").and_then(|v| v.as_str()) {
                id.to_string()
            } else {
                serde_json::to_string(obj).unwrap_or_else(|_| "{...}".to_string())
            }
        }
    }
}
