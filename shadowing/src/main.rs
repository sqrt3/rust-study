fn main() {
    let x = 5;
	
	// 아래처럼 변수의 이름을 같게 설정하면 새로운 값을 할당하고 해제하는듯..?
	let x = x + 1;
	
	{
		let x = x * 2;
		// 이 스코프 안에서만 x는 12로 한정됨
		println!("The value of x in the inner scope is: {x}");
	}
	
	// 그래서 아래의 출력은 The value of x is: 6
	println!("The value of x is: {x}");
}
