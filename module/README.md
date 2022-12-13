# 모듈
코드를 더 작은 덩어리로 쪼갬으로서, 각각의 덩어리들은 개별적으로 이해하기 더 수월해집니다. 하지만 함수가 너무 많으면 어떤 일이 벌어질까요? 러스트는 조직화된 방식으로 코드의 재사용을 할 수 있게 해주는 모듈(module) 시스템을 갖추고 있습니다.


 여러분은 함수 (혹은 구조체나 열거형 같은 다른 코드들)를 다른 모듈로 뽑아낼 수 있으며, 여러분은 이것들의 정의가 모듈의 바깥쪽에서 볼 수 있도록 하거나(public) 혹은 보이지 않게 하도록 (private) 선택할 수 있습니



* mod 키워드는 새로운 모듈을 선언합니다. 모듈 내의 코드는 이 선언 바로 뒤에 중괄호 로 묶여서 따라오거나 다른 파일에 놓일 수 있습니다.
* 기본적으로, 함수, 타입, 상수, 그리고 모듈은 private입니다. pub 키워드가 어떤 아이템을 public하게 만들어줘서 이것의 네임스페이스 바깥쪽에서도 볼 수 있도록 합니다.
* use 키워드는 모듈이나 모듈 내의 정의들을 스코프 안으로 가져와서 이들을 더 쉽게 참조할 수 있도록 합니다.
