mod communication;
mod double_ratchet;
mod x3dh;

use communication::client::Client;
use communication::server::Server;
use x25519_dalek::PublicKey;

use crate::communication::{key_collection::ServerKeyCollection, message::{Header, Ciphertext, Message}};



fn main() {
    // Init
    let mut server: Server = Server::new();
    // User creation
    let mut alice: Client = Client::new("Alice".to_string());
    
    server.add_user(alice.get_client_name(), alice.get_server_keys());
    
    let mut bob: Client = Client::new("Bob".to_string());
    
    server.add_user(bob.get_client_name(), bob.get_server_keys());
    
    // Alice want to send a message to Bob
    // Alice use X3DH to start the communication and use Double Ratchet to create the initial message
    let (ek_pub, opk_used, header, ciphertext): (Option<PublicKey>, Option<PublicKey>, Header, Ciphertext);

    if let Some(bob_username) = server.get_users(alice.get_client_name()).get(0) { // Gather all the users on the server and select the first one (in our case Bob)
        let bob_keys: &ServerKeyCollection = match server.get_user_keys(bob_username) {
            Ok(keys) => keys,
            Err(error) => panic!("{}", error)
        };
        
        ((ek_pub, opk_used), (header, ciphertext)) = match alice.send_message(bob_username, "Message A1".as_bytes(), bob_keys) {
            Ok((None, (header_result, ciphertext_result))) => ((None, None), (header_result, ciphertext_result)),
            Ok((Some((ek_pub_result, opk_used_result)), (header_result, ciphertext_result))) => ((Some(ek_pub_result), opk_used_result), (header_result, ciphertext_result)),
            Err(error) => panic!("{}", error),
        }
    } else {
        panic!("No user in the server");
    }

    // Alice send the information to the server (possibility that Bob is offline)
    if let Err(error) = server.add_message_to(&"Bob".to_string(), Message::new(alice.get_client_name(), header, ciphertext, ek_pub, opk_used)) {
        panic!("{}", error);
    }
    
    // Bob want to read the message sent by Alice
    // Ask the server for new messages
    let new_messages: Vec<Message> =  match server.get_user_messages(&bob.get_client_name()) {
        Ok(messages) => messages,
        Err(error) => panic!("{}", error),
    };

    //observe_double_ratchet(&new_messages);

    // Ask Alice X3DH public keys
    let alice_keys: &ServerKeyCollection = match server.get_user_keys(&"Alice".to_string()) {
        Ok(keys) => keys,
        Err(error) => panic!("{}", error),
    };

    // Read the message(s) sent by Alice
    match bob.read_messages(&"Alice".to_string(), Some(alice_keys.get_ik()), new_messages.clone()) {
        Ok(res) => {
            for plaintext in res {
                println!("Bob messages:");
                println!("Sent by Alice: {}", String::from_utf8_lossy(&plaintext));
            }
        },
        Err(error) => panic!("{}", error),
    }

    // (After initialization) Simulation of a conversation (Base on the signal example: https://signal.org/docs/specifications/doubleratchet/#double-ratchet AND https://signal.org/docs/specifications/doubleratchet/#out-of-order-messages)
    let mut out_of_order_messages: Vec<(String, Message)> = Vec::new();

    send_message(&mut server, &mut bob, "Alice".to_string(), "Message B1");
    simulate_out_of_order_message(&mut server, &mut bob, "Alice".to_string(), "Message B2", &mut out_of_order_messages);

    read_messages(&mut server, &mut alice, &bob.get_client_name());

    send_message(&mut server, &mut alice, "Bob".to_string(),"Message A2");
    send_message(&mut server, &mut alice, "Bob".to_string(),"Message A3");
    send_message(&mut server, &mut alice, "Bob".to_string(),"Message A4");

    read_messages(&mut server, &mut bob, &alice.get_client_name());

    simulate_out_of_order_message(&mut server, &mut bob, "Alice".to_string(), "Message B3", &mut out_of_order_messages);
    send_message(&mut server, &mut bob, "Alice".to_string(), "Message B4");

    read_messages(&mut server, &mut alice, &bob.get_client_name());

    send_message(&mut server, &mut alice, "Bob".to_string(), "Message A5");
    for ooom in out_of_order_messages {
        send_out_of_order_message(&mut server, &ooom.0, ooom.1);
    }
    
    read_messages(&mut server, &mut bob, &alice.get_client_name());

    read_messages(&mut server, &mut alice, &bob.get_client_name());

}

