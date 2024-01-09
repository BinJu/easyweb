use actix_web::{get, post, web, App, Error, HttpServer, HttpRequest, HttpResponse, Responder};
use actix_multipart::form::{MultipartForm, tempfile::TempFile, text::Text};
use std::path::PathBuf;

use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize)]
pub struct DirRequest {
    p: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct FileRequest {
    f: String
}

#[derive(Debug, Serialize)]
pub struct ListItem {
    title: String,
    #[serde(rename = "type")]
    tp: String,
    path: String
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    file: TempFile,
    path: Text<String>
}

fn root_dir() -> String {
    std::env::var("ROOT_DIR").unwrap_or("./".to_owned())
}

#[get("/dir")]
async fn dir(info: web::Query<DirRequest>) -> impl Responder {
    let root_dir = PathBuf::from(root_dir());
    let mut current_dir = root_dir.clone();
    if let Some(path) = &info.p {
        let p = path.replace("../", "");
        current_dir.push(p);
    };

    //if !current_dir.starts_with("./") { current_dir = "./".to_owned() + &current_dir };
    let paths = std::fs::read_dir(current_dir).unwrap();

    let mut result = Vec::new();
    for path in paths {
        let p = path.unwrap();
        let path = "./".to_owned() + p.path().as_path().strip_prefix(root_dir.clone()).unwrap().as_os_str().to_str().unwrap();
        
        result.push(ListItem{title: p.file_name().into_string().unwrap(), tp: if p.file_type().unwrap().is_dir() {"d"} else {"f"}.to_owned(), path});
    }
    serde_json::to_string(&result)
}

#[get("/file")]
async fn file(req: HttpRequest, info: web::Query<FileRequest>) -> impl Responder {
    format!("hello {}", info.f);
    let mut path = PathBuf::from(root_dir());
    path.push(info.f.replace("./", ""));

    let file = actix_files::NamedFile::open_async(path.as_path()).await.unwrap();
    file.into_response(&req)
}

#[post("/upload")]
async fn upload(MultipartForm(form): MultipartForm<UploadForm>) -> Result<impl Responder, Error> {
    let mut path = format!("./{}/{}", form.path.0, form.file.file_name.unwrap());
    println!("upload to: {}", path);
    path = path.replace("../", "");
    form.file.file.persist(path).unwrap();
    Ok(HttpResponse::Ok())

}

#[post("/new_dir")]
async fn new_dir(info: web::Query<DirRequest>) -> Result<impl Responder, Error> {
    let mut current_dir = root_dir(); //"./".to_owned();
    if let Some(path) = &info.p {
        current_dir += path;
    };
    current_dir = current_dir.replace("..", "");
    current_dir = current_dir.replace("./", "");
    if !current_dir.starts_with("./") { current_dir = "./".to_owned() + &current_dir };
    std::fs::create_dir(current_dir)?;
    Ok(HttpResponse::Ok())

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
       // let cors = actix_cors::Cors::default()
       //     .allowed_origin("http://localhost:8081")
       //     .allowed_methods(vec!["GET", "POST"]);
        App::new()
       //     .wrap(cors)
            .service(dir)
            .service(file)
            .service(upload)
            .service(new_dir)
            .service(web::redirect("/", "/index.html"))
            .service(actix_files::Files::new("/", "./front-ui/dist/"))
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}


//    #[tokio::main]
//    async fn main() {
//        // Running curl -F file=@.gitignore 'localhost:3030/' should print [("file", ".gitignore", "\n/target\n**/*.rs.bk\nCargo.lock\n.idea/\nwarp.iml\n")]
//        let index = warp::path("index.html").and(warp::fs::file("./examples/send_file_index.html"));
//        let route = warp::path("upload").and(warp::multipart::form().and_then(|form: FormData| async move {
//            let field_names: Vec<_> = form
//                .and_then(|mut field| async move {
//                    let mut bytes: Vec<u8> = Vec::new();
//                    println!("field: {:?}", field);
//
//                    // field.data() only returns a piece of the content, you should call over it until it replies None
//                    while let Some(content) = field.data().await {
//                        let content = content.unwrap();
//                        bytes.put(content);
//                    }
//                    Ok((
//                        field.name().to_string(),
//                        field.filename().unwrap().to_string(),
//                        String::from_utf8_lossy(&*bytes).to_string(),
//                    ))
//                })
//                .try_collect()
//                .await
//                .unwrap();
//
//            Ok::<_, warp::Rejection>(format!("{:?}", field_names))
//        })).or(index);
//        warp::serve(route).run(([127, 0, 0, 1], 3030)).await;
//    }

