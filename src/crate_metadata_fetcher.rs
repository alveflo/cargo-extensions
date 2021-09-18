use std::error::Error;
use std::collections::HashMap;
use substring::Substring;

fn get_crate_path(pkg: &str) -> String {
  if pkg.len() < 4 {
    return [pkg.len().to_string(), pkg.to_string()].join("/");
  }

  let path1 = pkg.substring(0,2);
  let path2 = pkg.substring(2,4);
  
  return [path1, path2, pkg].join("/");
}

pub async fn get_crate_metadata(pkg: &str) -> Result<(), Box<dyn Error>> {
  let github_url = "https://raw.githubusercontent.com/rust-lang/crates.io-index/master/";
  let url = [github_url.to_string(), get_crate_path(pkg)].join("/");

  let resp = reqwest::get(url)
    .await?
    .text()
    .await?;

  let splitted: Vec<&str> = resp.split("\n").collect();
  let latest = splitted[splitted.len() -2].to_string();

  // println!(latest);
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_crate_path_with_one_letter_name() {
    assert_eq!(get_crate_path("t"), "1/t");
  }

  #[test]
  fn test_get_crate_path_with_two_letter_name() {
    assert_eq!(get_crate_path("te"), "2/te");
  }

  #[test]
  fn test_get_crate_path_with_three_letter_name() {
    assert_eq!(get_crate_path("tes"), "3/tes");
  }

  #[test]
  fn test_get_crate_path_with_four_letter_name() {
    assert_eq!(get_crate_path("test"), "te/st/test");
  }

  #[test]
  fn test_get_crate_path_with_long_letter_name() {
    assert_eq!(get_crate_path("test_test"), "te/st/test_test");
  }
}