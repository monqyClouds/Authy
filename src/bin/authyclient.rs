use authy::domain::user::field::{Email, Name, Password};
use authy::service::ask::{GetUser, NewUser, UpdateUser};
use authy::web::api::{ApiKey, API_KEY_HEADER};
use authy::{ServiceError, User};
use std::error::Error;
use std::str::FromStr;
// use std::process::Command;
use structopt::StructOpt;
// use strum::EnumString;

#[derive(StructOpt, Debug)]
enum Command {
    Get {
        #[structopt(short, long, help = "email")]
        email: Email,
        #[structopt(short, long, help = "password")]
        password: Password,
    },
    New {
        #[structopt(short, long, help = "name")]
        name: Name,
        #[structopt(short, long, help = "email")]
        email: Email,
        #[structopt(short, long, help = "password")]
        password: Password,
    },
    Update {
        #[structopt(short, long, help = "name")]
        name: Name,
        #[structopt(short, long, help = "email")]
        email: Email,
        #[structopt(short, long, help = "password")]
        password: Password,
    },
    GetApiKey {},
    RevokeApiKey {},
}

#[derive(StructOpt, Debug)]
#[structopt(name = "authyclient", about = "Authy API Client")]
struct Opt {
    #[structopt(subcommand)]
    command: Command,

    #[structopt(default_value = "http://127.0.0.1:8000", env = "AUTHY_ADDR")]
    addr: String,

    #[structopt(long)]
    api_key: String,
}

fn get_user(addr: &str, ask_scv: GetUser, api_key: ApiKey) -> Result<User, Box<dyn Error>> {
    let client = reqwest::blocking::Client::builder().build()?;
    let addr = format!("{}/api/user/login", addr);
    let mut request = client.post(addr);

    request = request.header(API_KEY_HEADER, api_key.to_base64());

    let user: User = request.json(&ask_scv).send()?.json()?;

    if ask_scv.password.is_none() {
        Err(Box::new(ServiceError::PermissionError(
            "Invalid password".to_string(),
        )))
    } else if &ask_scv.password.unwrap().into_inner() != user.clone().password.into_inner().as_str()
    {
        Err(Box::new(ServiceError::PermissionError(
            "Invalid password".to_string(),
        )))
    } else {
        Ok(user)
    }

    // if ask_scv.password.is_some() && user.password.into_inner() == ask_scv.password.unwrap().into_inner()() {
    //     Ok(request.json(&ask_scv).send()?.json()?)
    // } else {
    //     Err(ServiceError::PermissionError("Invalid password".to_string()))
    // }
}

fn new_user(addr: &str, ask_scv: NewUser, api_key: ApiKey) -> Result<User, Box<dyn Error>> {
    let client = reqwest::blocking::Client::builder().build()?;
    let addr = format!("{}/api/user", addr);
    let mut request = client.post(addr);

    request = request.header(API_KEY_HEADER, api_key.to_base64());

    Ok(request.json(&ask_scv).send()?.json()?)
}

fn update_user(addr: &str, ask_scv: UpdateUser, api_key: ApiKey) -> Result<User, Box<dyn Error>> {
    let client = reqwest::blocking::Client::builder().build()?;
    let addr = format!("{}/api/user/login", addr);
    let mut request = client.post(addr.clone());

    request = request.header(API_KEY_HEADER, api_key.clone().to_base64());

    let get_user_req = GetUser {
        email: ask_scv.email,
        password: None,
    };

    let user: User = request.json(&get_user_req).send()?.json()?;

    let client = reqwest::blocking::Client::builder().build()?;
    let addr = format!("{}/api/user", addr);
    let mut request = client.patch(addr);

    request = request.header(API_KEY_HEADER, api_key.to_base64());

    let update_user_req = UpdateUser {
        email: user.email,
        name: match ask_scv.name {
            Some(value) => Some(value),
            None => Some(user.name),
        },
        password: match ask_scv.password {
            Some(value) => Some(value),
            None => Some(user.password),
        },
    };

    Ok(request.json(&update_user_req).send()?.json()?)
}

fn get_api_key(addr: &str) -> Result<ApiKey, Box<dyn Error>> {
    let client = reqwest::blocking::Client::builder().build()?;
    let addr = format!("{}/api/user", addr);
    let request = client.get(addr);

    Ok(request.send()?.json()?)
}

fn revoke_api_key(addr: &str, api_key: ApiKey) -> Result<bool, Box<dyn Error>> {
    let client = reqwest::blocking::Client::builder().build()?;
    let addr = format!("{}/api/user", addr);
    let mut request = client.get(addr);

    request = request.header(API_KEY_HEADER, api_key.to_base64());

    Ok(request.send()?.json()?)
}

fn run(opt: Opt) -> Result<(), Box<dyn Error>> {
    match opt.command {
        Command::Get { email, password } => {
            let req = GetUser {
                email,
                password: Some(password),
            };

            let user = get_user(&opt.addr.as_str(), req, ApiKey::from_str(&opt.api_key)?)?;
            println!("{:#?}", user);

            Ok(())
        }
        Command::New {
            name,
            email,
            password,
        } => {
            let req = NewUser {
                email,
                password,
                name,
            };

            let user = new_user(&opt.addr, req, ApiKey::from_str(&opt.api_key)?)?;

            println!("{user:#?}");
            Ok(())
        }
        Command::Update {
            name,
            email,
            password,
        } => {
            let req = UpdateUser {
                email,
                name: Some(name),
                password: Some(password),
            };

            let user = update_user(&opt.addr, req, ApiKey::from_str(&opt.api_key)?)?;

            println!("{user:#?}");
            Ok(())
        }
        Command::GetApiKey {} => {
            let api_key = get_api_key(&opt.addr)?;

            println!("{api_key:#?}");
            Ok(())
        }
        Command::RevokeApiKey {} => {
            let status = revoke_api_key(&opt.addr, ApiKey::from_str(&opt.api_key)?)?;

            if status {
                println!("logout successful");
            } else {
                println!("logout not successful");
            }

            Ok(())
        }
    }
}

fn main() {
    let opt = Opt::from_args();

    if let Err(e) = run(opt) {
        eprintln!("An error occured: {e}");
    }
}
