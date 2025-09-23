use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Asynchronously read a file
    let file_content = read_file_content("example.txt").await?;

    // Print "Hello, World!" along with the file content
    println!("Hello, World!");
    println!("File Content: {}", file_content);

    Ok(())
}

async fn read_file_content(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Asynchronously open the file
    let mut file = File::open(file_path).await?;

    // Read the file content into a String
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).await?;

    Ok(file_content)
}