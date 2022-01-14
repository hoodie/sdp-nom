use sdp_nom::Session;
use serde_json::Value;

fn read_from_args() -> Option<Session<'static>> {
    if let Some(arg) = std::env::args().nth(1) {
        if let Ok(content) = std::fs::read_to_string(arg) {
            Some(Session::read_str(&content).into_owned())
        } else {
            None
        }
    } else {
        println!("no input! please pass a file path as first parameter");
        None
    }
}

fn filter_undefineds(value: &mut Value) -> &mut Value {
    match value {
        Value::Array(array) => array.iter_mut().for_each(|v| {
            filter_undefineds(v);
        }),
        Value::Object(object) => {
            object.iter_mut().for_each(|(_, v)| {
                filter_undefineds(v);
            });
            object.retain(|_, value| match value {
                Value::Null => false,
                Value::Array(a) => !a.is_empty(),
                _ => true,
            });
        }
        _ => (),
    }
    value
}

fn main() {
    let session = read_from_args().unwrap();

    let value = serde_json::to_value(&session)
        .map(|mut v| {
            filter_undefineds(&mut v);
            v
        })
        .and_then(|value| serde_json::to_string_pretty(&value))
        .unwrap();
    println!("{}", value);
}
