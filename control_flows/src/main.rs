fn main() {
    let mut counter = 0;

    let result = loop { // result 를 선언함과 동시에 
                        // 반복문으로부터 반환된 값을 저장하도록 함
        counter += 1; // 매 틱마다 += 1을 실행

        if counter == 10 { // 매 틱마다 counter가 10과 같은지 검사
            break counter * 2; // 같다면 counter * 2 를 반환
        }
    }

    let mut count = 0;
    'counting_up: loop {
        println("count= {count}");
        let mut remaining = 10;

        loop {
            println("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count ==  {
                break `counting_up;
            }
            remaining -= 1;
        }

        coutn += 1;
    }
    println!("End count = {count}");
}