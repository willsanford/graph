// use crate::AdjList;

// use http::header::{HeaderValue, ACCEPT};
// use http::{Request, Response};
// use std::io::Read;

// use log::{debug, error, info, warn};

// static BASE_URL: &str = "http://overpass-api.de/api/interpreter?data="; 
// static EXAMPLE_REQUEST: &str = "[out:json];way[highway](bbox=34.0522,-118.2437,34.0231,-118.2053);(._;>;);out;";
// struct OSMDownloadParams<'a> {
//     bbox: [f64; 4],
//     filter: &'a str,
// }



// // This works well
// // [out:json];way[highway](bbox=34.0522,-118.2437,34.0231,-118.2053);(._;>;);out;
// fn format_request(params: OSMDownloadParams) -> String {

//     let query = format!("[out:json];way[highway]({});out;");
//     format!("{}{}", BASE_URL, query)
// }







// fn download() {
//     let p: OSMDownloadParams = OSMDownloadParams{
//         bbox: [34.0522,-118.2437,34.0231,-118.2053],
//         filter: "",
//     }; 

//     let bbox = "bbox=34.0522,-118.2437,34.0231,-118.2053";
//     let query = format!("[out:json];way[highway]({});out;", bbox);
//     let url = format!("http://overpass-api.de/api/interpreter?data={}", query);

//     let request = Request::builder()
//         .uri(url)
//         .header(ACCEPT, HeaderValue::from_static("application/json"))
//         .body(())
//         .unwrap();
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn download_test() {
//         download();
//     }
// }
