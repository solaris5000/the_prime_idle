#![windows_subsystem = "windows"]

use ggez::{*, graphics::{Mesh, TextLayout, Text, Rect, Color}, audio::SoundSource};
use rand::Rng;
use std::process::exit;


//unused since 0.9.3 const TIME_5_SECONDS : std::time::Duration = std::time::Duration::from_millis(5000);
const SCORE_NEW_PRIME_COST_MODIFIER : f64 = 1.0;
const WEALTH_NEW_PRIME_COST_MODIFIER : f64 = 0.4;
const PRISE_COST_MODIFIER_SPAWN_LIMIT : f64 = 2.2;
const PRISE_COST_MODIFIER_SPAWN_RATE : f64 = 1.2;
const SCORE_REPEATING_PRIME_COST_MODIFIER : f64 = 0.3;
const TICS_SPAWN_TIMER : u64 = 100; // use 60 fps math here, 60 fps * 5 secs = 300 tics

const TILE_SIZE : f32 = 40.0;
const TILES_OFFSET : f32 = 5.0;

#[derive(Debug, Default, Clone, Copy)]
/// Стуктура, описывающая себе значение клетки на поле
struct Boxy {
    value: u64,
}

impl Boxy {
    fn new() -> Boxy {
        Boxy { value: 1 }
    }

    fn spawn( value : u64 ) -> Boxy {
        Boxy { value }
    }
}

#[derive(Debug, Clone)]
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

    fn free(&mut self) {
        self.filler = None;
    }
}

impl Default for MatrixNode {
    fn default() -> Self {
        MatrixNode { aviable: true, filler: None }
    }
}


#[derive(Debug, Default, Clone)]
struct MatrixRow([MatrixNode; 4]);

struct Displacement {
    forward : bool,
    displasment : (i32, i32),
}

#[derive(Debug, Default, Clone)]
/// Структура, описывающую матрицу игрового поля, состоящую из нодов
struct GameMatrix([MatrixRow; 4]);

impl GameMatrix {
    /// Рандомно инициализирует матрицу игры с 1 начальным элементом
    fn init() -> GameMatrix {
        let (x, y) = GameMatrix::get_random_node_coords();
        //dbg!((x,y).clone());

        let mut initializator = GameMatrix {..Default::default()};
        initializator.0[x as usize].0[y as usize] = MatrixNode::new();
        initializator
    }

