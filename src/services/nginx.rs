#[derive(Debug)]
pub struct Nginx;

const NGINX_STUB: &str = include_str!("../../stubs/nginx.conf");

impl Nginx {
    pub fn install() {}
}
