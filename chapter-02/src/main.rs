use num::Complex;

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

    let t1: i32 = 10;
    let t2: u16 = 100;

    if t1 < (t2 as i32) { // 서로 다른 타입은 as 연산으로 변환해야한다. 작은 타입을 큰 타입으로 변환하는게 안전
        println!("Ten is less than one hundred.");
    }

    let t2_ = t2.try_into() // i32 타입으로 감싸 반환
        .unwrap(); // 변환 성공하면 i32로 반환, 실패하면 프로그램 중단

    if t1 < t2_ {
        println!("Ten is less than one hundred.");
    }

    let complex1 = Complex { re: 2.1, im: -1.3 };
    let complex2 = Complex::new(11.1, 22.2);
    let result = complex1 + complex2;

    println!("{} + {}i", result.re, result.im);

    // while true {} 대신 loop {}

    'outer: for x in 0.. { // 중첩 반복문 break
        for y in 0.. {
            for z in 0.. {
                if x + y + z > 1000 {
                    break 'outer;
                }
            }
        }
    }

    let item = 10;
    match item {
        0 => {}, // 0
        10 ..= 20 => {}, // 10 <= <= 20
        40 | 80 => {}, // 40 or 80
        _ => {}, // 나머지
    }

    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

    for item in &haystack {
        let result = match item {
            42 | 132 => "hit!",
            _ => "miss",
        };

        if result == "hit!" {
            println!("{}: {}", item, result);
        }
    }

    let num1 = 42;
    let num1_ref = &num1;
    let sum = num1 + *num1_ref;

    println!("num1 + num1 = {}", sum);

    // array
    let arr1 = [1, 2, 3];
    let arr2: [u8; 3] = [1, 2, 3];
    let arr3 = [0; 3];
    let arr4: [u8; 3] = [0; 3];

    let arrays = [arr1, arr2, arr3, arr4];

    for arr in &arrays {
        print!("{:?}: ", arr);
        for n in arr.iter() {
            print!("\t{} + 10 = {}", n, n+10);
        }

        let mut sum = 0;
        for i in 0..arr.len() {
            sum += arr[i];
        }
        println!("\t({:?} = {})", arr, sum);
    }
}

fn add(i: i32, j: i32) -> i32 { // 함수 정의 시 타입 반드시 필요
    i + j // 결과 반환에 return 필요 없음
}
