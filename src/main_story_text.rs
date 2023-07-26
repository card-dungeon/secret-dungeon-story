use bevy::prelude::*;
use std::collections::BTreeMap;

#[derive(Resource, Debug)]
pub struct GlobalStoryText {
    pub story: BTreeMap<String, Vec<String>>,
    pub story_part: Vec<String>,
}

impl FromWorld for GlobalStoryText {
    #[allow(unused_variables)]
    fn from_world(world: &mut World) -> Self {
        let story_part = vec!["myunghun_001".to_string(), "jaeho_001".to_string()];

        let mut story = BTreeMap::new();

        // myunghun_001: 명훈 1화
        let myunghun_001_text = vec![
            "<명훈>".to_string(),
            "\"으으.. 뭐야..\"".to_string(),
            "매서운 바람이 식은 땀을 훑으며 체온을 앗아가자, 찬 기운에 정신이 들었다.".to_string(),
            "오늘도 어김없이 코딩을 조지고 있던 와중에, 눈을 떠 보니 한 번도 본 적이 없던 광경이 내 눈에 들어왔다.".to_string(),
            "눈을 깜빡이고 다시 보니, 내가 있던 곳은 어딘가의 숲이었다.".to_string(),
            "숲 속에서는 새들의 지저귐과 나뭇잎들의 속삭임이 들려왔다.".to_string(),
            "\"뭐냐고!!!!!!!!!!!!!!\"".to_string(),
            "갑자기 눈 앞의 풍경이 돌변했다. 그러나 나는 바뀐 풍경에 놀라는 대신, 하고 있던 일이 먼저 생각났다.".to_string(),
            "위대하신 갑 오브 갑이신 클라이언트님께서 주문하신 프로그램의 납품일이 일주일이 채 남지 않았고, 아직 완성하지 못한 상태였기 때문이다.".to_string(),
            "납품일이 코 앞 까지 다가오기 시작해 발 등에 불이 떨어진 상태로 코딩을 하고 있었다.".to_string(),
            "전에 없던 집중력을 발휘해 무아지경 상태에 돌입했으나, 돌연 숲 속에 떨어진 것이다.".to_string(),
            "\"아.. 개열받네. 이제 조금만 손 보면 QA로 넘길 수 있었는데. 제발 농담이라고 해줘..\"".to_string(),
            "내 눈에 비치는 광경은 \'그게 농담이겠냐?\' 라고 놀리는 듯 했다.".to_string(),
        ];

        // jaeho_001: 재호 1화
        let jaeho_001 = vec![
            "<재호>".to_string(),
            "나는 어렸을 때 부터 눈치가 꽤 빠른 편 이라고 생각해왔다.".to_string(),
        ];

        story.insert(story_part[0].clone(), myunghun_001_text);
        story.insert(story_part[1].clone(), jaeho_001);

        GlobalStoryText { story, story_part }
    }
}
