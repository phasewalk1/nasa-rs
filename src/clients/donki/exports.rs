use super::endpoints::*;
use crate::create_client_impl;
use crate::prelude::{Client, ClientHandler};

create_client_impl!(Flr, FLR_BASE_URL, crate::params::ParamsCommon);
create_client_impl!(Gst, GST_BASE_URL, crate::params::ParamsCommon);
create_client_impl!(Sep, SEP_BASE_URL, crate::params::ParamsCommon);
create_client_impl!(Mpc, MPC_BASE_URL, crate::params::ParamsCommon);
create_client_impl!(Rbe, RBE_BASE_URL, crate::params::ParamsCommon);
create_client_impl!(Hss, HSS_BASE_URL, crate::params::ParamsCommon);
create_client_impl!(Wsa, WSA_BASE_URL, crate::params::ParamsCommon);
create_client_impl!(Ips, IPS_BASE_URL, super::ips::IpsParams);

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude::Spec;

    fn test_impl<S: Spec>() -> bool
    where
        Client<S>: ClientHandler<S>,
    {
        let client = Client::<S>::default();
        match client.query(&S::Params::default()) {
            Ok(_) => (return true),
            Err(_) => (return false),
        };
    }

    macro_rules! test_spec {
        ($spec:ty, $test_id:ident) => {
            #[test]
            fn $test_id() {
                assert!(test_impl::<$spec>());
            }
        };
    }

    test_spec!(Flr, flr);
    test_spec!(Gst, gst);
    test_spec!(Sep, sep);
    test_spec!(Mpc, mpc);
    test_spec!(Rbe, rbe);
    test_spec!(Hss, hss);
    test_spec!(Wsa, wsa);
}
