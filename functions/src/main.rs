fn main() {
    println!("Hello, world!");

    print_labeled_measurement(5, 'h');

    let y = {
        let x = 3;
        x + 1 // (중요) x + 1 이 ;로 끝나지 않음
    };

    println!("The value of y is: {y}");

    let f = five();

    println!("The value of f is: {f}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("Another Function, x is: {value}{unit_label}");
}

fn five() -> i32 {
    // 반환되는 값의 타입을 -> 뒤에 선언
    5 // 여기서도 역시 ;을 붙이지 않음
}
