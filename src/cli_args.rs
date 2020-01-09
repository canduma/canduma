use structopt::StructOpt;

/// Juniper (GraphQl API), Diesel PostgreSQL, session authentication and JWT boilerplate server
#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "canduma")]
pub(crate) struct Opt {
    /// Port to listen to
    #[structopt(short, long, env = "PORT", default_value = "3000")]
    pub port: u16,

    /// Domain
    #[structopt(long, env = "DOMAIN", default_value = "localhost")]
    pub domain: String,

    /// Database URL
    #[structopt(long, env = "DATABASE_URL")]
    pub database_url: String,

    /// Secret Key for Auth Cookie
    #[structopt(
        long,
        env = "AUTH_SECRET_KEY",
        default_value = "01230123012301230123012301230123"
    )]
    pub auth_secret_key: String,

    /// Use secure cookie (HTTPS),
    /// this can only be set if you have https
    #[structopt(long, env = "HTTPS_COOKIE")]
    pub secure_cookie: bool,

    /// Auth duration in hours,
    /// this is used for cookie and JWT
    #[structopt(long, env = "AUTH_DURATION_IN_HOUR", default_value = "24")]
    pub auth_duration_in_hour: u16,
}
