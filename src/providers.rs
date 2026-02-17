pub trait GitProvider {
    fn get_name(&self) -> String;
}

pub struct GitHub;

impl GitProvider for GitHub {
    fn get_name(&self) -> String {
        "GitHub".to_string()
    }
}