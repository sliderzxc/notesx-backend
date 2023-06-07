pub mod data {
    pub mod cloud {
        pub mod mongo_database;
    }
    pub mod entities {
        pub mod sign_up_data;
        pub mod sign_up_response;
    }
    pub mod repository {
        pub mod auth_repository_impl;
    }
}

pub mod domain {
    pub mod repository {
        pub mod auth_repository;
    }
    pub mod routes {
        pub mod sign_up_route;
    }
    pub mod usecases {

    }
}