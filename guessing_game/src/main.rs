use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
	
	let secret_number = rand::rng().random_range(1..=100); // start..=end
	println!("The secret number is {secret_number}");
	
	// secret_number와 guess가 같을때까지 무한 반복 시키기
	loop {
		println!("Please input your guess.");
	
		// let = 불변 ( const와 같은 불변이지만 런타임 할당 가능, 타입 명시가 선택 )
		// let mut = 가변
		// const = 불변 ( 컴파일 타임 상수만 런타임 할당 가능, 타입 명시가 필수 )
		let mut guess = String::new();
		
		// 한 줄 입력 받아서
		io::stdin()
			.read_line(&mut guess)			// guess를 '가변' 참조로
			.expect("Failed to read line"); // read_line에 실패했다면 except
			
		//let guess: u32 = guess.trim().parse().expect("Please type a number!");
		// unsigned int가 아니면 출력하는듯
		// trim() 으로 \r\n을 제거해줌
		
		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};
		// 숫자가 아니라면 continue로 무시하기
			
		// 입력 받은 guess 출력하기
		println!("You guessed: {guess}");
		
		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Too small!"), 				// 작거나
			Ordering::Greater => println!("Too big!"),				// 크거나
			Ordering::Equal => { println!("You win!"); break; }  	// 같으면 break로 루프 종료
		}
	}
}