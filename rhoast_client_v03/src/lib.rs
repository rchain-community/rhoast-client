//!This crate allows you to easily communicate with a running version 0.13.x node using typed safe rust
//!via the node's grpc and http modules.
//!
//! ```ignore
//! // example of a grpc propose and getting a block via hash
//! // example of a grpc propose and getting a block via hash
//! use rhoast_client_v03::grpc::Grpc;
//! use rhoast_client_v03::proto::casper::BlocksQueryByHeight;
//!
//!  let grpc=Grpc::new("endpoint");
//!  let propose = grpc.propose(true).await.unwrap();
//!
//! //for grpc stream actions like visualize_dag_util_stream, show_main_chain_util_stream, show_blocks_util_stream,
//! // get_blocks_by_height_util_stream  pass in a function that takes in the returned value of the stream
//! // as well as how many stream events should be listned to, passing in None as the number of optional stream event
//! // would make the grpc listen forever
//!
//! let block_query=BlocksQueryByHeight{
//!     start_block_number: 1,
//!     end_block_number: 40
//! }
//! fn write_stream_to_file(input: &BlockInfoResponse){
//! //write input to file
//! }
//! grpc.get_blocks_by_height_util_stream(block_query, write_stream_to_file, Some(40)).await.unwrap();
//!
//! ```
//!
//!
//!
pub mod error;
pub mod grpc;
pub mod proto;
