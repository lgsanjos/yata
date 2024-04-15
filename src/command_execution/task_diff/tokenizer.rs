use regex::Regex;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ParserTask {
    pub title: String,
    pub id: Option<i32>,
    pub status: String,
    pub project: String,
}

pub fn parse(input: &str) -> Vec<ParserTask> {
    let mut buffer: String = String::new();
    let mut result: Vec<ParserTask> = vec![];
    let mut last_status: String = String::new();
    let mut last_project: String = String::new();

    for i in 0..input.len() {
        let c = input.chars().nth(i).unwrap();

        if c == '\n' || i == input.len() - 1 {
            if i == input.len() - 1 {
                buffer.push(c);
            }

            if let Some(project) = parse_project(&buffer) {
                last_project = project;
            } else if let Some(status) = parse_status(&buffer) {
                last_status = status;
            } else if let Some(task) = parse_task(&buffer, &last_status, &last_project) {
                result.push(task);
            }
            buffer.clear();
        } else {
            buffer.push(c);
        }
    }

    return result;
}

fn parse_status(buffer: &str) -> Option<String> {
    let status_regex = Regex::new(r"(\w+):").unwrap();

    if status_regex.is_match(&buffer) {
        let cap = status_regex.captures(&buffer).unwrap();

        return Some(cap.get(1).unwrap().as_str().to_string());
    }

    return None;
}

fn parse_project(buffer: &str) -> Option<String> {
    let project_regex = Regex::new(r"\s\s(\w+):").unwrap();

    if project_regex.is_match(&buffer) {
        let cap = project_regex.captures(&buffer).unwrap();

        return Some(cap.get(1).unwrap().as_str().to_string());
    }

    return None;
}

fn parse_task(buffer: &str, status: &str, project: &str) -> Option<ParserTask> {
    let edit_task_regex = Regex::new(r"\s\s\s\s(\d+)\s\s(.+)").unwrap();
    let new_task_regex = Regex::new(r"\s\s\s\s(.+)").unwrap();

    if edit_task_regex.is_match(&buffer) {
        let cap = edit_task_regex.captures(&buffer).unwrap();

        return Some(ParserTask {
            id: cap.get(1).unwrap().as_str().parse::<i32>().ok(),
            project: project.to_owned(),
            status: status.to_owned(),
            title: cap.get(2).unwrap().as_str().to_string(),
        });
    }

    if new_task_regex.is_match(&buffer) {
        let cap = new_task_regex.captures(&buffer).unwrap();

        return Some(ParserTask {
            id: None,
            project: project.to_owned(),
            status: status.to_owned(),
            title: cap.get(1).unwrap().as_str().to_string(),
        });
    }

    return None;
}

#[cfg(test)]
#[test]
fn test_parse_single_status_and_project() {
    let input = "\nDOING:\n  groceries:\n    321  Buy eggs\n    322  Buy Milk\n";
    let expected = vec![
        ParserTask {
            status: "DOING".to_string(),
            project: "groceries".to_string(),
            title: "Buy eggs".to_string(),
            id: Some(321),
        },
        ParserTask {
            status: "DOING".to_string(),
            project: "groceries".to_string(),
            title: "Buy Milk".to_string(),
            id: Some(322),
        },
    ];

    assert_eq!(parse(input), expected);
}

#[test]
fn test_parse_without_newline_at_end() {
    let input = "
DOING:
  groceries:
    321  Buy eggs
    322  Buy Milk";
    let expected = vec![
        ParserTask {
            status: "DOING".to_string(),
            project: "groceries".to_string(),
            title: "Buy eggs".to_string(),
            id: Some(321),
        },
        ParserTask {
            status: "DOING".to_string(),
            project: "groceries".to_string(),
            title: "Buy Milk".to_string(),
            id: Some(322),
        },
    ];

    assert_eq!(parse(input), expected);
}

#[test]
fn test_parse_one_new_task_one_edit_task() {
    let input = "\n
DOING:
  groceries:
    321  Buy eggs
    Buy Flour\n";
    let expected = vec![
        ParserTask {
            status: "DOING".to_string(),
            project: "groceries".to_string(),
            title: "Buy eggs".to_string(),
            id: Some(321),
        },
        ParserTask {
            status: "DOING".to_string(),
            project: "groceries".to_string(),
            title: "Buy Flour".to_string(),
            id: None,
        },
    ];

    assert_eq!(parse(input), expected);
}

#[test]
fn test_parse_one_new_task_one_edit_task_without_newline() {
    let input = "
DOING:
  groceries:
    321  Buy eggs
    Buy Flour";
    let expected = vec![
        ParserTask {
            status: "DOING".to_string(),
            project: "groceries".to_string(),
            title: "Buy eggs".to_string(),
            id: Some(321),
        },
        ParserTask {
            status: "DOING".to_string(),
            project: "groceries".to_string(),
            title: "Buy Flour".to_string(),
            id: None,
        },
    ];

    assert_eq!(parse(input), expected);
}

#[test]
fn test_parse_multiple_status_and_projects() {
    let input = "
Unnecessary comment 1
DOING:
  groceries:
    321  Buy eggs
    Buy Flour
  yat:
    123  Do something
    Do something else
Unnecessary comment 2
TODO:
  groceries:
    325  Buy mozzarela
    Buy burrata
Unnecessary comment 3
  yat:
    finish parser
    Publish
Unnecessary comment 4
Unnecessary comment 5
";
    let expected = vec![
        ParserTask {
            status: "DOING".to_string(),
            project: "groceries".to_string(),
            title: "Buy eggs".to_string(),
            id: Some(321),
        },
        ParserTask {
            status: "DOING".to_string(),
            project: "groceries".to_string(),
            title: "Buy Flour".to_string(),
            id: None,
        },
        ParserTask {
            status: "DOING".to_string(),
            project: "yat".to_string(),
            title: "Do something".to_string(),
            id: Some(123),
        },
        ParserTask {
            status: "DOING".to_string(),
            project: "yat".to_string(),
            title: "Do something else".to_string(),
            id: None,
        },
        ParserTask {
            status: "TODO".to_string(),
            project: "groceries".to_string(),
            title: "Buy mozzarela".to_string(),
            id: Some(325),
        },
        ParserTask {
            status: "TODO".to_string(),
            project: "groceries".to_string(),
            title: "Buy burrata".to_string(),
            id: None,
        },
        ParserTask {
            status: "TODO".to_string(),
            project: "yat".to_string(),
            title: "finish parser".to_string(),
            id: None,
        },
        ParserTask {
            status: "TODO".to_string(),
            project: "yat".to_string(),
            title: "Publish".to_string(),
            id: None,
        },
    ];

    assert_eq!(parse(input), expected);
}
