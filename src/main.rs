#![feature(file_create_new)]
#![feature(fs_try_exists)]
#![allow(dead_code)]

use chrono::prelude::*;
use std::env;
use expanduser::expanduser;
use std::path::PathBuf;
use regex::Regex;
use serde::Deserialize;
use std::fs;
use std::process::{Command, Stdio};
use toml;

#[derive(Debug, Deserialize)]
struct Config {
    local_dir: String,
    remote_dir: String,
    editor: String,
    index_heading: String
}

fn create_or_update_homepage(string: String) -> Result<(), std::io::Error> {
    let config = read_config().expect("cannot read config");
    let filepath = format!("{}{}", config.local_dir, "index.gmi");
    match fs::try_exists(&filepath) {
      Ok(true) => {
          let homepage = fs::read_to_string(&filepath)?;
          match homepage.find(&string) {
              Some(_s) => {
                // if it's already listed, do nothing and move on...
              },
              None => {
                  let mut lines: Vec<&str> = homepage.lines().collect();
                  if lines.contains(&config.index_heading.as_str()) {
                      let i = lines.iter().position(|x| x == &config.index_heading).unwrap(); // ERROR here if the file exists but is empty
                      let first_entry = format!("\n{}", string);
                      lines[i + 1] = &first_entry;
                      lines.remove(i + 6);
                      fs::write(filepath, lines.join("\n"))?;
                  } else {
                      // if the file exists but is empty (or can't find the header) just write out the new entry
                      // Note: this will overwrite the homepage if it doesn't contain the header string exactly as
                      // per the config
                      let contents = format!("{}\n\n{}", config.index_heading, string);
                      fs::write(&filepath, contents)?;
                  }
              }
          }
      },
      Ok(false) => {
          // there's no homepage, create a new one
          let contents = format!("{}\n\n{}", config.index_heading, string);
          fs::write(&filepath, contents)?;
      },
      Err(err) => eprintln!("{}", err)
    };
    Ok(())
}

fn help() {
    println!("\x1B[1;37;41mСоюз (Soyuz) - v{}\x1B[0m", option_env!("CARGO_PKG_VERSION").unwrap());
    println!("A command line program for publishing Gemini posts\n\x1B[0m");
    println!("    \x1B[1;31mhelp | settings | write | publish | sync [ up | down [ --overwrite | --delete ]]\n\x1B[0m");
    println!("    \x1B[1;31mhelp\x1B[0m");
    println!("    - shows this help screen\n");
    println!("    \x1B[1;31msettings\x1B[0m");
    println!("    - create or edit the settings file\n");
    println!("    \x1B[1;31mwrite\x1B[0m");
    println!("    - create or edit today's gempost using the editor specified in the settings file\n");
    println!("    \x1B[1;31mpublish\x1B[0m");
    println!("    - update the homepage and year archive lists of posts, and publish to server");
    println!("      Syncs new or changed files from the server before updating indexes and syncing up\n");
    println!("      If you wish to make manual homepage changes you should use 'sync down' first\n");
    println!("    \x1B[1;31msync\x1B[0m");
    println!("    - synchronise files between local machine and server");
    println!("      This is a wrapper around rsync, the default being 'rsync -rtOq'");
    println!("      Without arguments sync defaults to 'sync up' which is the equivalent of 'publish'\n");
    println!("    \x1B[1;31msync up\x1B[0m");
    println!("    - synchronise new or changed files from local machine to server\n");
    println!("    \x1B[1;31msync down\x1B[0m");
    println!("    - synchronise new or changed files from server to local machine\n");
    println!("    \x1B[1;31msync (down | up) --overwrite\x1B[0m");
    println!("    - sync, and overwrite all files at the destination regardless of last edit date");
    println!("      Without this flag, sync will ignore files at the destination edited more recently than the source\n");
    println!("    \x1B[1;31msync (down | up) --delete\x1B[0m");
    println!("    - sync, and delete any files at the destination that do not exist at the source\n");
}

fn open_file(filepath: std::path::PathBuf) {
    let editor = match read_config() {
        Ok(config) => config.editor,
        Err(_err) => String::from("nano")
    };

    let mut open = Command::new(&editor)
            .arg(&filepath)
            .spawn()
            .expect(format!("Failed to run {} {:?} command", editor, filepath.into_os_string()).as_str());
    let _result = open.wait().expect("problem waiting for editor");
}

