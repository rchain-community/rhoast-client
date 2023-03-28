// Rholang code to bond a validator
pub fn pos_bond_rho(amount: u64) -> String {
    let rho_code = format!(
        r#"
    new retCh, PoSCh, rl(\`rho:registry:lookup\`), stdout(\`rho:io:stdout\`) in {{
        stdout!("About to lookup pos contract...") |
        rl!(\`rho:rchain:pos\`, *PoSCh) |
        for(@(_, PoS) <- PoSCh) {{
          stdout!("About to bond...") |
          @PoS!("bond", {}, *retCh) |
          for ( ret <- retCh) {{
            stdout!("PoS return!") |
            match *ret {{
              {{(true, message)}} => stdout!(("BOND_SUCCESS", "Successfully bonded!", message))
              {{(false, message)}} => stdout!(("BOND_ERROR", message))
            }}
          }}
        }}
      }}"#,
        amount
    );
    rho_code
}

//Rholang code to check balance
pub fn check_balance(addr: String) -> String {
    let rho_code = format!(
        r#"new return, rl(`rho:registry:lookup`), RevVaultCh, vaultCh in {{
        rl!(`rho:rchain:revVault`, *RevVaultCh) |
        for (@(_, RevVault) <- RevVaultCh) {{
          @RevVault!("findOrCreate", "{}", *vaultCh) |
          for (@maybeVault <- vaultCh) {{
            match maybeVault {{
              (true, vault) => @vault!("balance", *return)
              (false, err)  => return!(err)
            }}
          }}
        }}
      }}"#,
        addr
    );
    rho_code
}

//
// Rholang code to transfer REVs
//
// NOTE: Leading whitespaces are removed to fix strange bug in Trezor<->Metamask communication.
//
pub fn transfer_funds_rho(addr_from: String, addr_to: String, amount: u64) -> String {
    let rho_code = format!(
        r#"
new rl(\`rho:registry:lookup\`), RevVaultCh in {{
rl!(\`rho:rchain:revVault\`, *RevVaultCh) |
for (@(_, RevVault) <- RevVaultCh) {{
new vaultCh, vaultTo, revVaultkeyCh,
deployerId(\`rho:rchain:deployerId\`),
deployId(\`rho:rchain:deployId\`)
in {{
match ("{addr_from}", "{addr_to}", {amount}) {{
(revAddrFrom, revAddrTo, amount) => {{
@RevVault!("findOrCreate", revAddrFrom, *vaultCh) |
@RevVault!("findOrCreate", revAddrTo, *vaultTo) |
@RevVault!("deployerAuthKey", *deployerId, *revVaultkeyCh) |
for (@vault <- vaultCh; key <- revVaultkeyCh; _ <- vaultTo) {{
match vault {{
(true, vault) => {{
new resultCh in {{
@vault!("transfer", revAddrTo, amount, *key, *resultCh) |
for (@result <- resultCh) {{
match result {{
(true , _  ) => deployId!((true, "Transfer successful (not yet finalized)."))
(false, err) => deployId!((false, err))
}}
}}
}}
}}
err => {{
deployId!((false, "REV vault cannot be found or created."))
}}
}}
}}
}}
}}
}}
}}
}}"#
    );
    rho_code
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
