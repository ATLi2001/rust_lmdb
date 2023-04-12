use time::precise_time_ns;
use tikv_client::TransactionClient;

#[tokio::main]
async fn main() {
  let txn_client = TransactionClient::new(vec!["128.105.145.194:2379"], None).await.unwrap();

  for i in 0..100 {
    let key = Vec::from(i.to_string());
    let value = Vec::from(i.to_string());

    println!("{}", precise_time_ns());
    let mut txn = txn_client.begin_optimistic().await.unwrap();
    txn.put(key.to_owned(), value.to_owned()).await.unwrap();
    txn.commit().await.unwrap();
  }
  println!("{}", precise_time_ns());
}