fn publish() -> Result<(), Box<dyn std::error::Error>> {
    println!("Publishing, please wait...");
    let config = read_config()?;
    // sync down from server (only files that are newer on the server than local)
    let mut sync = Command::new("rsync")
            .args(["-rtOq", "--update", &config.remote_dir, &config.local_dir])
            .spawn()
            .expect("syncing from server failed");
    let _result = sync.wait().expect("problem waiting for rsync");
  // check for this year's directory
    let dt = Local::now();
    let year = &dt.format("%Y").to_string();
    let dirpath = format!("{}{}", &config.local_dir, year);
    let indexfilepath = format!("{}/{}", &dirpath, "index.gmi");
    if let Ok(entries) = fs::read_dir(&dirpath) {
        let mut most_recent: (i32, String) = (0,String::new());
        for entry in entries {
            if let Ok(entry) = entry {
                // if filename format yyyy-mm-dd.gmi
                let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").expect("Error creating regex");
                let file_string = entry.file_name().into_string().expect("Cannot parse filename");
                let ent = match &file_string.strip_suffix(".gmi") {
                    Some(stripped) => stripped,
                    None => ""
                };
                if re.is_match(&ent) {
                    let allnums = ent.replace("-", "");
                    let name_as_int: i32 = allnums.parse().unwrap();
                    if most_recent.0 < name_as_int {
                        most_recent = (name_as_int, file_string);
                    }
                }
            };
        };
        if most_recent.0 > 0 {
            let filename = most_recent.1;
            let sliced = &filename.strip_suffix(".gmi").expect("file is not a gemini file!");
            let post = fs::read_to_string(format!("{}/{}", dirpath, &filename))?;
            let post_lines: Vec<&str> = post.lines().collect();
            let title = post_lines[0].strip_prefix("# ");
            let entry_string = format!("=> {} {} ({})", filename, sliced, title.unwrap());
            let home_entry_string = format!("=> /{}/{} {} ({})", &year, filename, sliced, title.unwrap());
            if let Ok(index) = fs::read_to_string(&indexfilepath) {
                // is the file listed in the archive?
                match index.find(&filename) {
                  Some(_i) => {
                      // the file is already listed in the year index
                      // check that it's listed on the homepage too
                      create_or_update_homepage(home_entry_string)?;
                  },
                  None => {
                      let mut lines: Vec<&str> = index.lines().collect();
                      if lines.len() > 2 {
                          // there's an archive page, update it
                          let first_value = format!("{}\n{}", &entry_string, lines[2]);
                          lines[2] = &first_value;
                          let new_string = lines.join("\n");
                          fs::write(indexfilepath, &new_string)?;
                      } else {
                        // file is blank, overwrite it
                        let contents = format!("# {}\n\n{}", &year, &entry_string);
                        fs::write(indexfilepath, contents)?;
                      }
                      // update or create the homepage
                      create_or_update_homepage(home_entry_string)?;
                  }
                }
            } else {
                // year archive file doesn't exist, just create it
                let contents = format!("# {}\n\n{}", &year, &entry_string);
                fs::write(indexfilepath, contents)?;
                create_or_update_homepage(home_entry_string)?;
            }

        } else {
          // dir has no files for some reason in
          // in this case there's nothing to do
          // except sync up any changes from other dirs
        }
    } else {
      // create new directory then try again
      // this will create an empty archive if used
      // before there are any new posts for the year
      fs::create_dir_all(dirpath)?;
      publish()?;
    }

  // rsync to server (no del)
    let mut sync_up = Command::new("rsync")
            .args(["-rtOq", &config.local_dir, &config.remote_dir])
            .spawn()
            .expect("syncing to server failed");
    let _sync_up_result = sync_up.wait().expect("problem waiting for rsync up");
    println!("🎉 published!");
    Ok(())
}


fn read_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path: std::path::PathBuf = expanduser("~/.config/soyuz/config.toml")?;
    let vals = fs::read_to_string(config_path)?;

    let mut config: Config = toml::from_str(&vals)?;
    let local: PathBuf = expanduser(config.local_dir).unwrap();
    let remote: PathBuf = expanduser(config.remote_dir).unwrap();
    let l_trimmed = local.to_str().unwrap().trim_end_matches("/");
    let r_trimmed = remote.to_str().unwrap().trim_end_matches("/");
    config.local_dir = format!("{}/", l_trimmed);
    config.remote_dir = format!("{}/", r_trimmed);
    Ok(config)
}


