
# rs

이 페이지는 여러 러스트 프로젝트에 쓰일 수 있는 기본적인 레이아웃에 대해 소개한다.

우선 파일 구성은 다음과 같이 이루어져 있다.

* Cargo.toml
* src/
  * bin/
    * main.rs
  * lib/
    * lib.rs
    * prelude.rs

각각의 역할이 무엇인지는 하나씩 설명하도록 하겠다.

## Cargo.toml

프로젝트의 루트에 위치하며, 프로젝트의 기본적인 정보를 가지고 있다. 러스트에서 프로젝트는 패키지라고도 불린다. 패키지는 하나 또는 여러 개의 크레이트를 가질 수 있으며, 크레이트는 라이브러리 또는 바이너리다.

```embed-toml
PATH: vault://code/rs/_/Cargo.toml
```

여기 기본적인 Cargo.toml 파일이 있다.

package, lib, bin 같은 것들은 섹션이라고 하는데, 대괄호로 감싸져 있는 걸 볼 수 있다. 하나뿐인 섹션이라면 한 겹의 대괄호를 사용하고, 여러 개가 존재할 수 있는 섹션이라면 두 겹의 대괄호를 사용한다.

lib와 bin이 지정되어 있는 걸 볼 수 있다. 그리고 default-run은 main으로 지정되어 있다.

```embed-rust
PATH: vault://code/rs/_/src/lib/prelude.rs
```

prelude라는 것은 보통 기본적인 포함 요소라는 것을 의미한다. 위에 쓴 내용은 러스트 코딩 초기 단계에서 터미널 입출력에 관한 불필요한 부분을 최소화하기 위해 쓴 내용이다.

```embed-rust
PATH: vault://code/rs/_/src/lib/lib.rs
```

lib.rs에서는 prelude를 모듈로 선언한 뒤 pub use로 prelude의 모든 내용을 사용하고 있다. prelude의 내용은 바이너리에서 `use lib::*;` 또는 `use lib::prelude::*;` 등으로 사용할 수 있다.

```embed-rust
PATH: vault://code/rs/_/src/bin/main.rs
```

라이브러리의 내용을 본인 것처럼 자연스럽게 쓰기 위해 `use lib::*;`를 썼다. `main` 함수의 내용은 prelude의 사용 예시를 보여준다.

## 실행 결과

```
What is your name? Rust
Hello Rust
```
