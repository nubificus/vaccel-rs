use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "vAccel agent",
    about = "A vAccel agent that handles RPC acceleration requests"
)]
pub struct AgentCli {
    #[structopt(short = "a", long = "server-address")]
    pub uri: String,
}
