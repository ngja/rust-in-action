fn main() {
    // example 2.2
    let a = 10; // 데이터 타입 컴파일러 추론, 변수는 기본 불변 (읽기 전용)
    let b: i32 = 20; // 직접 명시
    let c = 30i32; // 숫자 뒤에 붙은 i32는 타입 annotation
    let d = 30_i32; // 숫자에 가독성을 위해 _ 사용 가능
    let e = add(add(a, b), add(c, d));

    println!("( a + b ) + ( c + d ) = {}", e);

    // example 2.3
    let twenty = 20;
    let twenty_one: i32 = 21;
    let twentky_two = 22i32;

    let addition = twenty + twenty_one + twentky_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twentky_two, addition);

    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2)); // 숫자가 method를 가진다.

    let forty_twos = [ // 배열은 모두 같은 타입
        42.0,
        42f32,
        42.0_f32,
    ];

    println!("{:02}", forty_twos[0]);

    let three = 0b11; // 0b 이진수
    let thirty = 0o36; // 0o 팔진수
    let three_hundred = 0x12C; // 0x 십육진수

    println!("base 10: {} {} {}", three, thirty, three_hundred);
    println!("base  2: {:b} {:b} {:b}", three, thirty, three_hundred);
    println!("base  8: {:o} {:o} {:o}", three, thirty, three_hundred);
    println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);
}

fn add(i: i32, j: i32) -> i32 { // 함수 정의 시 타입 반드시 필요
    i + j // 결과 반환에 return 필요 없음
}
