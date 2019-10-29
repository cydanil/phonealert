use std::convert::TryInto;
use std::env;
use std::error;
use std::process::exit;
use http::StatusCode;
extern crate url;
use url::form_urlencoded::Serializer;
extern crate reqwest;

const URL: &str = "https://smsapi.free-mobile.fr/sendmsg?";

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        println!("Expected a message");
        exit(400);  // The same exit code as StatusCode::BAD_REQUEST
    }

    let password: String = match env::var("FREE_PASSWD") {
        Ok(val) => val,
        Err(_e) => String::new(),
    };

    let username: String = match env::var("FREE_USR") {
       Ok(val) => val,
       Err(_e) => String::new(),
    };


    let mut msg: String = args.join(" ");
    msg.truncate(100);
    let encoded: String = Serializer::new(String::new())
        .append_pair("user", &username)
        .append_pair("pass", &password)
        .append_pair("msg", &msg)
        .finish();

    let url: String = URL.to_owned() + &encoded;

    let response: reqwest::Response = reqwest::get(&url).await?;
    let status: StatusCode = response.status();
    match status {
        StatusCode::BAD_REQUEST => println!("{}", "Missing argument"),
        StatusCode::PAYMENT_REQUIRED => println!("{}", "Too many messages"),
        StatusCode::FORBIDDEN => println!("{}", "Incorrect login"),
        StatusCode::INTERNAL_SERVER_ERROR => println!("{}", "Server error"),
        _ => (),  // assume it's 200ok, as described in the FreeMobile API
    }
    exit(status.as_u16().try_into().unwrap())
}