fn simulate_out_of_order_message(current_server: &mut Server, current_sender: &mut Client, receiver_name: String, message: &str, out_of_order_bundle: &mut Vec<(String, Message)>) -> () {
    let (ek_pub, opk_used, header, ciphertext) = create_message(current_server, current_sender, message);
    out_of_order_bundle.push((receiver_name, Message::new(current_sender.get_client_name(), header, ciphertext, ek_pub, opk_used)));
}

fn create_message(current_server: &mut Server, current_sender: &mut Client, message: &str) -> (Option<PublicKey>, Option<PublicKey>, Header, Ciphertext) {
    // Encrypt the message (Double ratchet and AES-GCM-SIV)
    let (ek_pub, opk_used, header, ciphertext): (Option<PublicKey>, Option<PublicKey>, Header, Ciphertext);
    if let Some(receiver) = current_server.get_users(current_sender.get_client_name()).get(0) { // Gather all the users on the server and select the first one (in our case Bob)
        let bob_keys: &ServerKeyCollection = match current_server.get_user_keys(receiver) {
            Ok(keys) => keys,
            Err(error) => panic!("{}", error)
        };
        
        ((ek_pub, opk_used), (header, ciphertext)) = match current_sender.send_message(receiver, message.as_bytes(), bob_keys) {
            Ok((None, (header_result, ciphertext_result))) => ((None, None), (header_result, ciphertext_result)),
            Ok((Some((ek_pub_result, opk_used_result)), (header_result, ciphertext_result))) => ((Some(ek_pub_result), opk_used_result), (header_result, ciphertext_result)),
            Err(error) => panic!("{}", error),
        };

        return (ek_pub, opk_used, header, ciphertext)
    } else {
        panic!("No user in the server");
    }
}

fn send_out_of_order_message(current_server: &mut Server, receiver_name: &String, message: Message) -> () {
    if let Err(error) = current_server.add_message_to(&receiver_name, message) {
        panic!("{}", error);
    }
}

fn send_message(current_server: &mut Server, current_sender: &mut Client, receiver_name: String, message: &str) -> () {
    // Encrypt the message (Double ratchet and AES-GCM-SIV)
    let (ek_pub, opk_used, header, ciphertext) = create_message(current_server, current_sender, message);
    if let Err(error) = current_server.add_message_to(&receiver_name, Message::new(current_sender.get_client_name(), header, ciphertext, ek_pub, opk_used)) {
        panic!("{}", error);
    }
}

fn read_messages(current_server: &mut Server, current_receiver: &mut Client, sender_name: &String) -> () {
    println!("===============================================");
    println!("{} messages:", current_receiver.get_client_name());
    // Ask the server for new messages
    let new_messages: Vec<Message> =  match current_server.get_user_messages(&current_receiver.get_client_name()) {
        Ok(messages) => messages,
        Err(error) => panic!("{}", error),
    };

    //observe_double_ratchet(&new_messages);

    // Read the message(s) sent by Alice
    match current_receiver.read_messages(sender_name, None, new_messages.clone()) {
        Ok(res) => {
            for plaintext in res {
                println!("- Sent by {}: {}", sender_name, String::from_utf8_lossy(&plaintext));
            }
        },
        Err(error) => panic!("{}", error),
    }
}

fn observe_double_ratchet(messages: &Vec<Message>) -> () {
    println!("*********************");
    println!("{:?}", messages);
    println!("*********************");
}