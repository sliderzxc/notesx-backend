pub mod data {
    pub mod cloud {
        pub mod mongo_database;
    }
    pub mod entities {
        pub mod sign_up {
            pub mod sign_up_cloud_data;
            pub mod sign_up_request_data;
            pub mod sign_up_response_data;
        }
        pub mod token {
            pub mod token_config;
            pub mod token_claim;
        }
    }
    pub mod repository {
        pub mod auth_repository_impl;
    }
    pub mod security {
        pub mod hashing {

        }
        pub mod token {
            pub mod token_service_impl;
        }
    }
    pub mod validation {
        pub mod auth_validation_data_repository_impl;
    }
}

pub mod domain {
    pub mod repository {
        pub mod auth_repository;
    }
    pub mod routes {
        pub mod sign_up_route;
    }
    pub mod security {
        pub mod hashing {

        }
        pub mod token {
            pub mod token_service;
        }
    }
    pub mod usecases {
        pub mod sign_up_use_case;
    }
    pub mod validation {
        pub mod auth_validation_data_repository;
    }
}