use std::str::FromStr;

use bytes::BufMut;
use futures::TryStreamExt;
use uuid::Uuid;
use warp::{
    multipart::{FormData, Part},
    Rejection, Reply,
};

use crate::model;
use crate::{environment::Environment, model::file::FileInsertable};

pub async fn health() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply())
}

pub async fn upload(form: FormData, env: Environment) -> Result<impl Reply, Rejection> {
    let parts: Vec<Part> = form.try_collect().await.map_err(|e| {
        eprintln!("form error: {}", e);
        warp::reject::reject()
    })?;

    let mut i = 0;

    for p in parts {
        println!("SALUT {}", i);
        i += 1;
        if p.name() == "file" {
            let content_type = p.content_type();
            let file_name = &String::from(p.filename().unwrap());

            match content_type {
                Some(file_type) => match file_type {
                    v => {}
                },
                None => {
                    eprintln!("file type could not be determined");
                    return Err(warp::reject::reject());
                }
            }

            let value = p
                .stream()
                .try_fold(Vec::new(), |mut vec, data| {
                    vec.put(data);
                    async move { Ok(vec) }
                })
                .await
                .map_err(|e| {
                    eprintln!("reading file error: {}", e);
                    warp::reject::reject()
                })?;

            let new_file = FileInsertable {
                name: file_name,
                uploader: Uuid::from_str("94f48dd8-a851-4af0-9ed3-aa18fea9c074").unwrap(),
            };

            println!("About to insert\n{:?}", &new_file);

            let inserted = model::file::insert_file(env.db_pool(), &new_file)
                .await
                .unwrap();

            let uuid = inserted.id;

            let generated_base_file_name = uuid.to_string();

            let generated_file_name = format!("./files/{}", &generated_base_file_name);

            tokio::fs::write(&generated_file_name, value)
                .await
                .map_err(|e| {
                    eprint!("error writing file: {}", e);
                    warp::reject::reject()
                })?;
            println!("created file: {}", generated_file_name);
        }
    }
    Ok("success")
}

pub async fn download(
    file: warp::fs::File,
    env: Environment,
) -> Result<impl warp::Reply, Rejection> {
    let file_path = file.path();
    let file_name = file_path.file_name().unwrap().to_str().unwrap();

    let uuid = Uuid::from_str(file_name).unwrap();

    println!("file name looked up : {}", file_name);

    model::file::bump_download_counter(env.db_pool(), &uuid)
        .await
        .unwrap();

    Ok(file)
}
