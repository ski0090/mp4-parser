# Mp4 Parser

mp4파일의 헤더를 파싱해주는 라이브러리입니다

프로젝트의 구조는 다음과 같습니다

* examples: 라이브러리를 다른 프로젝트에 사용시의 예를 담고 있습니다
  * simple: cmd창에서 파싱한 내용을 보여줍니다
  * tui: `tui`에서 파싱한 내용을 보여줍니다(미완성)

* [mp4-macro](./mp4-macros/ReadMe.md): mp4-parser에서 사용하는 macro들을 갖고 있습니다
* [mp4-parser](./mp4-parser/ReadMe.md): 실질적으로 mp4해더를 파싱하는 라이브러리입니다