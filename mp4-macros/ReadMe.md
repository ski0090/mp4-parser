# Mp4 macro

현재 Mp4 macro는 파싱한 mp4의 아톰들을 `print`해주는 역할을 맡습니다

사용법은 다음과 같습니다

각각의 매크로는 필드(멤버)에 대해 다음과 같은 함수를 만들어줍니다

```rust
#[derive(Debug, ImplMp4AtomPrint)]
pub struct Ftyp {
    base: BaseBox,
    #[print_comp()]
    major: String,
    #[print_comp()]
    minor: u32,
    #[print_comp(iter)]
    brands: Vec<String>,
}

impl Ftyp {
    fn print_major(&self) {
        // print func
    }

    fn print_minor(&self) {
        // print func
    }

    fn print_brands(&self) {
        // print func
    }
}

```

## Attribute

attribute로 다음의 기능을 지원합니다

Vector나 Map같은 iter를 가지고 있는 자료구조를 컨테이너라 부르겠습니다

* #[print_comp(iter)]: 컨테이너의 print를 정의합니다
* #[print_comp(st)]: 구조체자체를 print할 경우 사용합니다. "{:?}"의 형태로 print를 정의해 해줍니다
* #[print_comp(atom_container)]: `Mp4AtomPrint`가 정의된 구조체를 담고있는 컨테이너의 print를 정의해줍니다

## ImplMp4Print

`Mp4AtomPrint`를 매크로로 정의 해줍니다

`Mp4AtomPrint`는 자신의 Depth에 맞추어 print 해줍니다

Atom타입에 해당 매크로를 사용하는 것이 적합합니다

## Print

`Printer`매크로는 일반적인 구조체에 print를 만들어줍니다