fn settings() -> Result<(), Box<dyn std::error::Error>> {
    let config_dir: std::path::PathBuf = expanduser("~/.config/soyuz")?;
    let config_path: std::path::PathBuf = expanduser("~/.config/soyuz/config.toml")?;
    fs::create_dir_all(config_dir)?;
    let _file = match fs::File::create_new(&config_path) {
        Ok(_file) => {
          fs::write(&config_path, "local_dir = \nremote_dir = \neditor = \"nano\"\nindex_heading =\"## Latest notes\"\n")
        },
        Err(_err) => Ok(()) // do nothing, this means the file exists already
    };
    Ok(open_file(config_path))
}

fn sync(args: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let config = read_config()?;
    let rsync_args = match args.len() {
        2 => ["-rtOq", "--update", &config.local_dir, &config.remote_dir],
        3 | 4 => {
            match args[2].as_str() {
                "down" => {
                    if args.len() == 3 {
                        ["-rtOq", "--update", &config.remote_dir, &config.local_dir]
                    } else {
                        match args[3].as_str() {
                            "--overwrite" => {
                                ["-rtO", "--quiet", &config.remote_dir, &config.local_dir]
                            },
                            "--delete" => {
                                ["-rtOq", "--delete", &config.remote_dir, &config.local_dir]
                            },
                            _ => ["", "", "", ""],
                        }
                    }
                },
                "up" => {
                    if args.len() == 3 {
                        ["-rtOq", "--update", &config.local_dir, &config.remote_dir]
                    } else {
                        match args[3].as_str() {
                            "--overwrite" => {
                                ["-rtO", "--quiet", &config.local_dir, &config.remote_dir]
                            },
                            "--delete" => {
                                ["-rtOq", "--delete", &config.local_dir, &config.remote_dir]
                            },
                            _ => ["", "", "", ""],
                        }
                    }
                },
                _ => ["","","",""],
            }
        },
        _ => ["","","",""],
    };
    if rsync_args[0] == "" {
        Ok(help())
    } else {
        let mut sync = Command::new("rsync")
                .args(rsync_args)
                .spawn()
                .expect("syncing failed");
        let _sync_res = sync.wait().expect("problem waiting for rsync");
      Ok(println!("Sync complete."))
    }
}

fn write() -> Result<(), Box<dyn std::error::Error>> {
    println!("Checking server for latest post...");
    let config = read_config()?;
    let dt = Local::now();
    let year = &dt.format("%Y").to_string();
    let local_dir: std::path::PathBuf = expanduser(&config.local_dir)?;
    let remote_dir: std::path::PathBuf = expanduser(&config.remote_dir)?;
    fs::create_dir_all(format!("{}{}", &local_dir.display(), &year))?;
    let filepath = format!("{}{}/{}.gmi", local_dir.display(), year, dt.format("%Y-%m-%d"));
    let spath = format!("{}", remote_dir.display());
    let remote_vec = spath.split(':').collect::<Vec<_>>();
    let server_name = remote_vec[0];
    let server_path = remote_vec[1];
    let remote_filepath = format!("{}{}/{}.gmi", server_path, year, dt.format("%Y-%m-%d"));
    let cmd = format!("[[ -f {} ]] && echo 'true' || echo 'false';", &remote_filepath);
    let check = Command::new("ssh")
            .args(["-q", &server_name, &cmd])
            .stdout(Stdio::piped())
            .spawn()
            .expect("reading from server failed");
    let output = check
              .wait_with_output()
              .expect("something fucked up");
    let out = std::str::from_utf8(&output.stdout);
    let out2 = out.clone()?.trim();
    match out?.trim() {
          "true" => {
            // user has already published today
            println!("\x1B[1;31mYou have already published today!\x1B[0m");
            println!("To edit your post, run 'sync down' first.");
            Ok(())
          },
          "false" => {
            // open a new file
            Ok(open_file(filepath.into()))
          },
          _ => {
            println!("Something went wrong checking your gemini server");
            println!("Error: {:?}", out2);
            Ok(())
          }
        }
}

fn match_single_arg(args: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
  match args[1].as_str() {
        "write" => write(),
        "settings" => settings(),
        "publish" => publish(),
        "sync" => sync(args),
        _ => Ok(help())
  }
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args: Vec<String> = env::args().collect();
    match &args.len() {
        1 => Ok(help()),
        2 => match_single_arg(&args),
        3 | 4  => sync(&args),
        _ => Ok(help())
    }
}
