use tokio::{fs::File, io::AsyncReadExt};
use std::str;
use std::collections::HashMap;

const NEW_LINE_CHAR : char = '\n';
const EQUALS_CHAR : char = '=';

pub async fn deserialize_properties_file(file_path: &String) -> Result<HashMap<String, String>, std::io::Error> {
    let mut props = HashMap::new();
    let props_raw = read_file(file_path).await?;
    props_raw.split(NEW_LINE_CHAR).map(|x| {
        let mut parts = x.split(EQUALS_CHAR);
        let key = match parts.next() {
            Some(key) => key,
            None => return None
        };
        let value = match parts.next() {
            Some(value) => value,
            None => return None
        };
        return Some([key, value]);
    }).filter(|entry| entry.is_some()).for_each(|entry| {
        let entry_unwrap = entry.unwrap();
        props.insert(entry_unwrap[0].trim().to_string(), entry_unwrap[1].trim().to_string());
    });
    Ok(props)
}

pub async fn read_file(file_path: &String) -> Result<String, std::io::Error> {
    let mut file = File::open(file_path).await?;
    let mut contents = vec![];
    file.read_to_end(&mut contents).await?;
    let decoded = str::from_utf8(&contents).unwrap_or("");
    Ok(decoded.to_string())
}
