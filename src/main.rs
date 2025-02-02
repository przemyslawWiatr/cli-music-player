use player::Player;

mod player;

fn main() {
    let player = Player::init();
    player.search("test");
}
