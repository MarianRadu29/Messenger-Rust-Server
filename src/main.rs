mod utils;
use utils::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Set up the broadcast channel
    let (tx, _rx) = broadcast::channel(100);
    let tx = Arc::new(Mutex::new(tx));

    let connected_users = Arc::new(Mutex::new(Vec::new()));
    let db = Arc::new(Mutex::new(get_msg_db().await?));//serverul nu ruleaza deoarece nu s-a preluat BD


    let listener = TcpListener::bind("127.0.0.1:8081").await?;
    println!("Serverul WebSocket ruleaza pe 127.0.0.1:8081");

    
    loop {
        //astept asincron sa se conecteze clientii la server
        let (stream, _) = listener.accept().await?;

        //fac o copie a "trasmitatorului" care o sa ma ajute sa comunic intre thread-uri
        let tx = Arc::clone(&tx);

        let connected_users = Arc::clone(&connected_users);
        let db = Arc::clone(&db);

        // pentru fiecare client fac cate un thread
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, tx, connected_users,db).await {
                println!("Eroare in gestionarea conexiunii: {}", e);
            }
        });
    }
}

