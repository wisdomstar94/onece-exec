use std::{fs, path::Path};

#[derive(clap::Args)]
#[command(
  about="한번만 실행되는 명령어와 항상 실행되는 명령어를 지정하는 커맨드 입니다.
ex) onece-exec run --path=\"./oneced.txt\" --onece=\"source check.sh\" --always=\"source start.sh\"
  ", 
  long_about = None)
]
pub struct CliArgs {
  /// 한번만 실행됬는지 아닌지 체크하기 위한 파일 경로를 입력하세요
  #[arg(short='p', long)]
  path: Option<String>,
  /// 한번만 실행되기를 원하는 명령어를 입력하세요
  #[arg(short='o', long)]
  onece: Option<String>,
  /// 항상 실행되기를 원하는 명령어를 입력하세요
  #[arg(short='a', long)]
  always: Option<String>,
}

pub fn run(args: CliArgs) {
  if let None = args.path {
    panic!("path 인자가 주어지지 않았습니다!");
  }

  if let None = args.onece {
    panic!("onece 인자가 주어지지 않았습니다!");
  }

  if let (Some(path), Some(onece_command)) = (args.path, args.onece) {
    let path_obj = Path::new(&path);
    if is_exist(path_obj) == false {
      // let result = std::process::Command::new(onece_command).output();
      match run_command(onece_command.as_str()) {
        Ok(output) => {
          println!("onece 명령어 성공..! : {:?}", output);
        },
        Err(error) => {
          println!("onece 명령어 실패..! : {:?}", error);
        },
      }
      fs::write(path_obj, String::from("")).unwrap();
    } 

    if let Some(always_command) = args.always {
      match run_command(always_command.as_str()) {
        Ok(output) => {
          println!("always 명령어 성공..! : {:?}", output);
        },
        Err(error) => {
          println!("always 명령어 실패..! : {:?}", error);
        },
      }
    }
  }
}

fn is_exist(path: &Path) -> bool {
  match fs::metadata(path) {
    Ok(_) => true,
    Err(_) => false,
  }
}

fn run_command(cmd_string: &str) -> Result<std::process::Output, std::io::Error> {
  let output = if cfg!(target_os = "windows") {
    std::process::Command::new("cmd")
      .args(["/C", cmd_string])
      .output()
  } else {
    std::process::Command::new("sh")
      .arg("-c")
      .arg(cmd_string)
      .output()
  };
  output
}