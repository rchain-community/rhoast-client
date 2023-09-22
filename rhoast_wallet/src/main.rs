use rhoast_wallet::error::Error;
use serde_json::{json, Value};
use std::collections::BTreeMap;

pub mod rho;
#[tokio::main]
async fn main() {
    // let node = rhoast_wallet::Node::new(
    //     &"https://observer.services.mainnet.rchain.coop".to_string(),
    //     &"".to_string(),
    //     &"".to_string(),
    //     &"".to_string(),
    //     &"".to_string(),
    // );
    // let balance = node
    //     .check_balance(&"1111Zfr2UB1YXuD6zccsPhqthiik4jfQV8W8aoncbvVGtfU8QJQei".to_string())
    //     .await
    //     .unwrap();
    // println!("{}", balance)
    let k="{\"expr\":{\"ExprList\":{\"data\": [{\"expr\":{\"ExprInt\":{\"data\": 30}}}, {\"expr\":{\"ExprString\":{\"data\": \"hello\"}}}]}}}";
    let j="{\"expr\":{\"ExprMap\":{\"data\":  {\"item\" : {\"expr\":{\"ExprString\":{\"data\": \"hello\"}}}, \"vile\": {\"expr\":{\"ExprString\":{\"data\": \"hello\"}}}} }}}";
    let l="{\"expr\":{\"ExprUnforg\":{\"data\":{\"UnforgPrivate\":{\"data\": \"hello\"}}}}}";
    println!("{:?}", map_rnode_respose(j));
}

#[derive(Debug)]
struct Unforgable {
    pub unforg_private: Value,
    pub unforg_deploy: Value,
    pub unforg_deployer: Value,
}

#[derive(Debug)]
enum MapResult {
    OneValue(Value),
    VecValue(Vec<MapResult>),
    MapValue(BTreeMap<String, MapResult>),
    Unforge(Unforgable),
}
fn map_rnode_respose(response: &str) -> Result<MapResult, Error> {
    let value: std::result::Result<serde_json::Value, serde_json::Error> =
        serde_json::from_str(&response);
    let mut map: BTreeMap<String, MapResult> = BTreeMap::new();
    let mut unforge_struct = Unforgable {
        unforg_deploy: Value::Null,
        unforg_private: Value::Null,
        unforg_deployer: Value::Null,
    };

    match value {
        Ok(json) => {
            use MapResult::*;

            if json.get("expr").is_some() {
                let expr = json.get("expr").unwrap();
                //check for expr string
                if expr.get("ExprString").is_some()
                    && expr.get("ExprString").unwrap().get("data").is_some()
                {
                    Ok(OneValue(
                        expr.get("ExprString")
                            .unwrap()
                            .get("data")
                            .unwrap()
                            .to_owned(),
                    ))
                }
                // check for expr uri
                else if expr.get("ExprUri").is_some()
                    && expr.get("ExprUri").unwrap().get("data").is_some()
                {
                    Ok(OneValue(
                        expr.get("ExprUri").unwrap().get("data").unwrap().to_owned(),
                    ))
                }
                //check for expr int
                else if expr.get("ExprInt").is_some()
                    && expr.get("ExprInt").unwrap().get("data").is_some()
                {
                    Ok(OneValue(
                        expr.get("ExprInt").unwrap().get("data").unwrap().to_owned(),
                    ))
                }
                //check for expr boolean
                else if expr.get("ExprBool").is_some()
                    && expr.get("ExprBool").unwrap().get("data").is_some()
                {
                    Ok(OneValue(
                        expr.get("ExprBool")
                            .unwrap()
                            .get("data")
                            .unwrap()
                            .to_owned(),
                    ))
                }
                //check for list
                else if expr.get("ExprList").is_some()
                    && expr.get("ExprList").unwrap().get("data").is_some()
                {
                    let list = expr
                        .get("ExprList")
                        .unwrap()
                        .get("data")
                        .unwrap()
                        .as_array()
                        .to_owned()
                        .unwrap();
                    let list_map = list
                        .into_iter()
                        .map(|item| map_rnode_respose(&item.to_string()).unwrap())
                        .collect::<Vec<MapResult>>();

                    Ok(VecValue(list_map))
                }
                //check for tuple
                else if expr.get("ExprTuple").is_some()
                    && expr.get("ExprTuple").unwrap().get("data").is_some()
                {
                    let list = expr
                        .get("ExprTuple")
                        .unwrap()
                        .get("data")
                        .unwrap()
                        .as_array()
                        .to_owned()
                        .unwrap();
                    let list_map = list
                        .into_iter()
                        .map(|item| map_rnode_respose(&item.to_string()).unwrap())
                        .collect::<Vec<MapResult>>();

                    Ok(VecValue(list_map))
                }
                //check for set
                else if expr.get("ExprSet").is_some()
                    && expr.get("ExprSet").unwrap().get("data").is_some()
                {
                    let list = expr
                        .get("ExprSet")
                        .unwrap()
                        .get("data")
                        .unwrap()
                        .as_array()
                        .to_owned()
                        .unwrap();
                    let list_map = list
                        .into_iter()
                        .map(|item| map_rnode_respose(&item.to_string()).unwrap())
                        .collect::<Vec<MapResult>>();

                    Ok(VecValue(list_map))
                }
                //check for map
                else if expr.get("ExprMap").is_some()
                    && expr.get("ExprMap").unwrap().get("data").is_some()
                {
                    let object = expr
                        .get("ExprMap")
                        .unwrap()
                        .get("data")
                        .unwrap()
                        .as_object()
                        .to_owned()
                        .unwrap();

                    for key in object.keys().into_iter() {
                        map.insert(
                            String::from(key),
                            map_rnode_respose(&object.get(key).unwrap().to_string()).unwrap(),
                        );
                    }
                    Ok(MapValue(map))
                }
                //check for unforgable
                else if expr.get("ExprUnforg").is_some()
                    && expr.get("ExprUnforg").unwrap().get("data").is_some()
                {
                    let object = expr
                        .get("ExprUnforg")
                        .unwrap()
                        .get("data")
                        .unwrap()
                        .as_object()
                        .to_owned()
                        .unwrap();
                    for key in object.keys().into_iter() {
                        if key == "UnforgPrivate" {
                            unforge_struct.unforg_private =
                                expr["ExprUnforg"]["data"][key]["data"].to_owned();
                        } else if key == "UnforgDeploy" {
                            unforge_struct.unforg_deploy =
                                expr["ExprUnforg"]["data"][key]["data"].to_owned();
                        } else if key == "UnforgDeployer" {
                            unforge_struct.unforg_deployer =
                                expr["ExprUnforg"]["data"][key]["data"].to_owned();
                        }
                    }
                    Ok(Unforge(unforge_struct))
                } else {
                    Err(Error::CheckBlance(""))
                }
            } else {
                Err(Error::CheckBlance("Error getting expr"))
            }
        }
        Err(_) => Err(Error::CheckBlance("Error parsing json")),
    }
}
