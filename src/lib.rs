mod github;
pub use github::*;
use std::process::Command;

#[derive(Debug)]
pub enum Release {
    Major,
    Minor,
    Patch
}

impl From<&GithubContext> for Option<Release> {
    fn from(github: &GithubContext) -> Self {
        let major_label = std::env::var("MAJOR_LABEL").expect("Couldn't get MAJOR_LABEL");
        let minor_label = std::env::var("MINOR_LABEL").expect("Couldn't get MINOR_LABEL");
        let patch_label = std::env::var("PATCH_LABEL").expect("Couldn't get PATCH_LABEL");

        let labels = github.labels();
        if let Some(_) = labels.iter().find(|label| label.name == patch_label) {
            Some(Release::Patch)
        } else if let Some(_) = labels.iter().find(|label| label.name == minor_label) {
            Some(Release::Minor)
        } else if let Some(_) = labels.iter().find(|label| label.name == major_label) {
            Some(Release::Major)
        } else {
            None
        }
    }
}

pub fn check_publish() {
    let command = "cargo";
    let args = &["publish", "--dry-run"];
    let output = Command::new(command)
        .args(args.iter())
        .output()
        .expect("Couldn't get Output.");
    if output.status.success() {
        let output = String::from_utf8(output.stdout).expect("Couldn't parse utf8.");
        println!("{}", output);
        if let Some(_) = output.find("warning") {
            std::process::exit(-1);
        }
    } else {
        panic!("Command execution failed.");
    }
}