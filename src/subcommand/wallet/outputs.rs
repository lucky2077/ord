use super::*;

#[derive(Serialize, Deserialize)]
pub struct Output {
  pub output: OutPoint,
  pub amount: u64,
}

pub(crate) fn run(options: Options) -> Result {
  // let index = Index::open(&options)?;
  // index.update()?;
  let client = options.bitcoin_rpc_client_for_wallet_command(false)?;

  let mut utxos = BTreeMap::new();
  utxos.extend(
      client
      .list_unspent(None, None, None, None, None)?
      .into_iter()
      .map(|utxo| {
        let outpoint = OutPoint::new(utxo.txid, utxo.vout);
        let amount = utxo.amount;

        (outpoint, amount)
      }),
  );

  let mut outputs = Vec::new();
  for (output, amount) in utxos {
    outputs.push(Output {
      output,
      amount: amount.to_sat(),
    });
  }

  print_json(outputs)?;

  Ok(())
}
