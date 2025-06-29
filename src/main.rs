use actix_web::{
    HttpServer,
    get,
    App,
    web::Path,
    Responder,
};

use rhai::Engine;

#[get("/multiply/{num1}/{num2}")]
async fn multiply(path:Path<(i64,i64)>)->impl Responder{
    let (num1,num2)=path.into_inner();

    let mut engine=Engine::new();

    engine.register_fn("num1",move||num1);
    engine.register_fn("num2",move||num2);

    let res=engine.eval_file::<i64>("src/multiply.rhai".into()).unwrap();

    format!("{res}")


}
#[get("/add/{num1}/{num2}")]
async fn add(path:Path<(i64,i64)>)->impl Responder{
    let (num1,num2)=path.into_inner();

    let mut engine=Engine::new();

    engine.register_fn("num1",move||num1);
    engine.register_fn("num2",move||num2);

    let res=engine.eval_file::<i64>("src/add.rhai".into()).unwrap();

    format!("{res}")


}
#[get("/subtract/{num1}/{num2}")]
async fn subtract(path:Path<(i64,i64)>)->impl Responder{
    let (num1,num2)=path.into_inner();

    let mut engine=Engine::new();

    engine.register_fn("num1",move||num1);
    engine.register_fn("num2",move||num2);

    let res=engine.eval_file::<i64>("src/subtract.rhai".into()).unwrap();

    format!("{res}")


}
#[get("/divide/{num1}/{num2}")]
async fn divide(path:Path<(i64,i64)>)->impl Responder{
    let (num1,num2)=path.into_inner();

    let mut engine=Engine::new();

    engine.register_fn("num1",move||num1);
    engine.register_fn("num2",move||num2);

    let res=engine.eval_file::<i64>("src/divide.rhai".into()).unwrap();

    format!("{res}")


}


#[actix_web::main]
async fn main()->std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
        .service(multiply)
        .service(add)
        .service(subtract)
        .service(divide)
      })
      .bind(("127.0.0.1",8081))
      .unwrap()
      .run()
      .await
}
