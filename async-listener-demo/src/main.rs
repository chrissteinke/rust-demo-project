use std::os::linux::net::TcpStreamExt;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::fs::File;
use tokio::task::JoinHandle;


// async fn read_file_content(file_path: &str) -> std::io::Result<String> {
//    // Asynchronously open the file
//     let mut file = File::open(file_path).await?;
// 
//    // Read the file content into a String
//    let mut file_content = String::new();
//     file.read_to_string(&mut file_content).await?;
// 
//     Ok(file_content)
// }
// 
// async fn start_connection_handler(mut socket: TcpStream) -> std::io::Result<JoinHandle<bool>>  {
//     let handler = tokio::spawn(async move {
//        let mut buf = vec![0; 1024];
// 
//         loop {
//            match socket.read(&mut buf).await {
//                // Return value of `Ok(0)` signifies that the remote has
//                // closed
//                Ok(0) => return true ,
//                Ok(n) => {
//                    // Copy the data back to socket
//                    if let Ok(file_content) =
//                        read_file_content("/etc/hosts").await {
//                        if socket.write_all(&buf[0..n]).await.is_err() {
//                            return false;
//                        }
//                    } else {
// 
//                    }
//                 }
//                 Err(_) => {
//                     // Unexpected socket error. There isn't much we can do
//                     // here so just stop processing.
//                    return false;
//                }
//            }
//        }
//        true
//    });
//    Ok(handler)
// }


// #[tokio::main]
// async fn main() -> io::Result<()> {
//    let listener = TcpListener::bind("127.0.0.1:6142").await?;
//
//
//    loop {
//        //  Future<Output=Result<(TcpStream, SocketAddr)>>
//        let f =  listener.accept();
//        let (mut socket, _) = listener.accept().await?;
//        let handle = start_connection_handler(socket).await?;
//    }
// }


#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            loop {
                match socket.read(&mut buf).await {
                    // Return value of `Ok(0)` signifies that the remote has
                    // closed
                    Ok(0) => return,
                    Ok(n) => {
                        // Copy the data back to socket
                        if socket.write_all(&buf[..n]).await.is_err() {
                            // Unexpected socket error. There isn't much we can
                            // do here so just stop processing.
                            return;
                        }
                    }
                    Err(_) => {
                        // Unexpected socket error. There isn't much we can do
                        // here so just stop processing.
                        return;
                    }
                }
            }
        });
    }
}

