fn main() { // Rust도 main function이 entry point

    // " 뒤의 \ 를 붙여서 바로 다음 줄바꿈을 무시
    // 각 줄의 공백도 모두 문자열 penguin_data에 포함된다.
    let penguin_data = "\
    common name,length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";

    let records = penguin_data.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        let fields: Vec<_> = record
            .split(',')
            .map(|field| field.trim())
            .collect();
        if cfg!(debug_assertions) { // cfg! 컴파일 시 속성이나 조건 검사. 해당 조건은 cargo run --release 옵션을 주고 하는 경우 조건 결과가 false 가 된다.
            eprintln!("debug: {:?} -> {:?}", record, fields); // eprintln! 표준 에러 출력, rust interpolation 방식
        }

        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, length); // println! 표준 출력, rust interpolation 방식
        }
    }
}
