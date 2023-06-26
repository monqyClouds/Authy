use authy::data::AppDatabase;
use dotenv::dotenv;
// use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "httpd")]
struct Opt {
    #[structopt(default_value = "sqlite:data.db")]
    connection_string: String,
}

fn main() {
    dotenv().ok();
    let opt = Opt::from_args();
    let rt = tokio::runtime::Runtime::new().expect("failed to spawn tokio runtime");

    // let handle = rt.handle().clone();
    let database = rt.block_on(async move { AppDatabase::new(&opt.connection_string).await });

    let config = authy::RocketConfig { database };

    let _ = rt.block_on(async move {
        authy::rocket(config)
            .launch()
            .await
            .expect("failed to launch rocket server")
    });
}
