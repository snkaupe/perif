use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.1", author = "Sebastian N. Kaupe <sebastian.kaupe@t-online.de")]
pub struct Options {

}

pub fn get_options() -> Options {
    Options::parse()
}