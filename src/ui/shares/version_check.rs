use serde::Deserialize;
use std::error::Error;

pub fn version_check() -> i8 {
    let version: Vec<i8> = env!("CARGO_PKG_VERSION")
        .split('.')
        .map(|n| n.parse::<i8>().unwrap())
        .collect();
    let version_github = match fetch() {
        Ok(tag) => Some(tag),
        Err(e) => {
            println!("Fail to check update {e}");
            None
        }
    };
    match match_version(version, version_github) {
        Some(e) => {
            if e {
                1
            } else {
                -1
            }
        }
        None => 0,
    }
}

fn match_version(cur_ver: Vec<i8>, git_ver: Option<Vec<i8>>) -> Option<bool> {
    if let Some(git_ver) = git_ver {
        for (c, g) in cur_ver.iter().zip(git_ver.iter()) {
            if c < g {
                return Some(false);
            } else if c > g {
                return Some(true);
            }
        }
        None
    } else {
        None
    }
}

fn fetch() -> Result<Vec<i8>, Box<dyn Error>> {
    let re = ureq::get("https://api.github.com/repos/musdx/azul-box/releases/latest")
        .call()?
        .body_mut()
        .read_json::<Github>()?;
    let tag = re.tag_name.replace("v", "");
    let tag: Vec<i8> = tag.split(".").map(|n| n.parse::<i8>().unwrap()).collect();

    Ok(tag)
}

#[derive(Debug, Deserialize)]
struct Github {
    tag_name: String,
}
