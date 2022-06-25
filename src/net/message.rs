use serde::{Serialize, Deserialize};

#[repr(u8)]
#[derive(Serialize, Deserialize)]
pub enum MessageType {
    /*
    A message type representing that a Bulletin Board initially sends to a Blockchain when it wants to post it.
    The message body should contain the name of the blockchain together with its public key in a pair.
    The Blockchain is then free to either confirm or deny the request for subscription.
    */
    RequestForSubscription,
    /*
    A message type representing confirmation of the request for subscription.
    Before sending this message, the Blockchain should have added the name:key pair to its active subscriptions pool.
    */
    ConfirmRFS,
    /*
    A message type representing the denial of the request for subscription.
     */
    DenyRFS,
    /*
    A message type representing a request on a client's behalf for the Bulletin Boards that the Blockchain is currently subscribed to.
    */
    RequestActiveSubscriptions,
    /*
    A message type representing the response to RequestBoardPool.
    The body should contain a dictionary in the form of "name":"public key" for each Bulletin Board 
    that the Blockchain is currently subscribed to.
    */
    ReturnActiveSubscriptions,
    /*
    A message type representing a blockchain post from the Bulletin Board onto the Blockchain.
    */
    PostBlockchain,
    /*
    A message type representing a request on a client's behalf for the entire available Blockchain.
    This message should be the first that a client sends when connecting to a Blockchain. If the Blockchain does not receive it 
    within a reasonable timeframe of establishing a connection with the client, the connection should be terminated.
    */
    RequestBlockchain,
    /*
    A message type representing the response to RequestBlockchain. 
    The contents of the Blockchain are sent to a client when they issue a RequestBlockchain.
     */
    ReturnBlockchain,
    
    /*
    A message type representing a request for a resource from the Bulletin Board.
    This message does include a key.
     */
    RequestResource
}

#[derive(Serialize, Deserialize)]
pub struct MessageHeader {
    pub message_type: MessageType,
    pub message_size: u32
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub header: MessageHeader,
    pub body: Vec<u8>
}
