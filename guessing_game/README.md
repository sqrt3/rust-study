|만약 read_line을 &mut 없이 호출한다면?|최종 결과|
|---|---|
|![image](https://github.com/user-attachments/assets/94263a24-1780-4770-869a-d0babb180343)|![image](https://github.com/user-attachments/assets/aa67de5e-064a-4a1a-a8eb-8bd8a055c346)|
|read_line은 &mut String을 인자로 받기 때문에 컴파일이 안됨, 컴파일러가 &mut을 추가하는게 어떠냐고 물어봄||

### match에 대한 추가 사항
📌 match의 기본 구조
```rust
match 값 {
    패턴1 => 표현식1,
    패턴2 => 표현식2,
    _ => 기본값,  // 모든 경우를 처리하는 와일드카드
}
```
각 패턴이 값과 비교되면서 첫 번째로 일치하는 패턴이 실행됨.

✅ 기본 예제: 숫자 매칭
```rust
fn main() {
    let number = 3;

    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),  // number가 3이므로 여기가 실행됨
        _ => println!("Other"),  // 와일드카드: 나머지 모든 값 처리
    }
}
```
출력:
```
Three
```
✅ match를 이용한 열거형(enum) 처리
match는 특히 enum을 처리할 때 강력한 기능을 발휘함
```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let dir = Direction::Left;

    match dir {
        Direction::Up => println!("Moving Up"),
        Direction::Down => println!("Moving Down"),
        Direction::Left => println!("Moving Left"),  // dir이 Left이므로 실행됨
        Direction::Right => println!("Moving Right"),
    }
}
```
출력:

```
Moving Left
```
match를 쓰면 각각의 경우를 명확하게 처리할 수 있어서 안전하고 가독성이 좋음
✅ Option<T> 처리 (널값 방지)
Rust에서는 null이 없고, 대신 Option<T>을 사용함.

```rust
fn main() {
    let some_number = Some(5);
    let no_number: Option<i32> = None;

    match some_number {
        Some(n) => println!("Number: {}", n), // `Some(5)`에 매칭됨
        None => println!("No number"),
    }
}
```
출력:
```
Number: 5
```
이렇게 하면 None을 안전하게 처리 가능.

✅ match에서 값 바인딩
match를 사용하면 패턴에서 값을 변수에 바인딩 가능.

```rust
fn main() {
    let x = Some(10);

    match x {
        Some(n) => println!("Number: {}", n), // `n`에 10이 바인딩됨
        None => println!("No number"),
    }
}
```
출력:
```
Number: 10
```
✅ 여러 패턴 한 번에 매칭하기

여러 개의 패턴을 한꺼번에 처리 가능.

```rust
fn main() {
    let num = 2;

    match num {
        1 | 2 | 3 => println!("1, 2 또는 3입니다!"),
        4..=6 => println!("4에서 6 사이입니다!"),
        _ => println!("그 외의 숫자입니다!"),
    }
}
```
출력:
```
1, 2 또는 3입니다!
1 | 2 | 3 → OR 연산자 (|)를 사용해 여러 패턴을 한 번에 매칭
4..=6 → 범위 패턴 (4..=6)을 사용해 특정 범위를 한꺼번에 매칭
```
✅ 구조체 패턴 매칭
구조체의 필드 값을 패턴 매칭으로 분해할 수도 있음.

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 5, y: 10 };

    match point {
        Point { x: 0, y } => println!("Y축에 위치, y = {}", y),
        Point { x, y: 0 } => println!("X축에 위치, x = {}", x),
        Point { x, y } => println!("임의의 위치: ({}, {})", x, y),
    }
}
```
출력:
```
임의의 위치: (5, 10)
```
### match vs if let
if let은 특정 패턴만 처리할 때 match보다 간결함.

```rust
let some_number = Some(5);

if let Some(n) = some_number {
    println!("Number: {}", n);
}
```
출력:
```
Number: 5
```
하지만 모든 경우를 다 처리해야 한다면 match가 더 적합

🎯 정리
- match는 패턴 매칭을 수행하는 강력한 제어 흐름 도구
- switch 문과 비슷하지만 더 강력하고 유연한 패턴 매칭이 가능
- enum, Option<T>, Result<T, E>, 튜플, 구조체 등 다양한 값에 사용할 수 있음
- `_`(와일드카드)로 기본값 처리를 할 수 있음
- 단순한 경우는 if let을 사용하면 더 간결하게 표현 가능
