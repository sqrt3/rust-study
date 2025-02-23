fn main() {
    let mut x = 5;
	println!("The value of x is: {x}");
	
	// mut이 없을땐 값 변경이 안되지만 mut을 붙여줌으로써 값 변경 가능
	x = 6;
	println!("The value of x is: {x}");
	
	// const는 let과 달리 타입 명시가 필수 + 불변
	const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
	println!("Three hours to seconds: {THREE_HOURS_IN_SECONDS}");
}