use error_chain::error_chain;

error_chain! {
    foreign_links {
        Env(std::env::VarError);
        Pool(diesel::r2d2::PoolError);
        Query(diesel::result::Error);
    }
}
