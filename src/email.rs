use chrono::{NaiveDateTime, NaiveDate};
use regex::Regex;

const HEADERS_RE: &str = r#"Subject:.+\nOriginal Recipient:.+\nDate:.+\n"#;
const EMAIL_SOURCE_RE: &str = r#"[A-Z]+[a-z]+ [A-Z]+[a-z]+,\n(\s+|\t)[a-zA-Z,.?\-!@#$%^&*()"' \n]+\n\n(\s+|\t).+,\n(\s+|\t).+\n"#;

#[derive(Debug)]
pub struct Email {
    pub subject: String,
    pub paragraphs: Vec<String>,
    pub recipient: String,
    pub time: NaiveDateTime,
}

impl Email {
    pub fn new() -> Self {
        Self {
            subject: "N/A".to_string(),
            recipient: "N/A".to_string(),
            time: NaiveDate::from_ymd(1970, 1, 1).and_hms(0, 0, 0),
            paragraphs: vec![],
        }
    }
}

pub fn get_classes(source: &String) {
    let part_re = Regex::new(&format!("{}\n{}", HEADERS_RE, EMAIL_SOURCE_RE)).unwrap();
    let header_re = Regex::new(HEADERS_RE).unwrap();
    let email_source_re = Regex::new(EMAIL_SOURCE_RE).unwrap();

    let mut emails: Vec<Email> = vec![];
    let mut headers: Vec<String> = vec![];
    let mut email_sources: Vec<String> = vec![];

    for part_match in part_re.captures_iter(source) {
        let part = part_match.get(0).unwrap().as_str().to_string();
        let mut email: Email = Email::new();

        {
            let header_source = header_re.find(&part).unwrap().as_str().to_string();
            let lines = header_source.lines().collect::<Vec<&str>>()
                .iter().map(|s| s.to_string()).collect::<Vec<String>>();

            email.subject = lines.get(0).unwrap().split(" ").skip(1).collect::<Vec<&str>>().join(" ");
            email.recipient = lines.get(1).unwrap().split(" ").skip(2).collect::<Vec<&str>>().join(" ");
            email.time = NaiveDateTime::parse_from_str(
                &lines.get(2).unwrap().split(" ").skip(1).collect::<Vec<&str>>().join(" "),
                "%m/%d, %Y, %H:%M"
            ).unwrap();
        }

        {
            let email_source = email_source_re.find(&part).unwrap().as_str().to_string();
            let lines = email_source.split("\n").map(|i| i.to_string()).collect::<Vec<String>>();
            let stripped_email_source = lines[1..lines.len()-3].to_vec().join(" ");

            let paragraphs = stripped_email_source.split("\n\n").map(|s| s.to_string()).collect::<Vec<String>>();

            email.paragraphs = paragraphs;
        }

        emails.push(email);
    }

    println!("{:?}", emails);
}

