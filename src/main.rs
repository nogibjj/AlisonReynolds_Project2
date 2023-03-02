// Build a actix microservice

use actix_web::{get, App, HttpResponse, HttpServer, Responder};

//create a function that returns a hello world
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Transformer")
}

//create a function that log transforms and plots
#[get("/log")]
async fn log_data() -> impl Responder {
    //print helpful message
    println!("Converting the data to log");
    let (temp, pressure) = microservice::read_pressure_data();

    let log_press = microservice::log_transform(&pressure);

    // run the plot function and show plot on the actix server
    microservice::plot(temp, log_press);
    HttpResponse::Ok()
        .content_type("scatter.png")
        .body("Log data")
}

//create a function that sqrt transforms and plots
#[get("/sqrt")]
async fn sqrt_data() -> impl Responder {
    //print helpful message
    println!("Converting the data to sqrt");
    let (temp, pressure) = microservice::read_pressure_data();

    let sqrt_press = microservice::sqrt_transform(&pressure);

    // run the plot function and show plot on the actix server
    microservice::plot(temp, sqrt_press);

    // HttpResponse::Ok().body("Sqrt data")
    HttpResponse::Ok()
        .content_type("scatter.png")
        .body("Sqrt data")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //add a print message to the console that the service is running
    println!("Running the service");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(log_data)
            .service(sqrt_data)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
