#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Edit(Vec<String>),
    List(Vec<String>),
    New(Vec<String>),
    Status(Vec<String>),
    Version,
}

pub enum ListArgs {}

pub fn parse_command(user_entry: Vec<String>) -> Option<Command> {
    match user_entry.split_first() {
        Some((command, additional_args)) => match command.clone().as_ref() {
            "new" | "n" => Some(Command::New(additional_args.to_vec())),
            "list" | "l" => Some(Command::List(additional_args.to_vec())),
            "status" | "st" => Some(Command::Status(additional_args.to_vec())),
            "edit" | "e" => Some(Command::Edit(additional_args.to_vec())),
            "version" | "v" => Some(Command::Version),
            _ => None,
        },
        _ => None,
    }
}

fn parse_list_attributes(args: Vec<String>) -> Vec<ListArgs> {
    vec![]
}

#[test]
fn test_parse_command_new() {
    let expected = Command::New(vec!["hello".to_string(), "world".to_string()]);
    let output = parse_command(vec![
        "new".to_string(),
        "hello".to_string(),
        "world".to_string(),
    ])
    .unwrap();

    assert_eq!(expected, output);
}

#[test]
fn test_parse_command_edit() {
    let expected = Command::Edit(vec!["hello".to_string(), "world".to_string()]);
    let output = parse_command(vec![
        "edit".to_string(),
        "hello".to_string(),
        "world".to_string(),
    ])
    .unwrap();

    assert_eq!(expected, output);
}

#[test]
fn test_parse_command_list() {
    let expected = Command::List(vec!["hello".to_string(), "world".to_string()]);
    let output = parse_command(vec![
        "list".to_string(),
        "hello".to_string(),
        "world".to_string(),
    ])
    .unwrap();

    assert_eq!(expected, output);
}

#[test]
fn test_parse_command_status() {
    let expected = Command::Status(vec!["hello".to_string(), "world".to_string()]);
    let output = parse_command(vec![
        "status".to_string(),
        "hello".to_string(),
        "world".to_string(),
    ])
    .unwrap();

    assert_eq!(expected, output);
}
