use crate::script::executor::executor;

pub fn runner(src: &[u8]) -> Option<String> {
    // read src as string
    let src = String::from_utf8_lossy(src);

    let parsed = src.split(';');

    // iterate over parsed src
    for line in parsed {
        let result = executor(line);
        if result.is_some() {
            return result;
        }
    }

    return None;
}
