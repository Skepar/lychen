use std::env::Args;
use std::fmt;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Setting {
    name: &'static str,
    short: &'static str,
    default: Vec<u32>,
    help: &'static str,
    set_by_user: bool,
}

impl fmt::Display for Setting {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\t{}\tdefault={:?}\t{}",
               self.short,
               self.name,
               self.default,
               self.help)
    }
}

pub fn get_settings() -> Vec<Setting> {
    vec![
        Setting {
            name: "--square-size",
            short: "-s",
            default: vec![10],
            help: "Defines the size of an individual cell, in pixels.",
            set_by_user: false
        },
        Setting {
            name: "--window-size",
            short: "-w",
            default: vec![500, 300],
            help: "Defines the size of the window in pixels.",
            set_by_user: false
        },
        Setting {
            name: "--model-size",
            short: "-m",
            default: vec![50, 30],
            help: "Defines the size of the grid model.",
            set_by_user: false
        }
    ]
}

pub fn print_help(error_msg: String) {
    println!("{} Help :", error_msg);
    println!("Short\tLong\t\tDefault\t\tDesc");
    get_settings().iter().for_each(|s| println!("{}", s));
}

//returns a Result enum, containing a map, with the keys being the setting's name
// and the value the number it is set at.
pub fn from_args(mut args: Args) -> Result<HashMap<String, u32>, String> {
    let mut settings = get_settings();
    let mut res: HashMap<String, u32> = HashMap::new();
    // first argument is ignored, since it's the binary's location
    args.next();
    while let Some(arg) = args.next() {
        // for every argument, we see if there's a setting with the same name
        match settings.iter_mut().find(|s| arg == s.name.to_string()) {
            Some(s) => {
                s.set_by_user = true;
                // we check the next arguments and insert them after conversion to u32
                for i in 0..s.default.len() {
                    if let Some(a) = args.next() {
                        if let Ok(n) = a.parse::<u32>() {
                            res.insert(s.name[2..].to_string()+&i.to_string(), n);
                        } else {
                            return Err("Argument must be an integer.".to_string())
                        }
                    } else {
                        return Err("Argument must be an integer.".to_string())
                    }
                }
            },
            None => return Err("Invalid argument.".to_string())
        }
    }
    // the default settings not set by the user are inserted
    settings.iter().filter(|s| !s.set_by_user)
        .for_each(|s| s.default.iter().enumerate()
            .for_each(|(i, d)| {res.insert(s.name[2..].to_string()+&i.to_string(), *d);}));
    Ok(res)
}
