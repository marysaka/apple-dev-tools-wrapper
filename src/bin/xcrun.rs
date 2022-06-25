use apple_dev_tools_wrapper::AppleTarget;

#[derive(Debug)]
struct Argument {
    sdk_target: Option<AppleTarget>,
    sub_argument: SubArgument
}

#[derive(Debug)]
#[allow(unused)]
enum SubArgument {
    ShowSdkPath,
    ShowSdkVersion,
    ShowSdkBuildVersion,
    ShowSdkPlatformPath,
    ShowSdkPlatformVersion,

    Find { tool_name: String},
    Run { arguments: Vec<String> }
}

impl TryFrom<&str> for SubArgument {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "--show-sdk-path" => Ok(SubArgument::ShowSdkPath),
            "--show-sdk-version" => Ok(SubArgument::ShowSdkVersion),
            "--show-sdk-build-version" => Ok(SubArgument::ShowSdkBuildVersion),
            "--show-sdk-platform-path" => Ok(SubArgument::ShowSdkPlatformPath),
            "--show-sdk-platform-version" => Ok(SubArgument::ShowSdkPlatformVersion),
            _ => Err(())
        }
    }
}


fn parse_args(raw: impl IntoIterator<Item=impl Into<std::ffi::OsString>>) -> Result<Argument, String> {
    let raw = clap_lex::RawArgs::new(raw);
    let mut cursor = raw.cursor();

    // Skip binary name
    raw.next(&mut cursor);

    let mut run_arguments = Vec::new();

    let mut sdk_target = None;
    let mut sub_argument = None;
    let mut is_run_command = false;

    while let Some(arg) = raw.next(&mut cursor) {
        match arg.to_value() {
            Ok("--sdk") | Ok("-sdk") => {
                sdk_target = raw.next(&mut cursor).and_then(|x| x.to_value().ok().and_then(|x| AppleTarget::try_from(x).ok()));

                if sdk_target.is_none() {
                    return Err("Missing sdk name after --sdk or invalid target".into())
                }
            },
            Ok("--find") => {
                let tool_name = raw.next(&mut cursor).and_then(|x| x.to_value().ok().map(|x| x.to_string()));

                if tool_name.is_none() {
                    return Err("Missing tool name after --find".into())
                }

                if sub_argument.is_some() || is_run_command {
                    return Err("Conflicting argument --find found".into())
                }

                sub_argument = Some(SubArgument::Find { tool_name: tool_name.unwrap() })
            },
            Ok("--run") => {
                if sub_argument.is_some() || is_run_command {
                    return Err("Conflicting argument --run found".into())
                }

                is_run_command = true;
            }
            Ok(raw_arg) => {
                if let Ok(new_sub_argument) = SubArgument::try_from(raw_arg)  {
                    if sub_argument.is_some() || is_run_command {
                        return Err(format!("Conflicting argument {} found", raw_arg))
                    }

                    sub_argument = Some(new_sub_argument);
                }

                run_arguments.push(raw_arg.to_string())
            }
            _ => {}
        }
    }

    if let Some(sub_argument) = sub_argument {
        Ok(Argument { sdk_target, sub_argument})
    } else {
        Ok(Argument { sdk_target, sub_argument: SubArgument::Run { arguments: run_arguments }})
    }
}
fn main() {
    let args = parse_args(std::env::args_os());

    match args {
        Ok(Argument { sdk_target: Some(sdk_target), sub_argument: SubArgument::ShowSdkPath }) => {
            let target_name = sdk_target.to_string().to_uppercase();
            let environment_target = format!("{}_SDKROOT", target_name);

            if let Ok(sdk_path) = std::env::var(environment_target.as_str()) {
                println!("{}", sdk_path)
            } else {
                eprintln!("environment variable {} is missing", environment_target);
                std::process::exit(1)
            }
        },
        Ok(Argument { sdk_target: None, sub_argument: SubArgument::ShowSdkPath }) => {
            if let Ok(sdk_path) = std::env::var("SDKROOT") {
                println!("{}", sdk_path)
            } else {
                eprintln!("environment variable SDKROOT is missing");
                std::process::exit(1)
            }
        },
        Ok(arg) => {
            eprintln!("Command not implemented: {:?}", arg);
            std::process::exit(1)
        }
        Err(error) => {
            eprintln!("Argument error: {}", error);
            eprintln!("Usage: xcrun [--sdk <SDK name>] --show-sdk-path");
            std::process::exit(1)
        }
    }
}