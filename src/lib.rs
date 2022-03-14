pub struct CasAuth {
    pub cas_url: String,
    pub service_url: String,
    pub auth_route: String
}

impl CasAuth {
    pub fn new(cas_url: &str, service_url: &str, auth_route: &str) -> CasAuth {
        CasAuth {
            cas_url: String::from(cas_url),
            service_url: String::from(service_url),
            auth_route: String::from(auth_route)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::CasAuth;

    #[test]
    fn it_creates_a_CasAuth_object() {
        let cas_url = "https://cas.testu.edu/cas";
        let service_url = "https://testservice.edu";
        let auth_route = "/next";

        let test_cas_object = CasAuth::new(cas_url, service_url, auth_route);

        assert_eq!(test_cas_object.cas_url, cas_url);
        assert_eq!(test_cas_object.service_url, service_url);
        assert_eq!(test_cas_object.auth_route, auth_route);
    }
}
