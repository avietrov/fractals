use std::io::Cursor;

use hyper::{Body, Method, Request, Response, StatusCode, Uri};
use image::RgbImage;
use std::collections::HashMap;
use std::convert::Infallible;

use crate::math::complex::Complex;
use crate::math::polynomial::Polynomial;
use crate::rendering::render_image;
use crate::{newton_method_field, Field};

struct ServerError {
    code: StatusCode,
}

pub async fn api(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => match read_query(req.uri()) {
            Ok(query) => {
                let field = parse_field_params(&query);
                match parse_pol_param(&query) {
                    Ok(pol) => match handle_image_request(pol, field).await {
                        Ok(payload) => *response.body_mut() = payload.into(),
                        Err(err) => *response.status_mut() = err.code,
                    },
                    Err(err) => *response.status_mut() = err.code,
                }
            }

            Err(err) => {
                *response.status_mut() = err.code;
            }
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    }

    Ok(response)
}

fn read_query(uri: &Uri) -> Result<HashMap<String, String>, ServerError> {
    uri.query()
        .map(|v| {
            Ok(url::form_urlencoded::parse(v.as_bytes())
                .into_owned()
                .collect())
        })
        .unwrap_or(Err(ServerError {
            code: StatusCode::BAD_REQUEST,
        }))
}

fn parse_field_params(params: &HashMap<String, String>) -> Field {
    let tw: f64 = parse_param_f64(params, "tw", 10.);
    let tx: f64 = parse_param_f64(params, "tx", -5.);
    let ty: f64 = parse_param_f64(params, "ty", -5.);

    Field {
        source: Complex { re: tx, im: ty },
        size: tw,
        grid: 512,
    }
}

fn parse_param_f64(params: &HashMap<String, String>, name: &str, default: f64) -> f64 {
    if let Some(param) = params.get(name) {
        if let Ok(val) = param.parse() {
            return val;
        }
    }

    default
}

fn parse_pol_param(params: &HashMap<String, String>) -> Result<Polynomial, ServerError> {
    if let Some(param) = params.get("pol") {
        let string_coefs = param.split(",");
        let mut coefs: Vec<i32> = vec![];
        for c in string_coefs {
            if let Ok(ic) = c.parse() {
                coefs.push(ic)
            } else {
                return Err(ServerError {
                    code: StatusCode::BAD_REQUEST,
                });
            }
        }
        Ok(Polynomial::new(coefs))
    } else {
        Err(ServerError {
            code: StatusCode::BAD_REQUEST,
        })
    }
}

async fn handle_image_request(pol: Polynomial, field: Field) -> Result<Vec<u8>, ServerError> {
    let max_iter = 100;
    let solutins = newton_method_field(pol, field, max_iter).await;
    let image = render_image(&solutins, &field, max_iter);
    serialize_image(image)
}

fn serialize_image(image: RgbImage) -> Result<Vec<u8>, ServerError> {
    let mut data = Cursor::new(Vec::new());
    let res = image.write_to(&mut data, image::ImageOutputFormat::Jpeg(255));
    if res.is_ok() {
        return Ok(data.get_ref().clone());
    } else {
        return Err(ServerError {
            code: StatusCode::INTERNAL_SERVER_ERROR,
        });
    }
}
