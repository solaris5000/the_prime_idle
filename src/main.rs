#[derive(Debug, Default)]
/// Стуктура, описывающая себе значение клетки на поле
struct Boxy {
    value: u32,
}

impl Boxy {
    fn new() -> Boxy {
        Boxy { value: 1 }
    }
}

#[derive(Debug)]
/// Структура, описывающая ноду матрицы игрового поля
struct MatrixNode {
    aviable: bool,
    filler: Option<Boxy>,
}

impl MatrixNode {
    fn new() -> MatrixNode {
        MatrixNode {
            aviable: true,
            filler: Some(Boxy::new()),
        }
    }
}

impl Default for MatrixNode {
    fn default() -> Self {
        MatrixNode { aviable: true, filler: None }
    }
}


#[derive(Debug, Default)]
struct MatrixRow([MatrixNode; 4]);

#[derive(Debug, Default)]
/// Структура, описывающую матрицу игрового поля, состоящую из нодов
struct GameMatrix([MatrixRow; 4]);

impl GameMatrix {
    /// Рандомно инициализирует матрицу игры с 1 начальным элементом
    fn init() -> GameMatrix {
        let (x, y) = ((rand::random::<u8>() % 4u8), (rand::random::<u8>() % 4u8));
        dbg!((x,y).clone());

        let mut initializator = GameMatrix {..Default::default()};
        initializator.0[x as usize].0[y as usize] = MatrixNode::new();
        initializator
    }


    /// Консольная функция вывода текущей матрицы
    fn pretty_console_print(&self) {
        let mut _tmp = String::new();
        for row in 0..4usize {
            for col in 0..4usize {
                match &self.0[row].0[col].filler {
                    None => {_tmp += " x "},
                    Some(val) => {_tmp += &format!(" {} ", val.value)}
                }
            }
            _tmp += "\n";
        }
        println!("{_tmp}");
    }    
}

/*
impl IntoIterator for GameMatrix {
    type Item = MatrixRow;
    type IntoIter = std::array::IntoIter<Self::Item, 4usize>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}*/

#[derive(Debug, Default)]
/// Описывает объект, который содержит параметры для спавна новой ноды на поле по истечению таймера
struct Spawner {
    upper_limit: u32,
    cooldown: std::time::Duration,
}

#[derive(Debug, Default)]
/// Структура, описывающая игрока и все необходимые для него данные
struct Player {
    name: String,
    score: u32,
    wealth: u32,
    collected_primes: Vec<u32>,
}
#[derive(Debug, Default)]

/// Центральная структура, пакующая в себе все необходимые компоненты для работы игры
struct Game {
    player: Player,
    spawner: Spawner,
    matrix: GameMatrix,
}

impl Game {
    fn new() -> Game {
        Game { player: Player::default(), spawner: Spawner::default(), matrix: GameMatrix::init() }
    }
}

fn main() {
    let game = Game::new();

    game.matrix.pretty_console_print();
}
