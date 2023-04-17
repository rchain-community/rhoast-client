pub mod v0_13 {
    pub mod scalapb {
        tonic::include_proto!("scalapb");
    }

    pub mod casper {
        tonic::include_proto!("casper");
        pub mod v1 {
            tonic::include_proto!("casper.v1");
        }
    }

    pub mod routing {
        tonic::include_proto!("routing");
    }
}
