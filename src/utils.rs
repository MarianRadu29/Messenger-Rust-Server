use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use tokio::fs::read_to_string;
pub use std::sync::{Arc, Mutex};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
pub use tokio::net::TcpListener;
pub use tokio::sync::broadcast;
use futures::{SinkExt, StreamExt};
pub use std::error::Error;


#[derive(Debug, Clone,Serialize,Deserialize)]
struct SendClient{
    type_msg:u8,
    //1 login failed
    //2 login succed
    //3 user already loged
    //4 db send
    //5 msg between users
    //6 status online chat users
    content:String
}

#[derive(Debug, Clone,Serialize,Deserialize)]
struct User {
    username: String,
    password: String,
}

#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct MessageUser{
    sender:String,
    content:String,
}

// Lista de users autentificati
async fn get_users() -> Result<Vec<User>, std::io::Error> {
    let content = read_to_string("src/user_db.json").await?;
    let list:Vec<User> = serde_json::from_str(&content)?;
    Ok(list)
}


pub async fn get_msg_db() -> Result<Vec<MessageUser>,std::io::Error>{
    let content = read_to_string("src/msg.json").await?;
    let list:Vec<MessageUser> = serde_json::from_str(&content)?;
    Ok(list)
}


pub async fn handle_connection( stream:         tokio::net::TcpStream,
                                  tx:           Arc<Mutex<broadcast::Sender<String>>>,
                               connected_users: Arc<Mutex<Vec<String>>>,
                                  db:           Arc<Mutex<Vec<MessageUser>>>
) -> Result<(), Box<dyn Error>> 
{
    // Accept conexiunea WebSocket
    let ws_stream = accept_async(stream).await?;
    println!("O noua conexiune WebSocket");

    let (mut write, mut read) = ws_stream.split(); // Preiau descriptori pentru scriere si citire

    let mut username = String::new();
    while username.is_empty(){
        if let Some(result) = read.next().await {
            if let Ok(message) = result {
                if let Ok(text) = message.to_text() {
                    // Verificare date de autentificare
                    let parts: Vec<&str> = text.split(":").collect();
                    if parts.len() == 3 && parts[0] == "login" {
                        let username_input = parts[1].to_string();
                        let password_input = parts[2].to_string();

                        let users = get_users().await?;
                        if users.iter().any(|user| user.username == username_input && user.password == password_input) {
                            if connected_users.lock().unwrap().contains(&username_input) {
                                    let send_msg = SendClient {
                                        type_msg:3,
                                        content:"Utilizatorul este deja conectat.".to_string()
                                    };
                                    write.send(Message::text(json!(send_msg).to_string())).await?;   
                                }
                            else{
                                        // autentificare reusita
                                        username = username_input.clone();
                                        connected_users.lock().unwrap().push(username.clone()); // adaug utilizatorul in lista de utilizatori conectati
                                        
                                        let mut list = get_users().await?.iter().map(|x| x.username.clone()).filter(|x| *x!=username_input).collect::<Vec<String>>();
                                        list.insert(0, username_input);
                                        let send_msg = SendClient {
                                            type_msg:2,
                                            content:json!(list).to_string()
                                        };
                                        write.send(Message::text(json!(send_msg).to_string())).await?;
                                        
                                        let send_msg = SendClient {
                                            type_msg:4,
                                            content:json!(db.lock().unwrap().clone()).to_string()
                                        };
                                        write.send(Message::text(json!(send_msg).to_string())).await?;
                            }
                        } else {
                            // Autentificare esuata
                            let send_msg = SendClient {
                                type_msg:1,
                                content:"Autentificare esuta".to_string()
                            };
                            write.send(Message::text(json!(send_msg).to_string())).await?;
                        }
                    } else {
                        write.send(Message::text("Format mesaj invalid.")).await?;
                        return Ok(());
                    }
                }
            }
        } else {
            println!("Conexiune inchisa inainte de autentificare.");
            return Ok(());
        }
        
    }//final while username.is_empty(),insemnand ca m am autentificat

    // Pasul 2: Clientul poate trimite si primi mesaje
    //let mut rx = tx.lock().unwrap().subscribe();
    let mut rx = match tx.lock() {
        Ok(rez)=>{ rez.subscribe()},
        Err(_)=> {
            delete_connect_user(connected_users, username);
            panic!();
        }
    };

    // Task pentru a trimite mesaje catre client atunci cand sunt transmise pe canalul de broadcast
    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if write.send(Message::text(msg)).await.is_err() {
                break;
            }
        }
    });

    // Bucla pentru primirea mesajelor(din chat) de la client
    while let Some(result) = read.next().await {
        match result {
            Ok(message) => {
                if let Ok(text) = message.to_text() {
                    if text=="close"{
                        delete_connect_user(connected_users, username);
                        return Ok(());
                    }
                    else if text=="status"{
                        let send_msg = SendClient{
                            type_msg:6,
                            content:json!(*connected_users.lock().unwrap()).to_string()
                        };
                        if tx.lock().unwrap().send(json!(send_msg).to_string()).is_err() {
                            eprintln!("Eroare la transmiterea mesajului");
                        }
                    }
                    else{
                        println!("Mesaj primit de la {}: {}",username, text );
                        let send_msg = SendClient{
                            type_msg:5,
                            content:text.to_string()
                        };
    
                        db.lock().unwrap().push(serde_json::from_str(text).unwrap());
                        
                        // Transmiterea mesajului la ceilalti clienti prin canalul de broadcast
                        if tx.lock().unwrap().send(json!(send_msg).to_string()).is_err() {
                            eprintln!("Eroare la transmiterea mesajului");
                        }
                    }
                    
                }
            }
            Err(e) => {
                eprintln!("Eroare la receptionarea mesajului: {}", e);
                break;
            }
        }
    }

    delete_connect_user(connected_users,username);

    Ok(())
}

//functie care sa ma scoata din lista de useri care sunt online atunci thread-ul isi termina executia sau clientul paraseste chat-ul
fn delete_connect_user(connected_users:Arc<Mutex<Vec<String>>>,username:String){
    let mut users_list = connected_users.lock().unwrap();
    if let Some(index) = users_list.iter().position(|u| *u == username) {
        users_list.remove(index);
        println!("{} a fost deconectat.", username);
    }
}