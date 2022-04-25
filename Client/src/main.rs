use std::env;
use hyper::{body::HttpBody as _, Client};
use hyper::{Body, Method, Request, Uri};
use tokio::io::{self, AsyncWriteExt as _};

/*
Estructura que tendra Result
*/
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
/*
Funcion principal del programa 
*/
#[tokio::main]
async fn main() -> Result<()> {

    pretty_env_logger::init();
    //url del servidor //
    let url =  "127.0.0.1:7878";

    let url = url.parse::<hyper::Uri>().unwrap();
    //llama la funcion fetch//
    fetch_url(url).await
}

/*
funcion en la cual se crea un nuevo cliente para hacer la solicitud al servidor
*/
async fn fetch_url(url: hyper::Uri) -> Result<()> {
    let client = Client::new();
    let mut res = client.get(url).await?;
    
    while let Some(next) = res.data().await {
        let chunk = next?;
        io::stdout().write_all(&chunk).await?;
    }
    println!("\n\nDone!");
    Ok(())
}

/*
Funcion metodo get por parte del cliente
*/

async fn get_Method() -> Result<()>  {
    let req = Request::builder()
        .method(Method::GET)
        .uri("127.0.0.1:7878")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"library":"hyper"}"#))?;
    Ok(())
}

/*
Funcion del metodo POST por parte del cliente
*/
async fn post_Method() -> Result<()>  {
    let req = Request::builder()
        .method(Method::POST)
        .uri("127.0.0.1:7878")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"library":"hyper"}"#))?;
    Ok(())
}