use crate::script::executor::Executor;

pub fn runner(src: &[u8]) -> Option<String> {
    // read src as string
    let src = String::from_utf8_lossy(src);

    let parsed = src.split(';');
    
    // 1つのExecutorインスタンスを使用して変数の状態を保持
    let mut executor = Executor::new();

    // iterate over parsed src
    for line in parsed {
        let result = executor.execute(line);
        if result.is_some() {
            return result;
        }
    }

    return None;
}
