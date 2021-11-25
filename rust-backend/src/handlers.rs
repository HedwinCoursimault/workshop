use std::str::FromStr;

use bytes::BufMut;
use futures::TryStreamExt;
use uuid::Uuid;
use warp::{
    multipart::{FormData, Part},
    Rejection, Reply,
};

use crate::{
    auth,
    model::{
        self,
        user::{UserInsertable, UserLogin},
    },
};
use crate::{environment::Environment, model::file::FileInsertable};

pub async fn health() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply())
}

pub async fn upload(
    form: FormData,
    user_id: Uuid,
    env: Environment,
) -> Result<impl Reply, Rejection> {
    let parts: Vec<Part> = form.try_collect().await.map_err(|e| {
        eprintln!("form error: {}", e);
        warp::reject::reject()
    })?;

    let mut i = 0;

    let mut file_ids: Vec<String> = Vec::new();

    for p in parts {
        println!("SALUT {}\n{:?}", i, &p);
        i += 1;
        if p.name() == "file" {
            let file_name = &String::from(p.filename().unwrap());

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
                uploader: user_id,
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

            file_ids.push(generated_base_file_name);
        }
    }
    Ok(warp::reply::json(&file_ids))
}

pub async fn get_files_by_uploader(
    user_id: Uuid,
    env: Environment,
) -> Result<impl warp::Reply, Rejection> {
    match model::file::get_files_by_uploader(env.db_pool(), &user_id).await {
        Ok(files) => Ok(warp::reply::json(&files)),
        Err(e) => {
            println!("{:?}", e);
            Err(warp::reject())
        }
    }
}

pub async fn download(
    file: warp::fs::File,
    user_id: Uuid,
    env: Environment,
) -> Result<impl warp::Reply, Rejection> {
    let file_path = file.path();
    let file_name = file_path.file_name().unwrap().to_str().unwrap();

    let uuid = Uuid::from_str(file_name).unwrap();

    println!("file name looked up : {}", file_name);

    let file_db = match model::file::get_file(env.db_pool(), &uuid).await {
        Ok(f) => f,
        Err(_) => return Err(warp::reject()),
    };

    let file_db = file_db.unwrap();

    if file_db.uploader != user_id {
        return Err(warp::reject());
    }

    model::file::bump_download_counter(env.db_pool(), &uuid)
        .await
        .unwrap();

    Ok(file)
}

pub async fn login(
    env: Environment,
    login: UserLogin,
) -> Result<impl warp::Reply, warp::Rejection> {
    match model::user::login(env.db_pool(), &login.login, &login.password).await {
        Ok(user) => {
            let token = auth::create_jwt(&user.id.to_string(), &auth::Role::from_str("User"))
                .map_err(warp::reject::custom)?;
            Ok(warp::reply::json(&model::user::UserLoginResponse { token }))
        }
        Err(e) => {
            println!("{:?}", e);
            Err(warp::reject())
        }
    }
}

pub async fn insert_user(
    env: Environment,
    new: UserInsertable,
) -> Result<impl warp::Reply, warp::Rejection> {
    match model::user::insert_user(env.db_pool(), &new).await {
        Ok(u) => Ok(warp::reply::json(&u)),
        Err(e) => {
            println!("{:?}", e);
            Err(warp::reject())
        }
    }
}
