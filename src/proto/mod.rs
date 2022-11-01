pub mod casper {
    tonic::include_proto!("casper"); // The string specified here must match the proto package name
    pub mod casper_v1 {
        tonic::include_proto!("casper.v1");
    }
}

pub mod deployv1{
    tonic::include_proto!("deployv1");
}
   
pub mod casper_msg {
    tonic::include_proto!("casper_msg");
}
pub mod deploy{
    tonic::include_proto!("deploy");

}

pub mod scalapb {
    tonic::include_proto!("scalapb");
}

pub mod routing {
    tonic::include_proto!("routing");
}
