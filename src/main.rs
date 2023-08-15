
const TIME_5_SECONDS : std::time::Duration = std::time::Duration::from_millis(5000);

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
        let (x, y) = GameMatrix::get_random_node_coords();
        dbg!((x,y).clone());

        let mut initializator = GameMatrix {..Default::default()};
        initializator.0[x as usize].0[y as usize] = MatrixNode::new();
        initializator
    }

    /// Генерирует случайные координаты для матрицы, возвращая их в виде кортежа из 2 элементов
    fn get_random_node_coords() -> (u8, u8) {
        ((rand::random::<u8>() % 4u8), (rand::random::<u8>() % 4u8))
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

    /// Проверка полей матрицы на доступность к слиянияю по правилам слияния.
    /// Правила слияния: Сначала проверяем правого соседа, затем нижнего соседа. 
    /// Если слияние возможно -> значение текущей ячейки добавляется к соседу, 
    /// а текущая ячейка заменяется на пустую, доступную.
    /// 
    /// # Примеры
    /// ```
    ///  x x x 1        x x x x
    ///  x x x 1    ->  x x x 2
    ///  x x x x        x x x x
    ///  x x x x        x x x x
    /// 
    ///  x 2 3 x        x x 5 x
    ///  x 3 x x    ->  x 3 x x
    ///  x x x x        x x x x
    ///  x x x x        x x x x
    /// ```
    #[allow(unreachable_code)]
    fn check_conjoin(&mut self) {
        //todo!("Необходимо сделать для версии 0.3.0");

        let mut moving = false;
        let mut boxy_from : (usize, usize) = (0usize, 0usize);
        let mut boxy_into : (usize, usize) = (0usize, 0usize);

        'outer: for row in 0..4usize {
            for col in 0..4usize {
                match &self.0[row].0[col].filler {
                    None => {},
                    Some(_) => {
                        if col != 3usize {
                            match &self.0[row].0[col+1usize].filler {
                                None => {},
                                Some(_) => {

                                    boxy_from = (row, col);
                                    boxy_into = (row, col+1);
                                    moving = true;

                                    break 'outer;
                                }
                            }
                        } // if col != 3usize end

                        // ветка на случай если правый сосед несливаемый, проверяем нижнего соседа
                        match &self.0[row+1usize].0[col].filler {
                            None => {},
                            Some(_) => {
                                
                                boxy_from = (row, col);
                                boxy_into = (row+1, col);
                                moving = true;

                                break 'outer;
                            }
                        }
                    }
                }
            }
        } // 'outer end

        if moving {
            let new_boxy = self.0[boxy_into.0].0[boxy_into.1].filler.as_ref().unwrap().value + &self.0[boxy_from.0].0[boxy_from.1].filler.as_ref().unwrap().value;
            self.0[boxy_into.0].0[boxy_into.1].filler = Some(Boxy { value: new_boxy });
            self.0[boxy_from.0].0[boxy_from.1].filler = None;
        }

    }
    #[allow(unreachable_code)]
    /// Создаёт в случайной точке новую ноду, содержащую значение по верхнему пределу.
    /// Если случайно сгенерированная точка уже занята, пробует повторно сгенерировать точку
    fn spawn(&mut self, upper_limit : u32) {
        //todo!("Необходимо сделать для версии 0.3.0");
        loop {
            let point = GameMatrix::get_random_node_coords();
            match self.0[point.0 as usize].0[point.1 as usize].filler {
                Some(_) => {},
                None => {
                    self.0[point.0 as usize].0[point.1 as usize].filler = Some(Boxy::new());
                    break;
                },
            }
        }
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
        Game { player: Player::default(), spawner: Spawner { upper_limit: 5, cooldown: TIME_5_SECONDS }, matrix: GameMatrix::init() }
    }


    /// Тестовый цикл для проверки логики приложения
    fn idle(&mut self) {
        loop {
            self.matrix.check_conjoin();
            println!("Nodes conjoined: ");
            self.matrix.pretty_console_print();
            self.matrix.spawn(self.spawner.upper_limit);
            println!("New node spawned: ");
            self.matrix.pretty_console_print();
            std::thread::sleep(self.spawner.cooldown);
        }
    }
}

fn main() {
    let game = Game::new();

    game.matrix.pretty_console_print();
}
