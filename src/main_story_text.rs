use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct GlobalStoryText(pub Vec<String>);

impl FromWorld for GlobalStoryText {
    #[allow(unused_variables)]
    fn from_world(world: &mut World) -> Self {
        let mut text = Vec::new();
        text.push("\"으으.. 뭐야..\"".to_string());
        text.push(
            "매서운 바람이 식은 땀을 훑으며 체온을 앗아가자, 찬 기운에 정신이 들었다.".to_string(),
        );
        text.push(
            "오늘도 어김없이 코딩을 조지고 있던 와중에, 눈을 떠 보니 한 번도 본 적이 없던 광경이 내 눈에 들어왔다.".to_string(),
        );
        text.push("눈을 깜빡이고 다시 보니, 내가 있던 곳은 어딘가의 숲이었다.".to_string());
        text.push("숲 속에서는 새들의 지저귐과 나뭇잎들의 속삭임이 들려왔다.".to_string());
        text.push("\"뭐냐고!!!!!!!!!!!!!!\"".to_string());
        text.push("갑자기 눈 앞의 풍경이 돌변했다. 그러나 나는 바뀐 풍경에 놀라는 대신, 하고 있던 일이 먼저 생각났다.".to_string());
        text.push("위대하신 갑 오브 갑이신 클라이언트님께서 주문하신 프로그램의 납품일이 일주일이 채 남지 않았고, 아직 완성하지 못한 상태였기 때문이다.".to_string());
        text.push(
            "납품일이 코 앞 까지 다가오기 시작해 발 등에 불이 떨어진 상태로 코딩을 하고 있었다."
                .to_string(),
        );
        text.push(
            "전에 없던 집중력을 발휘해 무아지경 상태에 돌입했으나, 돌연 숲 속에 떨어진 것이다."
                .to_string(),
        );
        text.push(
            "\"아.. 개열받네. 이제 조금만 손 보면 QA로 넘길 수 있었는데. 제발 농담이라고 해줘..\""
                .to_string(),
        );
        text.push("내 눈에 비치는 광경은 \'그게 농담이겠냐?\' 라고 놀리는 듯 했다.".to_string());

        GlobalStoryText(text)
    }
}
