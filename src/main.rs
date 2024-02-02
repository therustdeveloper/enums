fn main() {
    println!("Rust Enums!");

    let api_response_success = ApiResponse::Success("Connected to database!".to_string());
    println!("api_response_success content: {:?}", api_response_success);

    let api_response_error = ApiResponse::Error("Could not connect to database.".to_string());
    println!("api_response_error content: {:?}", api_response_error);

    process_response(api_response_success);

    let ok_response = HTTPStatus::Ok;
    println!("ok_response content: {:?}", ok_response);

    let notfound_response = HTTPStatus::NotFound;
    println!("notfound_response content: {:?}", notfound_response);

    let bad_request_response = HTTPStatus::BadRequest;
    println!("bad_request_response content: {:?}", bad_request_response);

    let internal_server_error_response = HTTPStatus::InternalServerError;
    println!("internal_server_error_response content: {:?}", internal_server_error_response);
}

#[derive(Debug)]
enum HTTPStatus {
    Ok,
    NotFound,
    BadRequest,
    InternalServerError,
}

#[derive(Debug)]
pub enum ApiResponse {
    Success(String),
    Error(String),
}

pub enum Request {
    Get(String),
    Post { path: String, body: String },
}

fn process_response(response: ApiResponse) {
    match response {
        ApiResponse::Success(msg) => {
            println!("Success: {}", msg)
        }
        ApiResponse::Error(msg) => {
            println!("Error: {}", msg)
        }
    }
}