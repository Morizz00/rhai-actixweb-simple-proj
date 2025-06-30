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

    let res=engine.eval_file::<i64>("rhai/multiply.rhai".into()).unwrap();

    format!("{res}")


}
#[get("/add/{num1}/{num2}")]
async fn add(path:Path<(i64,i64)>)->impl Responder{
    let (num1,num2)=path.into_inner();

    let mut engine=Engine::new();

    engine.register_fn("num1",move||num1);
    engine.register_fn("num2",move||num2);

    let res=engine.eval_file::<i64>("rhai/add.rhai".into()).unwrap();

    format!("{res}")


}
#[get("/subtract/{num1}/{num2}")]
async fn subtract(path:Path<(i64,i64)>)->impl Responder{
    let (num1,num2)=path.into_inner();

    let mut engine=Engine::new();

    engine.register_fn("num1",move||num1);
    engine.register_fn("num2",move||num2);

    let res=engine.eval_file::<i64>("rhai/subtract.rhai".into()).unwrap();

    format!("{res}")


}
#[get("/divide/{num1}/{num2}")]
async fn divide(path:Path<(i64,i64)>)->impl Responder{
    let (num1,num2)=path.into_inner();

    let mut engine=Engine::new();

    engine.register_fn("num1",move||num1);
    engine.register_fn("num2",move||num2);

    let res=engine.eval_file::<i64>("rhai/divide.rhai".into()).unwrap();

    format!("{res}")
}
#[get("/factorial/{num}")]
async fn factorial(path: Path<i64>) -> impl Responder {
    let num = path.into_inner();

    let mut engine = Engine::new();
    engine.register_fn("num", move || num);

    let res = engine.eval_file::<i64>("rhai/factorial.rhai".into()).unwrap();
    format!("{res}")
}

#[get("/lcm/{num1}/{num2}")]
async fn lcm(path: Path<(i64, i64)>) -> impl Responder {
    let (num1, num2) = path.into_inner();

    let mut engine = Engine::new();
    engine.register_fn("num1", move || num1);
    engine.register_fn("num2", move || num2);

    let res = engine.eval_file::<i64>("rhai/lcm.rhai".into()).unwrap();
    format!("{res}")
}

#[get("/hcf/{num1}/{num2}")]
async fn hcf(path: Path<(i64, i64)>) -> impl Responder {
    let (num1, num2) = path.into_inner();

    let mut engine = Engine::new();
    engine.register_fn("num1", move || num1);
    engine.register_fn("num2", move || num2);

    let res = engine.eval_file::<i64>("rhai/hcf.rhai".into()).unwrap();
    format!("{res}")
}

#[get("/power/{base}/{exponent}")]
async fn power(path: Path<(i64, i64)>) -> impl Responder {
    let (base, exponent) = path.into_inner();

    let mut engine = Engine::new();
    engine.register_fn("base", move || base);
    engine.register_fn("exponent", move || exponent);

    let res = engine.eval_file::<i64>("rhai/power.rhai".into()).unwrap();
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
        .service(factorial)
        .service(power)
        .service(lcm)
        .service(hcf)

      })
      .bind(("127.0.0.1",8081))
      .unwrap()
      .run()
      .await
}
