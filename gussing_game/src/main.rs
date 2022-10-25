// io 라이브러리 가져오기, std : 표준 라이브러리안에 io라이브러리가 있다.
use std::io;

// fn 문법은 새로운 함수를 선언, 빈 괄호는 매개변수가 없다는 뜻이다.
fn main() {
    println!("숫자를 맞춰봅시다!");

    println!("정답이라고 생각하는 숫자를 입력하세요.");

    // 변수에 값 저장, string 타입의 instance를 생성하는 함수의 실행 결과를 바인딩했다. new 함수는 어떤 타입의 새로운 값을 생성하는 함수에 일반적으로 부여하는 이름
    let mut guess = String::new();      // :: 문법은 new 함수가 string 타입의 연관 함수 라는 점을 의미한다. 다른 언어에서는 이것을 정적 메서드(static method)라고 부르기도 한다.

    /*
        let foo = bar; 는 foo라는 변수를 만든 후 bar라는 변수의 값을 binding한다.
        러스트에서 변수는 기본적으로 값을 변경할 수 없다.
        그래서 값을 변경할 수 있는 변수를 생성하기 위해 변수명 이전에 mut 키워드를 사용

        let foo = 5;            불변 변수
        let mut bar = 5;        가변 변수
    */
    /* 그럼 stdin은 io의 연관 함수라는 뜻이다.
        read_line method를 통해 사용자가 입력한 값을 읽어온다. 그리고 &mut guess 인수를 read_line method에 전달한다.
        &(reference)는 참조를 의미하는데 메모리의 복사 없이 접근할 수 있다는 것을 의미한다.
        변수와 마찬가지로 참조 역시 기본적으로는 변경할 수 없다. 그래서 변경이 가능한 참조를 전달하기 위해 mut를 붙여줬다.
    */
    io::stdin().read_line(&mut guess)              // 2개의 method 호출은 2줄로 나눈다. 
        .expect("입력한 값을 읽지 못했습니다.");   

    println!("입력한 값: {}", guess);
}
