# The Rhoast Client

This crate allows you to easily communicate with a running node using typed safe rust
via the node's grpc and http modules.

 ```rs
 // example of a grpc propose and getting a block via hash
 use rhoast_client::grpc::propose::propose_util;
 use rhoast_client::http::block::hash_block_call;
 use rhoast_client::grpc::deploy::get_blocks_by_height_util_stream;
 use rhoast_client::proto::casper::BlocksQueryByHeight;

  let propose = propose_util("endpoint".to_string(), true).await.unwrap();
  let block_info = hash_block_call(&"endpoint".to_string(), &"hash".to_string()).await.unwrap()

 //for grpc stream actions like visualize_dag_util_stream, show_main_chain_util_stream, show_blocks_util_stream,
 // get_blocks_by_height_util_stream  pass in a function that takes in the returned value of the stream
 // as well as how many stream events should be listned to, passing in None as the number of optional stream event
 // would make the grpc listen forever

 let block_query=BlocksQueryByHeight{
     start_block_number: 1,
     end_block_number: 40
 }
 fn write_stream_to_file(input: &BlockInfoResponse){
 //write input to file
 }
 get_blocks_by_height_util_stream("endpoint".to_string(), block_query, write_stream_to_file, Some(40)).await.unwrap();

 ```

Link to doc [here](https://docs.rs/rhoast_client/0.1.2/rhoast_client/index.html)


### Test
- To run test exec `URL=http://server_url cargo test` if no URL env var is provided, all test would be skipped