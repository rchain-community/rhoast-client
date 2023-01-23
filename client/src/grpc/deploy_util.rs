use crate::models::model::{DeployData, DeployDataPayload, DeployDataReturn};
use crate::proto::{casper::Par, deploy::DataWithBlockInfo};
use bitcoin_hashes::{sha256, Hash};
use secp256k1::{ecdsa, Message, PublicKey, Secp256k1, SecretKey, Signing, Verification};
use utils::error::Error;
use utils::pub_from_private::{get_pub_key, get_seckey_from_string};
use utils::{get_blake2_hash::get_blake2_hash, SIG_ALGORITHM};

pub fn get_first_block(block_info: &Vec<DataWithBlockInfo>) -> &DataWithBlockInfo {
    &block_info[0]
}

pub fn get_value_from_block(block_info: &Vec<DataWithBlockInfo>) -> Option<&Par> {
    for i in 0..block_info.len() {
        let block = &block_info[i];
        if block.post_block_data.len() > 0 {
            for j in 0..block.post_block_data.len() {
                let data = &block.post_block_data[j];
                return Some(data);
            }
        }
    }
    None
}

//pass in secret key as a byte slice
pub fn sign_secp_256k1<C: Signing + Verification>(
    secp: &Secp256k1<C>,
    msg: &[u8],
    seckey: &[u8],
) -> Result<ecdsa::SerializedSignature, Error> {
    let msg = sha256::Hash::hash(msg);
    match Message::from_slice(&msg) {
        Ok(msg) => match SecretKey::from_slice(&seckey) {
            Ok(seckey) => {
                let sig = secp.sign_ecdsa(&msg, &seckey);
                let pubkey = PublicKey::from_secret_key(secp, &seckey);
                match secp.verify_ecdsa(&msg, &sig, &pubkey) {
                    Ok(_) => Ok(sig.serialize_der()),
                    Err(_) => Err(Error::DeployUtil("Failed to verify signature")),
                }
            }
            Err(_) => Err(Error::DeployUtil("error writing secret to slice")),
        },
        Err(_) => Err(Error::DeployUtil("error writing message to slice")),
    }
}

pub fn transfer_rev_term(from: String, to: String, amount: u64) -> String {
    let term=format!("
    new
    deployId(`rho:rchain:deployId`),
    rl(`rho:registry:lookup`),
    RevVaultCh,
    stdout(`rho:io:stdout`)
    in {{

    rl!(`rho:rchain:revVault`, *RevVaultCh) |
    for (@(_, RevVault) <- RevVaultCh) {{

    match (
      \"{}\",
      \"{}\",
      \"{}\"
    ) {{
      (from, to, amount) => {{

        new vaultCh, revVaultkeyCh, deployerId(`rho:rchain:deployerId`) in {{
          @RevVault!(\"findOrCreate\", from, *vaultCh) |
          @RevVault!(\"deployerAuthKey\", *deployerId, *revVaultkeyCh) |
          for (@(true, vault) <- vaultCh; key <- revVaultkeyCh) {{

            stdout!((\"Beginning transfer of \", amount, \"REV from\", from, \"to\", to)) |

            new resultCh in {{
              @vault!(\"transfer\", to, amount, *key, *resultCh) |
              for (@result <- resultCh) {{
                stdout!((\"Finished transfer of \", amount, \"REV to\", to, \"result was:\", result))
              }}
            }}
          }}

      }}
    }}
  }}
  }}", from , to, amount);
    term
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts((p as *const T) as *const u8, ::std::mem::size_of::<T>())
}

pub fn get_deploy_data(payload: &DeployDataPayload) -> Result<DeployDataReturn, Error> {
    //sign sec key
    let sec_key_hash = get_seckey_from_string(&payload.private_key)?;
    //get public key
    let pub_key = get_pub_key(&sec_key_hash);

    let deploy_data = DeployData {
        timestamp: payload.timestamp,
        term: payload.term.to_string(),
        shard_id: payload.shard_id.to_string(),
        phlo_price: payload.phlo_price,
        phlo_limit: payload.phlo_limit,
        valid_after_block_number: payload.valid_after_block_number,
    };
    let to_sign = unsafe { any_as_u8_slice(&deploy_data) };
    let hash = get_blake2_hash(&to_sign, Some(32))?;
    let secp = Secp256k1::new();
    let signature = sign_secp_256k1(&secp, &hash, &sec_key_hash[..])?;
    let sign_hex = hex::encode(&signature[..]);

    Ok(DeployDataReturn {
        data: deploy_data,
        deployer: pub_key.to_string(),
        signture: sign_hex,
        sig_algorithm: SIG_ALGORITHM.to_string(),
    })
}
