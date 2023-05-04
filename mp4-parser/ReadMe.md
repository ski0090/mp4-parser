# Mp4 parser

mp4-parser는 atom의 구조만 알면 쉽게 파싱할 수 있도록 다음과 같은 기능을 가집니다

## [BaseAtom](./src/atoms/mod.rs#24)

모든 아톰들이 가지는 필드에 대해 정의합니다

### 필드
* offset
* size
* name: atom name
* depth: 현재 아톰에 대한 depth를 가집니다

### 메소드

* new: 새로운 mp4 헤드를 파싱할 때 사용합니다
* next: 같은 depth의 다음 atom을 파싱할 때 사용합니다
* child: 자신의 자식 atom을 파싱할 때사용합니다

## [Mp4AtomParser](./src/atoms/mod.rs#12)

atom parsing을 지원하는 특성을 정의해주어야  합니다

## [Mp4AtomPrint](./src/atoms/mod.rs#19)

atom print를 지원하는 특성을 정의합니다
[mp4-macro](../mp4-macros/ReadMe.md)를 이용하여 매크로로 정의할 수 있습니다

Mp4AtomPrint를 이용하면 assciate 타입으로 컨테이너에 담아 [iterator 패턴](https://en.wikipedia.org/wiki/Iterator_pattern)으로 프린트를 수행할 수 있습니다