    /// Тестовый инициализатор
    fn inittest() -> GameMatrix {
        let (x, y) = GameMatrix::get_random_node_coords();

        let mut initializator = GameMatrix {
                0: [ MatrixRow { 0 : [ 
                                    MatrixNode {aviable : true, filler : Some(Boxy { value : 2 })},
                                    MatrixNode {aviable : true, filler : Some(Boxy { value : 2 })},
                                    MatrixNode {aviable : true, filler : Some(Boxy { value : 2 })},
                                    MatrixNode {aviable : true, filler : Some(Boxy { value : 2 })}
                ]}, 
                MatrixRow { 0 : [   MatrixNode {aviable : true, filler : Some(Boxy { value : 2 })},
                                    MatrixNode {aviable : true, filler : Some(Boxy { value : 2 })},
                                    MatrixNode {aviable : true, filler : Some(Boxy { value : 2 })},
                                    MatrixNode {aviable : true, filler : Some(Boxy { value : 2 })}
                ]}, 
                MatrixRow { 0 : [   MatrixNode {aviable : true, filler : Some(Boxy { value : 2 })},
                                    MatrixNode {aviable : true, filler : Some(Boxy { value : 2 })},
                                    MatrixNode {aviable : true, filler : Some(Boxy { value : 2 })},
                                    MatrixNode {aviable : true, filler : Some(Boxy { value : 2 })}
                ]}, 
                MatrixRow { 0 : [   MatrixNode {aviable : true, filler : Some(Boxy { value : 2 })},
                                    MatrixNode {aviable : true, filler : Some(Boxy { value : 2 })},
                                    MatrixNode {aviable : true, filler : Some(Boxy { value : 2 })},
                                    MatrixNode {aviable : true, filler : Some(Boxy { value : 2 })}
                ]}, 

                ]
        };
        //initializator.0[x as usize].0[y as usize] = MatrixNode::new();
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
                    None => {_tmp += "  X "},
                    Some(val) => {_tmp += &format!("{:^4}", val.value)}
                }
            }
            _tmp += "\n";
        }
        _tmp += "\nInput direction or command (T B L R collected):\n";
        print!("{_tmp}");
    }

    /// Функция смещения всех значений в матрице к какому-либо краю, в зависимости от направления
    fn lean(&mut self, direction : MatrixNodesMoveDirection) {
        let displacement = match direction {
            MatrixNodesMoveDirection::ToBottom => {
                Displacement {forward : false, displasment : (1, 0)}
            },
            MatrixNodesMoveDirection::ToLeft => {
                Displacement {forward : true, displasment : (0, -1)}
            },
            MatrixNodesMoveDirection::ToRight => {
                Displacement {forward : false, displasment : (0, 1)}
            },
            MatrixNodesMoveDirection::ToTop => {
                Displacement {forward : true, displasment : (-1, 0)}
            },
        };

        if displacement.forward {
            for row in 0..4usize {
                for col in 0..4usize {

                    // проверяем что текущее значение не является None, чтобы не двигать пустоту зазря
                    match self.0[row].0[col].filler {
                        None => {continue;},
                        Some(_) => {}
                    };
                    if displacement.displasment.0 == -1 {
                        // Условия для отсечки пограничных значений верха и лево
                        if  row == 0 {
                            continue;
                        } else {
                            for rw in (1..=row).rev() {

                                match self.0[(rw as i32 + displacement.displasment.0) as usize].0[col].filler {
                                    None => {
                                        self.0[(rw as i32 + displacement.displasment.0) as usize].0[col].filler = Some(Boxy { value: self.0[rw].0[col].filler.clone().unwrap().value });
                                        self.0[rw].0[col].filler = None;
                                    },
                                    Some(_) => {break;}
                                };
                            }
                        };
                    };

                    if displacement.displasment.1 == -1 {
                        if col == 0 {
                        continue;
                    } else {
                        for cl in (1..=col).rev() {
                            match self.0[row].0[(cl as i32 + displacement.displasment.1) as usize].filler {
                                None => {
                                    self.0[row].0[(cl as i32 + displacement.displasment.1) as usize].filler = Some(Boxy { value: self.0[row].0[cl].filler.clone().unwrap().value });
                                    self.0[row].0[cl].filler = None;
                                },
                                Some(_) => {break;}
                            };
                        }
                    };
                }
                }

            } 
        }   // displacment.forward == false =>
        else {
            for row in (0..4usize).rev() {
                for col in (0..4usize).rev() {

                    // проверяем что текущее значение не является None, чтобы не двигать пустоту зазря
                    match self.0[row].0[col].filler {
                        None => {continue;},
                        Some(_) => {}
                    };
                    if displacement.displasment.0 == 1 {
                        // Условия для отсечки пограничных значений верха и лево
                        if  row == 3 {
                            continue;
                        } else {
                            for rw in row..3usize {
                                match self.0[(rw as i32 + displacement.displasment.0) as usize].0[col].filler {
                                    None => {
                                        self.0[(rw as i32 + displacement.displasment.0) as usize].0[col].filler = Some(Boxy { value: self.0[rw].0[col].filler.clone().unwrap().value });
                                        self.0[rw].0[col].filler = None;
                                    },
                                    Some(_) => {break;}
                                };
                            }
                        };
                    };

                    if displacement.displasment.1 == 1 {
                        if col == 3 {
                        continue;
                    } else {
                        for cl in col..3usize {
                            match self.0[row].0[(cl as i32 + displacement.displasment.1) as usize].filler {
                                None => {
                                    self.0[row].0[(cl as i32 + displacement.displasment.1) as usize].filler = Some(Boxy { value: self.0[row].0[cl].filler.clone().unwrap().value });
                                    self.0[row].0[cl].filler = None;
                                },
                                Some(_) => {break;}
                            };
                        }
                    };
                }
                }

            } 

        }
    }

    // fn new_check_conjoinn(&mut self, player : &std::sync::Arc<RwLock<Player>>) { старая сигнатура
    fn new_check_conjoinn(&mut self, player : &mut Player) -> bool {
        // я думал что в функцию надо передать arc..., но опказывается я вызываю функцию итак из того, что получаю из arc rwlock 
        //let matrix = mtx.write().unwrap();
        let matrix = self;
        //let mut player = player.write().unwrap();

        let rows_limit = 3usize;
        let cols_limit = 3usize;

        // Флаги того, какие элементы будут слиты
        let mut f_top = false;
        let mut f_bot = false;
        let mut f_lef = false;
        let mut f_rig = false;

        let mut return_conjoining_result = false;

        // проходимся по каждой строке
        for row in 0..=rows_limit {
            // проходимся по каждой клетке
            for col in 0..=cols_limit {
                // Обнуление флагов
                f_top = false;
                f_bot = false;
                f_lef = false;
                f_rig = false;

                let conjoined = 
                'conjoiner : loop  {
                    match matrix.0[row].0[col].filler {
                    None => { break false; }, // если у нас точка матрицы пустая, зачем нам с ней вообще работать?
                        Some(operating_cell) => {
                            // Создаём массив из возможных соседей клетки
                            let mut operating_cell_neighbours : [Option<&MatrixNode>;4] = [None; 4];

                            // Заполняем массив соседей
                            for it in 0usize..=3usize {
                                match it {
                                    // Верх
                                    0usize => {
                                        if row != 0 {
                                            operating_cell_neighbours[it] = Some(&matrix.0[row-1usize].0[col]);
                                        }
                                    },
                                    // Лево
                                    1usize => {
                                        if col != 0 {
                                            operating_cell_neighbours[it] = Some(&matrix.0[row].0[col-1usize]);
                                        }
                                    },
                                    // Право
                                    2usize => {
                                        if col != cols_limit {
                                            operating_cell_neighbours[it] = Some(&matrix.0[row].0[col+1usize]);
                                        }
                                    },
                                    // Низ
                                    3usize => {
                                        if row != rows_limit {
                                            operating_cell_neighbours[it] = Some(&matrix.0[row+1usize].0[col]);
                                        }
                                    },
                                    _ => { unreachable!("По идее сюда нельзя добраться, т.к. соседей у клетки максимум 4 и массив ток на 4 элемента")},
                                }
                            } //  for neighbour_inner in operating_cell_neighbours.into_iter().enumerate() end
                            // Обрабатываем поведение слияния, вливаем слияние внутрь

                            // Проверка что все 4 
                            let mut kinda_new_prime = operating_cell.value;
                            for it in 0..=3usize {
                                match operating_cell_neighbours[it] {
                                    None => {},
                                    Some(nei) => {
                                        match nei.filler {
                                            None => {},
                                            Some(val) => {
                                                kinda_new_prime += val.value;
                                                match it {
                                                    0usize => {f_top = true;},
                                                    1usize => {f_lef = true;},
                                                    2usize => {f_rig = true;},
                                                    3usize => {f_bot = true;},
                                                    _ => {unreachable!()},
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // Проверяем, станет ли целевая клетка новым простым числом и есть ли у неё соседи, замешанные в этом
                            if (primes::is_prime(kinda_new_prime) && (f_top || f_lef || f_rig || f_bot)) {
                                // Флаги установлены, новое, потенциально prime значение получено, можно выходить из цикла-обёртки
                                break 'conjoiner true
                            }
                            else {
                                // Обнуление флагов
                                f_top = false;
                                f_bot = false;
                                f_lef = false;
                                f_rig = false;
                                // оказывается только этого варианта недостаточно, пора переписывать

                                // проверка на одного изи соседей в сторонах по правилу В Л П Н
                                kinda_new_prime = operating_cell.value;
                                for it in 0..=3usize {
                                    match operating_cell_neighbours[it] {
                                        None => {},
                                        Some(nei) => {
                                            match nei.filler {
                                                None => {},
                                                Some(val) => {
                                                    if primes::is_prime(kinda_new_prime + val.value) {
                                                        match it {
                                                            0usize => {f_top = true;},
                                                            1usize => {f_lef = true;},
                                                            2usize => {f_rig = true;},
                                                            3usize => {f_bot = true;},
                                                            _ => {unreachable!()},
                                                        }
                                                        break 'conjoiner true;
                                                    } 
                                                }
                                            }
                                        }
                                    }
                                }
                                break false
                            }
                        },
                    };
                };// loop { match  end

                // обработка флагов и нового значения
                if conjoined {
                    return_conjoining_result = true;
                    //matrix.pretty_console_print();
                    if f_top {
                        matrix.0[row].0[col].filler = Some(Boxy { value: matrix.0[row].0[col].filler.unwrap().value + matrix.0[row-1].0[col].filler.unwrap().value});
                        matrix.0[row-1].0[col].filler = None;
                    }

                    if f_lef {
                        matrix.0[row].0[col].filler = Some(Boxy { value: matrix.0[row].0[col].filler.unwrap().value +  matrix.0[row].0[col-1].filler.unwrap().value});
                        matrix.0[row].0[col-1].filler = None;
                    }

                    if f_rig {
                        matrix.0[row].0[col].filler = Some(Boxy { value : matrix.0[row].0[col].filler.unwrap().value +  matrix.0[row].0[col+1].filler.unwrap().value});
                        matrix.0[row].0[col+1].filler = None;
                    }

                    if f_bot {
                        matrix.0[row].0[col].filler = Some(Boxy { value: matrix.0[row].0[col].filler.unwrap().value +  matrix.0[row+1].0[col].filler.unwrap().value});
                        matrix.0[row+1].0[col].filler = None;
                    }
                    // Le classique debug lines
                    //println!("Conjoing new prime {} into the {} {}", matrix.0[row].0[col].filler.unwrap().value, row, col);
                     //           dbg!(f_top.clone());
                    //            dbg!(f_lef.clone());
                    //            dbg!(f_rig.clone());
                    //            dbg!(f_bot.clone());
                    player.add_score_wealth(matrix.0[row].0[col].filler.unwrap().value);
                    //matrix.pretty_console_print();
                }
            

            }
            
        }
        return_conjoining_result
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
    //fn check_conjoin(&mut self, rand_vec : bool, player : &std::sync::Arc<RwLock<Player>>) { старая версия
    fn check_conjoin(&mut self, rand_vec : bool, player : &mut Player) {
        //todo!("Необходимо сделать для версии 0.5.0 правильное слияние");
        //todo!("Необходимо сделать для версии 0.5.0 подсчёт очков за слияние, подсчёт собраных уникальных простых чисел");
        //todo!("0.5.2 -> необходимо сделать проверку на то, доступна ли яччейка игрового поля, задел на будущее расширение");
        //todo!("0.6.1 -> Пофиксить то, что только 1 значение за конжойн сливается");
        //todo!("0.6.2 -> Пофиксить то, что не идёт сливание 3 с 4 строки и 3 с 4 столбца");
        //todo!("0.7.0 -> рефакторинг конжойна, необходимо смотреть соседей со всех сторон, и втягивать их в число которое сливается, переписать документацию к методу")
        //todo!(self обернуть в arc, а то параллельности нету)
        !panic!("Вырезано в 0.7.0.");
        let mut moving = false;
        let mut boxy_from : (usize, usize) = (0usize, 0usize);
        let mut boxy_into : (usize, usize) = (0usize, 0usize);
        //let mut player = player.write().unwrap();

        let rows_limit = 3usize;
        let cols_limit = 3usize;

        
        

        for row in 0..=rows_limit {
            for col in 0..=cols_limit {
                let operating_cell = &self.0[row].0[col];

                // Порядок соседей для обработки В Л П Н
                // Сброс всех соседей со счетов, для начала новой итерации
                let mut operating_cell_neighbours : [Option<&MatrixNode>;4] = [None; 4];

                //todo!("0.7.0 -> рефакторинг конжойна, необходимо смотреть соседей со всех сторон, и втягивать их в число которое сливается, переписать документацию к методу");
                // Заделка на будущее возможное расширение игрового поля.
                if operating_cell.aviable {
                    match operating_cell.filler {
                        None => {/*ну если ячейка пустая, то зачем пытаться с ней что-то делать*/},
                        Some(mut oc_inner) => {
                            // А вот тут уже что-то делаем, если ячейка полная
                            // А именно, нужно проверить соседей со всех сторон
                            // Проверяем существование соседей в порядке В Л П Н
                            // Если соседи сущестувют, в цикле начинаем складывает число из рабочей ячейки с соседними
                            // Порядок сложения : В Л П Н
                            // Если в процессе сложения получилось простое число, записываем его в целевую ячейку, задействованных соседей очищаем
                            // Если простое число не получилось, а мы вышли из цикла, работаем дальше

                            for it in 0usize..=3usize {
                                match it {
                                    // Верх
                                    0usize => {
                                        if row != 0 {
                                            operating_cell_neighbours[it] = Some(&self.0[row-1usize].0[col]);
                                        }
                                    },
                                    // Лево
                                    1usize => {
                                        if col != 0 {
                                            operating_cell_neighbours[it] = Some(&self.0[row].0[col-1usize]);
                                        }
                                    },
                                    // Право
                                    2usize => {
                                        if col != cols_limit {
                                            operating_cell_neighbours[it] = Some(&self.0[row].0[col+1usize]);
                                        }
                                    },
                                    // Низ
                                    3usize => {
                                        if row != rows_limit {
                                            operating_cell_neighbours[it] = Some(&self.0[row+1usize].0[col]);
                                        }
                                    },
                                    _ => { unreachable!("По идее сюда нельзя добраться, т.к. соседей у клетки максимум 4 и массив ток на 4 элемента")},
                                }
                            } //  for neighbour_inner in operating_cell_neighbours.into_iter().enumerate() end

                            let mut maybe_new_prime = oc_inner.value;

                            /* Нужно придумать алгоритм или просто кусок кода, который бы охватывал все возможные комбанции для сляния
                            со всеми соседями, предполагаю, что стоит сделать следующее:
                            
                            сначала пробуем сливать всех соседей, если не получилось,
                            пробуем верх + низ, затем лево + право
                            если не получилось, идём по порядку проверок В Л П Н */

                            // Проверяем попытку слияния со всеми соседями вместе
                            for operating_neighbour in operating_cell_neighbours {
                                match operating_neighbour {
                                    None => {},
                                    Some(on_inner) => {
                                        match on_inner.filler {
                                            None => {},
                                            Some(on_inner_boxy) => {
                                                maybe_new_prime += on_inner_boxy.value;
                                            }
                                        }
                                    },
                                }
                            }
                            if primes::is_prime(maybe_new_prime) {

                                for mut ope_nei in operating_cell_neighbours {
                                    match ope_nei {
                                        None => {},
                                        Some(_) => { 
                                            let mut a = ope_nei.as_mut();
                                            a = None; 
                                        }
                                    }
                                }

                                oc_inner.value = maybe_new_prime;
                                //println!("All together");
                                // обработка на случай если все сразу стали 
                                //let a = operating_cell_neighbours[1].as_mut();
                                // Возможно так можно будет мутировать, надо будет разобраться с этим, либо до конца блока
                                // тащить массив с соседями и там его курочить
                            } else {
                                maybe_new_prime = oc_inner.value;
                            }

                            let mut even = true;
                            // Проверяем попытку слияния с соседями верх-низ
                            for operating_neighbour in operating_cell_neighbours {
                                if even {
                                    even = !even;
                                    match operating_neighbour {
                                        None => {},
                                        Some(on_inner) => {
                                            match on_inner.filler {
                                                None => {},
                                                Some(on_inner_boxy) => {
                                                    maybe_new_prime += on_inner_boxy.value;
                                                }
                                            }
                                        },
                                    }
                                } else {
                                    even = !even;
                                }
                                
                            }

                            if primes::is_prime(maybe_new_prime) {
                                // обработка на случай если все сразу стали 
                                //let a = operating_cell_neighbours[1].as_mut();
                                // Возможно так можно будет мутировать, надо будет разобраться с этим, либо до конца блока
                                // тащить массив с соседями и там его курочить
                            } else {
                                maybe_new_prime = oc_inner.value;
                            }

                            let mut even: bool = true;
                            // Проверяем попытку слияния с соседями лево-право
                            for operating_neighbour in operating_cell_neighbours {
                                if !even {
                                    even = !even;
                                    match operating_neighbour {
                                        None => {},
                                        Some(on_inner) => {
                                            match on_inner.filler {
                                                None => {},
                                                Some(on_inner_boxy) => {
                                                    maybe_new_prime += on_inner_boxy.value;
                                                }
                                            }
                                        },
                                    }
                                } else {
                                    even = !even;
                                }
                                
                            }

                            if primes::is_prime(maybe_new_prime) {
                                // обработка на случай если все сразу стали 
                                //let a = operating_cell_neighbours[1].as_mut();
                                // Возможно так можно будет мутировать, надо будет разобраться с этим, либо до конца блока
                                // тащить массив с соседями и там его курочить
                            } else {
                                maybe_new_prime = oc_inner.value;
                            }

                            // Проверяем попытку слияния В П Л Н
                            for operating_neighbour in operating_cell_neighbours {
                                match operating_neighbour {
                                    None => {},
                                    Some(on_inner) => {
                                        match on_inner.filler {
                                            None => {},
                                            Some(on_inner_boxy) => {
                                                if primes::is_prime(maybe_new_prime + oc_inner.value) {
                                                    // какая-то обработка если первй попавшийся сосед образует с этим числом простое число.
                                                }
                                            }
                                        }
                                    },
                                }
                            }
                        }
                    }
                } 
                /*
                Это старый кусок кода, слияние только 2 соседних элементов. Пораждает проблему 7.
                Проблемой 7 - тут названо невозможность составления простого числа из пар простых чисел до 7, плюс еденица

                moving = false; 
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
                                    }
                                }
                            }
                        } // if col != 3usize end

                        // ветка на случай если правый сосед несливаемый, проверяем нижнего соседа
                        if row != 3usize {
                        match &self.0[row+1usize].0[col].filler {
                            None => {},
                            Some(neigbour) => {
                                let temp = base.value + neigbour.value;
                                if primes::is_prime(temp) {
                                    boxy_from = (row, col);
                                    boxy_into = (row+1, col);
                                    moving = true;
                                }
                            }
                        }
                        }
                    }
                }

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
                    
                    
                    
                } */
            }
        } 

        

    }

    //fn gameover_check(&self, player : &std::sync::Arc<RwLock<Player>>) { Старая сигнатура
    fn gameover_check(&self) -> bool {
        for row in 0..3usize {
            for col in 0..3usize {
                match self.0[row].0[col].filler {
                    None => {  return false; } 
                    Some(_) => {},
                }
            }
        }
        true
        //let player = player.read().unwrap();
        /*println!("
        ===========================\n
                GAME OVER\n
        Your score: {}\n
        You have collected {} unique primes\n
        List of collected primes:", player.score, player.collected_primes.len());
        player.print_colledted();
        exit(0);*/
    }
    //#[allow(unreachable_code)]
    /// Создаёт в случайной точке новую ноду, содержащую значение по верхнему пределу.
    /// Если случайно сгенерированная точка уже занята, пробует повторно сгенерировать точку
    /// В случае, если все ячейки заняты, инициирует завершение игры
    //fn spawn(&mut self, upper_limit : u64, player : &std::sync::Arc<RwLock<Player>>) { Старая сигнатура
    fn spawn(&mut self, spawner : &Spawner,  player : &mut Player) -> bool {
        //todo!("Необходимо сделать для версии 0.3.0");
        //todo!("0.9.0 переписать эту функцию, чтобы она брала только пустые клетки, иначе возможен бесконечный цикл")
        if self.gameover_check() {
            return false;
        }

        loop {
            let point = GameMatrix::get_random_node_coords(); // Вот эту фкнцию надо переписать чтобы она брала только пустые клетки в расчёт
            match self.0[point.0 as usize].0[point.1 as usize].filler {
                Some(_) => {},
                None => {
                    let mut rng = rand::thread_rng();
                    let rand_value = rng.gen_range(1..=spawner.upper_limit);
                    self.0[point.0 as usize].0[point.1 as usize].filler = Some(Boxy::spawn(rand_value));
                    return true;
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

#[derive(Debug, Clone)]
/// Описывает объект, который содержит параметры для спавна новой ноды на поле по истечению таймера
struct Spawner {
    upper_limit: u64,
    increase_price : u64,
    ticks_to_spawn: u64,
    reduce_ticks_price : u64,
}

impl Spawner {
    fn increase_limit(&mut self) {
        self.upper_limit += 1;
    }
}

impl Default for Spawner {
    fn default() -> Self {
        Spawner { upper_limit: 1, increase_price: 11, ticks_to_spawn: TICS_SPAWN_TIMER, reduce_ticks_price : 11  }
    }
}

#[derive(Debug, Default, Clone)]
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
            //println!("Wow, you have collected {}", prime);
            self.collected_primes.push(prime);
            false
        }
    }

    /// Увеличивает количество очков игрока в зависимости от переданного в функцию простого числа
    /// При просчёте очков применяет модификаторы константы для новых и повторяющихся простых чисел
    fn add_score_wealth(&mut self, prime : u64) {
        //println!("adding score for {}", prime);
        if self.is_prime_collected(prime) {
            self.score += (prime as f64 * SCORE_REPEATING_PRIME_COST_MODIFIER).ceil() as u64;
        } else {
            self.score += (prime as f64 * SCORE_NEW_PRIME_COST_MODIFIER).ceil() as u64;
            self.wealth += (prime as f64 * WEALTH_NEW_PRIME_COST_MODIFIER).ceil() as u64;
        }
    }

    fn print_colledted(&self) {
        let mut first = true;
        for prime in &self.collected_primes {
            if first {
                first = !first;
                print!("{prime}");
            } else {
                print!(", {prime}");
            }
        }
    }
}

#[derive(Debug, Clone)]
enum GameState {
    Menu,
    Game,
    Pause,
    GameOver,
    Exiting,
    NewGame,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Menu
    }
}

#[derive(Debug, Default, Clone)]
/// Структура настроек, чего не понятного то
struct Settings {
    rand_conjoin_vector : bool, // Рандомизация направления слияния. Проверка будет происходить всё так же  ЛВ -> ПН
    spawn_timer : u64,
}


struct Sounds {
    pause : audio::Source,
    unpause : audio::Source,
    spawn : audio::Source,
    conjoin : audio::Source,
    checkout : audio::Source,
}

impl Sounds {
    fn init(ctx : &mut Context) -> GameResult<Sounds> {
        let pause = audio::Source::new(ctx, "/pause.ogg")?;
        let unpause = audio::Source::new(ctx, "/unpause.ogg")?;
        let spawn = audio::Source::new(ctx, "/spawn_beep.ogg")?;
        let conjoin = audio::Source::new(ctx, "/conjoin_sound.ogg")?;
        let checkout = audio::Source::new(ctx, "/checkout.ogg")?;
        let s = Sounds {
            pause,
            unpause,
            spawn,
            conjoin,
            checkout,
        };
        Ok(s)
    }
}

//#[derive(Debug, Default, Clone)]

/// Центральная структура, пакующая в себе все необходимые компоненты для работы игры
struct Game {
    player: Player,
    spawner: Spawner,
    matrix: GameMatrix,
    settings : Settings,
    gamestate : GameState,
    sounds : Sounds,
}



impl Game {
    fn new(sounds : Sounds) -> Game {
        Game { player: Player::default(),
            spawner: Spawner { ..Default::default()}, 
            //matrix: GameMatrix::init(),
            matrix: GameMatrix::inittest(),
            gamestate : GameState::Menu,
            settings : Settings { rand_conjoin_vector: true, spawn_timer : 5u64 },
            sounds 
        }
    }


    fn prepare_tile(_ctx: &mut Context, tile_position : (f32, f32), padding_color : graphics::Color, tile_number : usize, tile_data : &MatrixNode) -> (Mesh, Text) {

        let empty_color = graphics::Color::from_rgb(60, 60, 60);
        let (text, mesh)  = match tile_data.filler {
            None => {
                    (graphics::Text::new("").
                    set_bounds(mint::Vector2::<f32>{x : TILE_SIZE, y : TILE_SIZE}).
                    set_wrap(true).
                    to_owned(),

                    graphics::Mesh::new_rounded_rectangle(
                        _ctx, 
                        graphics::DrawMode::fill(), 
                        Rect {x : tile_position.0 + TILES_OFFSET, y : tile_position.1 + TILES_OFFSET, w : TILE_SIZE, h : TILE_SIZE}, 
                        10.0, 
                        empty_color).unwrap()
                )
            },
            Some( boxy ) => {
                (
                    graphics::Text::new(&boxy.value.to_string()).
                    set_bounds(mint::Vector2::<f32>{x : TILE_SIZE, y : TILE_SIZE}).
                    set_wrap(true).
                    set_layout(TextLayout { h_align: graphics::TextAlign::Middle, v_align: graphics::TextAlign::Middle }).
                    to_owned(),

                    graphics::Mesh::new_rounded_rectangle(
                        _ctx, 
                        graphics::DrawMode::fill(), 
                        Rect {x : tile_position.0 + TILES_OFFSET, y : tile_position.1 + TILES_OFFSET, w : TILE_SIZE, h : TILE_SIZE}, 
                        10.0, 
                        padding_color).unwrap()
                )
            }
        };
        (mesh, text)
    }


    fn increase_spawn_limit(&mut self, _ctx: &mut Context,) {
        if self.player.wealth >= self.spawner.increase_price {
            self.player.wealth -= self.spawner.increase_price;
            self.spawner.increase_price = (self.spawner.increase_price as f64 * PRISE_COST_MODIFIER_SPAWN_LIMIT).ceil() as u64;
            self.spawner.increase_limit();
            let _ = self.sounds.checkout.play_detached(_ctx);
        }
    }

    fn reduce_spawn_tics(&mut self, _ctx: &mut Context,) {
        // some limitations to reduce spawnrate
        if self.player.wealth >= self.spawner.reduce_ticks_price &&  self.spawner.ticks_to_spawn != 1u64 {
            self.player.wealth -= self.spawner.reduce_ticks_price;
            self.spawner.reduce_ticks_price = (self.spawner.reduce_ticks_price as f64 * PRISE_COST_MODIFIER_SPAWN_RATE).ceil() as u64;
            self.spawner.ticks_to_spawn -= 1;
            let _ = self.sounds.checkout.play_detached(_ctx);
        }
    }

}

// Часть ответсвенная за отрисовку и оконность

impl ggez::event::EventHandler<GameError> for Game {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        if _ctx.time.check_update_time(60) { // all update is bounede to 1 fps
        match self.gamestate {
            GameState::NewGame => {
                // обновление игры
                self.matrix = GameMatrix::init();
                self.player = Player::default();
                self.spawner = Spawner { ..Default::default() };
                self.gamestate = GameState::Game;
                self.settings = Settings { rand_conjoin_vector: true, spawn_timer : TICS_SPAWN_TIMER };
            },

            GameState::Game   => {
                        //println!("tick");
                        self.settings.spawn_timer -=1;
                        if self.settings.spawn_timer == 0 {
                            self.settings.spawn_timer = TICS_SPAWN_TIMER;
            
                            if self.matrix.spawn(&self.spawner, &mut self.player) {
                                let _ = self.sounds.spawn.play_detached(_ctx);
                            } else {
                                self.gamestate = GameState::GameOver;
                            }
                        }
            
                    if self.matrix.new_check_conjoinn(&mut self.player) {
                        if self.sounds.conjoin.elapsed() > std::time::Duration::from_millis(200) || self.sounds.conjoin.elapsed().as_millis() == 0{
                            let _ = self.sounds.conjoin.play_detached(_ctx);
                        }
                    }


            },

            GameState::Pause => {
                ggez::timer::sleep(std::time::Duration::from_millis(100));
            },  
            GameState::GameOver => {
                ggez::timer::sleep(std::time::Duration::from_millis(100));
            }, 
            GameState::Menu => {
                ggez::timer::sleep(std::time::Duration::from_millis(100));
            }, 
            _ => { unimplemented!()},
        }
        } // all update is bounede to 1 fps
        
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        let mut canvas = graphics::Canvas::from_frame(_ctx, graphics::Color::from_rgb(185, 125, 125));

        match self.gamestate {
            
            GameState::Game | GameState::Pause | GameState::GameOver => {

                let mut tiles : Vec<(graphics::Mesh, graphics::Text)> = Vec::new();
        for i in 0..=3usize {
            for j in 0..=3usize {
                tiles.push(
                Game::prepare_tile(
                    _ctx, 
                    (15.0 + TILE_SIZE * j as f32 + TILES_OFFSET * j as f32, 15.0 + TILE_SIZE * i as f32 + TILES_OFFSET * i as f32),
                    graphics::Color::from_rgb(127, 127, 127), 
                    i*4+j, 
                    &self.matrix.0[i].0[j]));
            }
        }

        let matrix_field = graphics::Mesh::new_rounded_rectangle(
            _ctx, 
            graphics::DrawMode::fill(), 
            Rect {x : 15.0, y : 15.0, w : (TILE_SIZE + TILES_OFFSET) * 4.0 + TILES_OFFSET, h : (TILE_SIZE + TILES_OFFSET) * 4.0 + TILES_OFFSET}, 
            10.0, 
            Color::from_rgb(180, 180, 180)).unwrap();
        
        //Отрисовка подложки поля
        canvas.draw(&matrix_field, graphics::DrawParam::default());

        // Отрисовка таймера спавна 
        canvas.draw( &graphics::Text::new("Spawn in: ").add(
            &format!("{:.2}",(self.settings.spawn_timer as f32 / 60.0))).to_owned()
            , graphics::DrawParam::default().dest([
                _ctx.gfx.size().0 - 150.0, TILES_OFFSET
            ]));

        // Отрисовка ренджа спавнера
        canvas.draw( &graphics::Text::new(format!("Range: [1..{}]", self.spawner.upper_limit)).to_owned()
            , graphics::DrawParam::default().dest([
                _ctx.gfx.size().0 - 150.0, TILES_OFFSET + 20.0
            ]));

        // Отрисовка Стоимости улучшения unused since 0.9.3
        //canvas.draw( &graphics::Text::new(format!("Up price: {}", self.spawner.increase_price)).to_owned()
        //    , graphics::DrawParam::default().dest([
        //        _ctx.gfx.size().0 - 150.0, TILES_OFFSET + 40.0
        //    ]));

        // Отрисовка счётчика счёта
        canvas.draw( &graphics::Text::new("Score: ").add(self.player.score.to_string()).to_owned()
            , graphics::DrawParam::default().dest([
                _ctx.gfx.size().0 - 150.0, TILES_OFFSET + 40.0
            ]));

        // Отрисовка текущей "валюты" игрока
        canvas.draw( &graphics::Text::new("Wealth: ").add(self.player.wealth.to_string()).to_owned()
            , graphics::DrawParam::default().dest([
                _ctx.gfx.size().0 - 150.0, TILES_OFFSET + 60.0
            ]));
        
        // Отрисовка последних 3 собранных 
        canvas.draw( &graphics::Text::new("Last collected:").to_owned()
            , graphics::DrawParam::default().dest([
                _ctx.gfx.size().0 - 150.0, TILES_OFFSET + 80.0
            ]));
        if self.player.collected_primes.len() <= 3 {
            for i in 0..self.player.collected_primes.len() {
                canvas.draw( &graphics::Text::new(self.player.collected_primes[i].to_string()).to_owned()
                , graphics::DrawParam::default().dest([
                    _ctx.gfx.size().0 - 150.0, TILES_OFFSET + 100.0 + 15.0 * i as f32
                ]));
            }
        } else {
            let mut displasment = 0;
            for i in self.player.collected_primes.len()-3..self.player.collected_primes.len() {
                canvas.draw( &graphics::Text::new(self.player.collected_primes[i].to_string()).to_owned()
                , graphics::DrawParam::default().dest([
                    _ctx.gfx.size().0 - 150.0, TILES_OFFSET + 100.0 + 15.0 * displasment as f32
                ]));
                displasment+=1;
            }
            
        }

        // Отрисовка wow текста
        match self.player.collected_primes.last() {
            None => {},
            Some(val) => {
                canvas.draw( &graphics::Text::new(format!("Wow, you have collected: {}", val)).to_owned()
                , graphics::DrawParam::default().color(Color::BLACK)
                .dest([
                    15.0,  (TILE_SIZE + TILES_OFFSET) * 4.0 + TILES_OFFSET + 40.0
                ]));
            }
        }
        
       

        let button_enabled_color =  Color::from_rgb(180, 180, 180);
        let button_disabled_color =  Color::from_rgb(240, 240, 240);
        
        

        // Отрисовка кнопки улучшения спавн рейта
        let spawnrate_frame = graphics::Mesh::new_rectangle(
            _ctx, 
            graphics::DrawMode::fill(), 
            Rect { x: 240.0, y : 15.0, w : 100.0, h : 50.0},
            Color::from_rgb(0, 0, 0)).unwrap();

        let spawnrate_filler = graphics::Mesh::new_rectangle(
            _ctx, 
            graphics::DrawMode::fill(), 
            Rect { x: 243.0, y : 18.0, w : 94.0, h : 44.0},
            if self.player.wealth >= self.spawner.reduce_ticks_price {
                button_enabled_color  
            } else {
                button_disabled_color
            }
            ).unwrap();

            canvas.draw(&spawnrate_frame, graphics::DrawParam::default());
            canvas.draw(&spawnrate_filler, graphics::DrawParam::default());
        
        let spawnrate_text = graphics::Text::new(format!("Spawn rate\n{}", self.spawner.reduce_ticks_price)).
            set_bounds(mint::Vector2::<f32>{x : 100.0, y : 50.0}).
            set_wrap(true).
            set_layout(TextLayout { h_align: graphics::TextAlign::Middle, v_align: graphics::TextAlign::Middle }).
            to_owned();

    
        canvas.draw( &spawnrate_text
            , graphics::DrawParam::default().dest([
                290.0, 40.0
            ]));

            
        // Отрисовка кнопки улучшения макс спавна
        let spawnlimit_frame = graphics::Mesh::new_rectangle(
            _ctx, 
            graphics::DrawMode::fill(), 
            Rect { x: 240.0, y : 80.0, w : 100.0, h : 50.0},
            Color::from_rgb(0, 0, 0)).unwrap();


        let spawnlimit_filler = graphics::Mesh::new_rectangle(
            _ctx, 
            graphics::DrawMode::fill(), 
            Rect { x: 243.0, y : 83.0, w : 94.0, h : 44.0},
            if self.player.wealth >= self.spawner.increase_price {
                button_enabled_color  
            } else {
                button_disabled_color
            }).unwrap();

            canvas.draw(&spawnlimit_frame, graphics::DrawParam::default());
            canvas.draw(&spawnlimit_filler, graphics::DrawParam::default());
        
        let spawnlimit_text = graphics::Text::new(format!("Spawn limit\n{}", self.spawner.increase_price)).
            set_bounds(mint::Vector2::<f32>{x : 100.0, y : 50.0}).
            set_wrap(true).
            set_layout(TextLayout { h_align: graphics::TextAlign::Middle, v_align: graphics::TextAlign::Middle }).
            to_owned();
    
        canvas.draw( &spawnlimit_text
            , graphics::DrawParam::default().dest([
                290.0, 105.0
            ]));
        
        

        for i in tiles.into_iter().enumerate() {
            let col = (i.0 % 4) as f32;
            let row = ((i.0 / 4) as f32).floor();

        // Отрисовка самой матрицы игровой 
            canvas.draw(&i.1.0, graphics::DrawParam::default());
            canvas.draw(&i.1.1, graphics::DrawParam::default()
            .dest([
                15.0 + (TILE_SIZE + TILES_OFFSET) * (col) + ((TILE_SIZE ) / 2.0 + TILES_OFFSET),
                15.0 + (TILE_SIZE + TILES_OFFSET) * (row) + ((TILE_SIZE ) / 2.0 + TILES_OFFSET)
            ])
        );

        match &self.gamestate {
            GameState::Pause => {
                // В случае паузы, прорисовка окна пауза
                let pause_frame = graphics::Mesh::new_rectangle(
                    _ctx, 
                    graphics::DrawMode::fill(), 
                    Rect { x: 190.0, y : 115.0, w : 160.0, h : 90.0},
                    Color::from_rgb(0, 0, 0)).unwrap();

                let pause_filler = graphics::Mesh::new_rectangle(
                    _ctx, 
                    graphics::DrawMode::fill(), 
                    Rect { x: 193.0, y : 118.0, w : 154.0, h : 84.0},
                    Color::from_rgb(180, 180, 180)).unwrap();

                    canvas.draw(&pause_frame, graphics::DrawParam::default());
                    canvas.draw(&pause_filler, graphics::DrawParam::default());
                

                let pause_text = graphics::Text::new("PAUSED\nSpace \\ Pause - continue\nESC - Exit to menu").
                set_bounds(mint::Vector2::<f32>{x : 160.0, y : 90.0}).
                set_wrap(true).
                set_layout(TextLayout { h_align: graphics::TextAlign::Middle, v_align: graphics::TextAlign::Middle }).
                to_owned();

                // Отрисовка текста паузы
                canvas.draw( &pause_text
                , graphics::DrawParam::default().dest([
                    270.0, 160.0
                ]));

                
            }
            GameState::GameOver => {

                // В случае проигрыша, прорисовка окна поражения
                let lose_frame = graphics::Mesh::new_rectangle(
                    _ctx, 
                    graphics::DrawMode::fill(), 
                    Rect { x: 190.0, y : 115.0, w : 160.0, h : 90.0},
                    Color::from_rgb(0, 0, 0)).unwrap();

                let lose_filler = graphics::Mesh::new_rectangle(
                    _ctx, 
                    graphics::DrawMode::fill(), 
                    Rect { x: 193.0, y : 118.0, w : 154.0, h : 84.0},
                    Color::from_rgb(180, 180, 180)).unwrap();

                    canvas.draw(&lose_frame, graphics::DrawParam::default());
                    canvas.draw(&lose_filler, graphics::DrawParam::default());

                
                let lose_text = graphics::Text::new("YOU LOSE\nSpace - start new game\nESC - Exit to menu").
                set_bounds(mint::Vector2::<f32>{x : 160.0, y : 90.0}).
                set_wrap(true).
                set_layout(TextLayout { h_align: graphics::TextAlign::Middle, v_align: graphics::TextAlign::Middle }).
                to_owned();

                // Отрисовка текста паузы
                canvas.draw( &lose_text
                , graphics::DrawParam::default().dest([
                    270.0, 160.0
                ]));

            }
            _ => {},
        }
        
        }

            },
            GameState::Menu => {
                // Отрисовка меню
                canvas.draw( 
                &graphics::Text::new("THE PRIME IDLE\n\nPRESS ANY KEY TO START\nESC - TO EXIT").
                set_bounds(mint::Vector2::<f32>{x : 540.0, y : 320.0}).
                set_wrap(true).
                set_layout(TextLayout { h_align: graphics::TextAlign::Middle, v_align: graphics::TextAlign::Middle }).
                to_owned()
                , graphics::DrawParam::default().dest([
                    270.0, 160.0
                ]));
            },
            _ => { },
        }

        
        canvas.finish(_ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(
            &mut self,
            _ctx: &mut Context,
            _button: event::MouseButton,
            x: f32,
            y: f32,
        ) -> Result<(), GameError> {
            /*
            rate 240.0, y : 15.0, w : 100.0, h : 50.0
            limit 240.0, y : 80.0, w : 100.0, h : 50.0*/

        if (x >= 240.0 && x <= 340.0) && (y >= 15.0 && y <= 65.0) {
            self.reduce_spawn_tics(_ctx);
        };

        if (x >= 240.0 && x <= 340.0) && (y >= 80.0 && y <= 130.0) {
            self.increase_spawn_limit(_ctx);
        };
        

        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: input::keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<(), GameError> {
      //dbg!(input.clone());

      match self.gamestate {
        
        GameState::Game => {
            match input.scancode {
                32 => {
                    self.matrix.lean(MatrixNodesMoveDirection::ToRight);
                }, // D
                31 => {
                  self.matrix.lean(MatrixNodesMoveDirection::ToBottom);
                }, // S
                30 => {
                  self.matrix.lean(MatrixNodesMoveDirection::ToLeft);
                }, // A
                17 => {
                  self.matrix.lean(MatrixNodesMoveDirection::ToTop);
                }, // W
                1 | 57433 => {
                    let _ = self.sounds.pause.play_detached(ctx);
                    self.gamestate = GameState::Pause;
                }
                _ => {
                } // Any other
            }
        },
        GameState::GameOver => {
            match input.scancode {
                1 => {
                    let _ = self.sounds.pause.play_detached(ctx);
                    self.gamestate = GameState::Menu;
                },
                57  => {
                    let _ = self.sounds.pause.play_detached(ctx);
                    self.gamestate = GameState::NewGame;
                }
                _ => {
                } // Any other
            }
        },
        GameState::Menu => {
            match input.scancode {
                1 => {
                    exit(0);
                }
                _ => {
                    let _ = self.sounds.pause.play_detached(ctx);
                    self.gamestate = GameState::NewGame;
                } // Any other
            }
        },
        GameState::Pause => {
            match input.scancode {
                57433 | 57 => {
                    let _ = self.sounds.unpause.play_detached(ctx);
                    self.gamestate = GameState::Game;
                },
                1 => {
                    let _ = self.sounds.unpause.play_detached(ctx);
                    self.gamestate = GameState::Menu;
                },
                _ => {
                } // Any other
            }
        },
        _ => { unimplemented!()},
    }
    Ok(())
}
}



fn main() {  
    
    let mut c: conf::Conf = conf::Conf::new();
    c.window_mode = conf::WindowMode{height : 320.0, width : 540.0 ,  resizable : false, borderless : false, ..Default::default()};
    let (mut ctx, event_loop) = ContextBuilder::new("The Prime Idle", "Solaris5000")
        .default_conf(c)
        .build()
        .unwrap();
    ctx.gfx.set_window_title("The Prime Idle");

    let sounds = Sounds::init(&mut ctx);

    let game = Game::new(sounds.unwrap());

    //game.matrix.pretty_console_print();

    event::run(ctx, event_loop, game);

    //game.idle();
}

