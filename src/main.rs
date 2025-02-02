use player::Player;

mod player;

fn main() {
    let mut player = Player::init();
    player.search("test");
}
