use std::collections::VecDeque;

#[derive(PartialEq)]
enum PaddingFormat {
    None,
    Space,
    Zero,
}

fn pad_string(in_str: String, format_info: String) -> Result<String, String> {
    if format_info.is_empty() {
        return Ok(in_str);
    }

    let mut left_align = false;
    let mut idx_cur = 0;

    let format_info_chars: Vec<char> = format_info.chars().collect();
    if idx_cur < format_info_chars.len() && format_info_chars[idx_cur] == '-' {
        left_align = true;
        idx_cur += 1;
    }

    let mut padding_format = PaddingFormat::None;
    if idx_cur < format_info_chars.len() {
        padding_format = match format_info_chars[idx_cur] {
            ' ' => {
                idx_cur += 1;
                PaddingFormat::Space
            }
            '0' => {
                idx_cur += 1;
                PaddingFormat::Zero
            }
            _ => PaddingFormat::Space,
        };
    }

    if format_info.contains('.') {
        panic!("TODO: double format not supported");
    }

    let width = match format_info[idx_cur..].parse::<usize>() {
        Ok(val) => val,
        Err(e) => return Err(e.to_string()),
    };
    // .unwrap_or_else(|_| Err(format!("Invalid format [{}]", format_info)));
    // .unwrap_or_else(|_| panic!("Invalid format [{}]", format_info));

    if width < in_str.len() {
        return Ok(in_str);
    }

    let width_delta = width - in_str.len();

    let padding_str = match padding_format {
        PaddingFormat::Space => " ".repeat(width_delta),
        PaddingFormat::Zero => "0".repeat(width_delta),
        _ => "".to_string(),
    };

    if left_align {
        return Ok(in_str + &padding_str);
    }
    Ok(padding_str + &in_str)
}

pub enum FormatArg {
    Int(i64),
    Uint(u64),
    Char(char),
    Str(String),
    Float(f64),
}

pub fn format(format: &str, in_args: Vec<FormatArg>) -> Result<String, String> {
    let mut args: VecDeque<FormatArg> = VecDeque::from(in_args);

    let mut result_string = String::new();

    let format_chars: Vec<_> = format.chars().collect();

    let mut last_fmt_idx = 0;
    let mut idx = 0;
    while idx < (format_chars.len() - 1) {
        if format_chars[idx] != '%' {
            idx += 1;
            continue;
        }

        let mut end_idx = idx + 1;
        if end_idx < format_chars.len()
            && (format_chars[end_idx] == '-' || format_chars[end_idx] == ' ')
        {
            end_idx += 1;
        }

        while end_idx < format_chars.len() && format_chars[end_idx].is_ascii_digit() {
            end_idx += 1;
        }

        // let number_str = &format[idx + 1..end_idx];
        // let mu

        if end_idx >= format_chars.len() {
            break;
        }

        if format_chars[end_idx] == '.' {
            end_idx += 1;

            // let mut num_2_start = end_idx;
            while end_idx < format_chars.len() && format_chars[end_idx].is_ascii_digit() {
                end_idx += 1;
            }

            if end_idx >= format_chars.len() {
                break;
            }

            todo!("TODO: add float comma specifier stuff");
            return Err("TODO: add float comma specifier stuff".into());
            // number_str = format[idx + 1..end_idx];
        }

        // add string up untill fmt
        if last_fmt_idx != idx {
            result_string.push_str(&format[last_fmt_idx..idx]);
        }

        let fmt_str = &format[idx + 1..end_idx];

        idx = end_idx;
        last_fmt_idx = end_idx + 1;

        let fmt_type = format_chars[idx];

        let formatted_str = match fmt_type {
            'X' => match args.pop_front() {
                Some(FormatArg::Uint(val)) => {
                    &pad_string(format!("{:X}", val), fmt_str.to_string())?
                }
                Some(FormatArg::Int(val)) => {
                    &pad_string(format!("{:X}", val), fmt_str.to_string())?
                }
                Some(FormatArg::Char(val)) => {
                    &pad_string(format!("{:X}", val as i64), fmt_str.to_string())?
                }
                _ => return Err("Invalid format!".into()),
            },
            'x' => match args.pop_front() {
                Some(FormatArg::Uint(val)) => {
                    &pad_string(format!("{:x}", val), fmt_str.to_string())?
                }
                Some(FormatArg::Int(val)) => {
                    &pad_string(format!("{:x}", val), fmt_str.to_string())?
                }
                Some(FormatArg::Char(val)) => {
                    &pad_string(format!("{:x}", val as i64), fmt_str.to_string())?
                }
                _ => return Err("Invalid format!".into()),
            },

            'c' => match args.pop_front() {
                Some(FormatArg::Uint(val)) => {
                    &pad_string(format!("{}", (val as u8) as char), fmt_str.to_string())?
                }
                Some(FormatArg::Int(val)) => {
                    &pad_string(format!("{}", (val as u8) as char), fmt_str.to_string())?
                }
                Some(FormatArg::Char(val)) => &pad_string(format!("{}", val), fmt_str.to_string())?,
                _ => return Err("Invalid format!".into()),
            },

            'D' | 'd' => match args.pop_front() {
                Some(FormatArg::Uint(val)) => &pad_string(format!("{}", val), fmt_str.to_string())?,
                Some(FormatArg::Int(val)) => &pad_string(format!("{}", val), fmt_str.to_string())?,
                Some(FormatArg::Char(val)) => {
                    &pad_string(format!("{}", val as i64), fmt_str.to_string())?
                }
                _ => return Err("Invalid format!".into()),
            },

            's' => {
                match args.pop_front() {
                    // Some(FormatArg::Uint(val)) => &pad_string(format!("{}", val), fmt_str.to_string()),
                    // Some(FormatArg::Int(val)) => &pad_string(format!("{}", val), fmt_str.to_string()),
                    // Some(FormatArg::Char(val)) => &pad_string(format!("{}", val as i64), fmt_str.to_string()),
                    Some(FormatArg::Str(val)) => &pad_string(val, fmt_str.to_string())?,
                    _ => return Err("Invalid format!".into()),
                }
            }

            '%' => "%",

            // _ => panic!("unsupported"),
            _ => return Err("unsupported format type".into()),
        };

        result_string.push_str(formatted_str);

        idx += 1;
    }

    if last_fmt_idx != idx {
        result_string.push_str(&format[last_fmt_idx..]);
    }

    Ok(result_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string() {
        assert_eq!(
            format(
                "%s|% 10s",
                vec![
                    FormatArg::Str("test1".to_string()),
                    FormatArg::Str("test2".to_string()),
                ]
            )
            .expect("failed format"),
            "test|     test2"
        );
    }
}
