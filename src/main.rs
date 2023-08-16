use std::{thread, sync::RwLock};

const TIME_5_SECONDS : std::time::Duration = std::time::Duration::from_millis(5000);
const SCORE_NEW_PRIME_COST_MODIFIER : f64 = 1.0;
const SCORE_REPEATING_PRIME_COST_MODIFIER : f64 = 0.3;

#[derive(Debug, Default)]
/// Стуктура, описывающая себе значение клетки на поле
struct Boxy {
    value: u64,
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
                    None => {_tmp += "    "},
                    Some(val) => {_tmp += &format!("{:4}", val.value)}
                }
            }
            _tmp += "\n";
        }
        println!("{_tmp}");
    }

    /// # Слияние
    /// Проверка полей матрицы на доступность к слиянияю по правилам слияния.
    /// Правила слияния зависят от переменной `rand_vec`, принимающей `bool`: 
    /// Сначала проверяем правого соседа, затем нижнего соседа. 
    /// Если слияние возможно -> проверяет значение `rand_vec = false` значение текущей ячейки добавляется к соседу, 
    /// а текущая ячейка заменяется на пустую, доступную; Иначе если `rand_vec = true`, 
    /// направление слияния зависит от результата зависит от генерируемого функицей `rand::random()` значения.
    /// 
    /// # Примеры
    /// `rand_vec = false`
    /// ```
    ///  x x x 1        x x x x
    ///  x x x 1    ->  x x x 2
    ///  x x x x        x x x x
    ///  x x x x        x x x x
    /// ```
    /// `rand_vec = true`
    /// ```
    ///  x 2 3 x        x x 5 x
    ///  2 3 x 2    ->  5 x x 2
    ///  x x 1 x        x x 2 x
    ///  x x 1 x        x x x x
    /// ```
    /// 
    /// # Начисление очков
    /// При обнаружении слияния игроку `player` начисляются очки, зависящие от констант 
    /// `SCORE_NEW_PRIME_COST_MODIFIER` и `SCORE_REPEATING_PRIME_COST_MODIFIER`, определяющих сколько игрок
    /// получит очков за получение очередного простого числа на поле
    #[allow(unreachable_code)]
    fn check_conjoin(&mut self, rand_vec : bool, player : &std::sync::Arc<RwLock<Player>>) {
        //todo!("Необходимо сделать для версии 0.5.0 правильное слияние");
        //todo!("Необходимо сделать для версии 0.5.0 подсчёт очков за слияние, подсчёт собраных уникальных простых чисел");
        //todo!("0.5.2 -> необходимо сделать проверку на то, доступна ли яччейка игрового поля, задел на будущее расширение");
        let mut moving = false;
        let mut boxy_from : (usize, usize) = (0usize, 0usize);
        let mut boxy_into : (usize, usize) = (0usize, 0usize);
        let mut player = player.write().unwrap();

        'outer: for row in 0..3usize {
            for col in 0..4usize {
                match &self.0[row].0[col].filler {
                    None => {},
                    Some(base) => {
                        if col != 3usize {
                            match &self.0[row].0[col+1usize].filler {
                                None => {},
                                Some(neigbour) => {
                                    let temp = base.value + neigbour.value;
                                    if primes::is_prime(temp) {
                                        boxy_from = (row, col);
                                        boxy_into = (row, col+1);
                                        moving = true;

                                        break 'outer;
                                    }
                                }
                            }
                        } // if col != 3usize end

                        // ветка на случай если правый сосед несливаемый, проверяем нижнего соседа
                        match &self.0[row+1usize].0[col].filler {
                            None => {},
                            Some(neigbour) => {
                                let temp = base.value + neigbour.value;
                                if primes::is_prime(temp) {
                                    boxy_from = (row, col);
                                    boxy_into = (row+1, col);
                                    moving = true;

                                    break 'outer;
                                }
                            }
                        }
                    }
                }
            }
        } // 'outer end

        if moving {
            let new_boxy = self.0[boxy_into.0].0[boxy_into.1].filler.as_ref().unwrap().value + &self.0[boxy_from.0].0[boxy_from.1].filler.as_ref().unwrap().value;

            // функция проверки содержания есть ли это простое число в векторе уже собранных простых чисел

            if player.is_prime_collected(new_boxy) {
                player.score += (new_boxy as f64 * SCORE_REPEATING_PRIME_COST_MODIFIER).ceil() as u64;
            } else {
                player.score += (new_boxy as f64 * SCORE_NEW_PRIME_COST_MODIFIER).ceil() as u64;
            }

            if rand_vec {
                if rand::random() {
                    self.0[boxy_into.0].0[boxy_into.1].filler = Some(Boxy { value: new_boxy });
                    self.0[boxy_from.0].0[boxy_from.1].filler = None;
                } else {

                    self.0[boxy_from.0].0[boxy_from.1].filler = Some(Boxy { value: new_boxy });
                    self.0[boxy_into.0].0[boxy_into.1].filler = None;
                }
            } else {
                self.0[boxy_into.0].0[boxy_into.1].filler = Some(Boxy { value: new_boxy });
                self.0[boxy_from.0].0[boxy_from.1].filler = None;
            }
            
            
            
        }

    }
    #[allow(unreachable_code)]
    /// Создаёт в случайной точке новую ноду, содержащую значение по верхнему пределу.
    /// Если случайно сгенерированная точка уже занята, пробует повторно сгенерировать точку
    fn spawn(&mut self, upper_limit : u64) {
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

enum MatrixNodesMoveDirection {
    ToLeft,
    ToRight,
    ToTop,
    ToBottom
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
    upper_limit: u64,
    cooldown: std::time::Duration,
}

#[derive(Debug, Default)]
/// Структура, описывающая игрока и все необходимые для него данные
struct Player {
    name: String,
    score: u64,
    wealth: u64,
    collected_primes: Vec<u64>,
}

impl Player {

    /// Проверят, собирал ли игрок уже переданное простое число.
    /// В случае если простое число не было собрано, возввращается false и число заносится в вектор собранных чисел
    /// В случае если простое число было собрано, возвращается true
    fn is_prime_collected(&mut self, prime : u64) -> bool {
        if self.collected_primes.contains(&prime) {
            true
        } else {
            self.collected_primes.push(prime);
            false
        }
    }
}

#[derive(Debug, Default)]
/// Структура настроек, чего не понятного то
struct Settings {
    rand_conjoin_vector : bool, // Рандомизация направления слияния. Проверка будет происходить всё так же  ЛВ -> ПН
}
#[derive(Debug, Default)]

/// Центральная структура, пакующая в себе все необходимые компоненты для работы игры
struct Game {
    player: Player,
    spawner: Spawner,
    matrix: GameMatrix,
    settings : Settings,
}

impl Game {
    fn new() -> Game {
        Game { player: Player::default(),
             spawner: Spawner { upper_limit: 5, cooldown: TIME_5_SECONDS /*std::time::Duration::from_millis(500)*/}, 
             matrix: GameMatrix::init(),
            settings : Settings { rand_conjoin_vector: true } }
    }


    /// Тестовый цикл для проверки логики приложения
    fn idle(self) {
        //loop {
            // делаем многопоточность между вводом данных, выводом и спавном.
            let matrix_arc = std::sync::Arc::new(std::sync::RwLock::new(self.matrix));
            let player_arc = std::sync::Arc::new(std::sync::RwLock::new(self.player));
            let settings_arc = std::sync::Arc::new(std::sync::RwLock::new(self.settings));
            let spawner_arc = std::sync::Arc::new(std::sync::RwLock::new(self.spawner));

            // создаём скоп, чтобы наши потоки не могли пережить функцию и оставить утечку данных
            thread::scope(|s| {

            // поток, ответственный за ввод данных от пользователя
            s.spawn(|| {
                let spawner = spawner_arc.clone();
                let matrix = matrix_arc.clone();
                let settings = settings_arc.clone();
                let player = player_arc.clone();

                loop {
                    let direction_input = &catch_input::input!("Choose direction (T B L R): ")[..];

                    let direction = match direction_input {
                        "T" => {MatrixNodesMoveDirection::ToTop},
                        "B" => {MatrixNodesMoveDirection::ToBottom},
                        "L" => {MatrixNodesMoveDirection::ToLeft},
                        "R" => {MatrixNodesMoveDirection::ToRight},
                        _ => {
                            println!("Wrong input, try again.\n");
                            continue;
                        },
                    };

                    match direction {
                        MatrixNodesMoveDirection::ToBottom => {println!("Bot");},
                        MatrixNodesMoveDirection::ToLeft => {println!("Left");},
                        MatrixNodesMoveDirection::ToRight => {println!("Right");},
                        MatrixNodesMoveDirection::ToTop =>{println!("Top");}
                    }

                    // функиця смещения матрицы по направлению
                    matrix.write().unwrap().check_conjoin(settings.read().unwrap().rand_conjoin_vector, &player);
                    println!("\n\n\n\n\n\n\n\n\n\n\n\n");
                    matrix.read().unwrap().pretty_console_print();
                    println!("Score : {}", player.read().unwrap().score);
                }
            });

            // поток, ответственный за спавн
            s.spawn(||{
                let spawner = spawner_arc.clone();
                let matrix = matrix_arc.clone();
                let settings = settings_arc.clone();
                let player = player_arc.clone();
                loop {
                    matrix.write().unwrap().spawn(spawner.read().unwrap().upper_limit.clone());
                    matrix.write().unwrap().check_conjoin(settings.read().unwrap().rand_conjoin_vector, &player);
                    println!("\n\n\n\n\n\n\n\n\n\n\n\n");
                    matrix.read().unwrap().pretty_console_print();
                    println!("Score : {}", player.read().unwrap().score);
                    std::thread::sleep(spawner.read().unwrap().cooldown.clone());
                }
            });
        });
            
            //self.matrix.check_conjoin(self.settings.rand_conjoin_vector, &mut self.player);
            //println!("Nodes conjoined: ");
            //self.matrix.pretty_console_print();
            
            
           // self.matrix.pretty_console_print();
            //println!("Score : {}", self.player.score);
            
        //}
    }
}

fn main() {
    let mut game = Game::new();

    //game.matrix.pretty_console_print();

    game.idle();
}