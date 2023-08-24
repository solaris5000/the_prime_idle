use std::{thread, sync::RwLock, rc::Rc};

const TIME_5_SECONDS : std::time::Duration = std::time::Duration::from_millis(10);
const SCORE_NEW_PRIME_COST_MODIFIER : f64 = 1.0;
const SCORE_REPEATING_PRIME_COST_MODIFIER : f64 = 0.3;

#[derive(Debug, Default, Clone, Copy)]
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

    fn free(&mut self) {
        self.filler = None;
    }
}

impl Default for MatrixNode {
    fn default() -> Self {
        MatrixNode { aviable: true, filler: None }
    }
}


#[derive(Debug, Default)]
struct MatrixRow([MatrixNode; 4]);

struct Displacement {
    forward : bool,
    displasment : (i32, i32),
}

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

    /// Тестовый инициализатор
    fn inittest() -> GameMatrix {
        let (x, y) = GameMatrix::get_random_node_coords();

        let mut initializator = GameMatrix {
                0: [ MatrixRow { 0 : [ 
                                    MatrixNode {aviable : true, filler : Some(Boxy { value : 23 })},
                                    MatrixNode {aviable : true, filler : Some(Boxy { value : 1 })},
                                    MatrixNode {aviable : true, filler : Some(Boxy { value : 1 })},
                                    MatrixNode {aviable : true, filler : Some(Boxy { value : 23 })}
                ]}, 
                MatrixRow { 0 : [   MatrixNode {aviable : true, filler : Some(Boxy { value : 1 })},
                                    MatrixNode {aviable : true, filler : Some(Boxy { value : 199 })},
                                    MatrixNode {aviable : true, filler : Some(Boxy { value : 1 })},
                                    MatrixNode {aviable : true, filler : Some(Boxy { value : 1 })}
                ]}, 
                MatrixRow { 0 : [   MatrixNode {aviable : true, filler : Some(Boxy { value : 1 })},
                                    MatrixNode {aviable : true, filler : Some(Boxy { value : 1 })},
                                    MatrixNode {aviable : true, filler : Some(Boxy { value : 23 })},
                                    MatrixNode {aviable : true, filler : Some(Boxy { value : 1 })}
                ]}, 
                MatrixRow { 0 : [   MatrixNode {aviable : true, filler : Some(Boxy { value : 7 })},
                                    MatrixNode {aviable : true, filler : None},
                                    MatrixNode {aviable : true, filler : Some(Boxy { value : 1 })},
                                    MatrixNode {aviable : true, filler : Some(Boxy { value : 1 })}
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
        println!("{_tmp}");
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
                            println!("BOING");
                            continue;
                        } else {
                            println!("SOME COOLer STUFF");
                            for rw in (1..=row).rev() {
                                println!("SOME COOLer STUFF INNER");
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
                        println!("BOING");
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
                            println!("BOING");
                            continue;
                        } else {
                            println!("SOME COOLer STUFF");
                            for rw in row..3usize {
                                println!("SOME COOLer STUFF INNER");
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
                        println!("BOING");
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

    fn new_check_conjoinn(&mut self, player : &std::sync::Arc<RwLock<Player>>) {
        // я думал что в функцию надо передать arc..., но опказывается я вызываю функцию итак из того, что получаю из arc rwlock 
        //let matrix = mtx.write().unwrap();
        let matrix = self;
        let mut player = player.write().unwrap();

        let rows_limit = 3usize;
        let cols_limit = 3usize;

        // Флаги того, какие элементы будут слиты
        let mut f_top = false;
        let mut f_bot = false;
        let mut f_lef = false;
        let mut f_rig = false;

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
                    println!("Conjoing new prime {} into the {} {}", matrix.0[row].0[col].filler.unwrap().value, row, col);
                                dbg!(f_top.clone());
                                dbg!(f_lef.clone());
                                dbg!(f_rig.clone());
                                dbg!(f_bot.clone());
                    player.add_score(matrix.0[row].0[col].filler.unwrap().value);
                    //matrix.pretty_console_print();
                }
            

            }
            
        }
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
        //todo!("0.6.1 -> Пофиксить то, что только 1 значение за конжойн сливается");
        //todo!("0.6.2 -> Пофиксить то, что не идёт сливание 3 с 4 строки и 3 с 4 столбца");
        //todo!("0.7.0 -> рефакторинг конжойна, необходимо смотреть соседей со всех сторон, и втягивать их в число которое сливается, переписать документацию к методу")
        //todo!(self обернуть в arc, а то параллельности нету)
        let mut moving = false;
        let mut boxy_from : (usize, usize) = (0usize, 0usize);
        let mut boxy_into : (usize, usize) = (0usize, 0usize);
        let mut player = player.write().unwrap();

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
                                println!("All together");
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

    /// Увеличивает количество очков игрока в зависимости от переданного в функцию простого числа
    /// При просчёте очков применяет модификаторы константы для новых и повторяющихся простых чисел
    fn add_score(&mut self, prime : u64) {
        println!("adding score for {}", prime);
        if self.is_prime_collected(prime) {
            self.score += (prime as f64 * SCORE_REPEATING_PRIME_COST_MODIFIER).ceil() as u64;
        } else {
            self.score += (prime as f64 * SCORE_NEW_PRIME_COST_MODIFIER).ceil() as u64;
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
             //matrix: GameMatrix::inittest(),
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
                    //println!("==========DBG ZONE==========");
                    //matrix.write().unwrap().new_check_conjoinn(&player);
                    //println!("\n\n\n\n\n\n\n\n\n\n\n\n");
                    //matrix.read().unwrap().pretty_console_print();
                   // println!("==========DBG ZONE==========");
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
                        MatrixNodesMoveDirection::ToBottom => {println!("Bot"); matrix.write().unwrap().lean(MatrixNodesMoveDirection::ToBottom);},
                        MatrixNodesMoveDirection::ToLeft => {println!("Left"); matrix.write().unwrap().lean(MatrixNodesMoveDirection::ToLeft);},
                        MatrixNodesMoveDirection::ToRight => {println!("Right"); matrix.write().unwrap().lean(MatrixNodesMoveDirection::ToRight);},
                        MatrixNodesMoveDirection::ToTop =>{println!("Top"); matrix.write().unwrap().lean(MatrixNodesMoveDirection::ToTop);}
                    }

                    // функиця смещения матрицы по направлению
                    //todo!("0.5.х, смещение значений в матрице, разработать функцию которая будет принимать MatrixNodesMoveDirection");

                    matrix.write().unwrap().new_check_conjoinn(&player);
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
                    matrix.write().unwrap().new_check_conjoinn(&player);
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
    let game = Game::new();
   
    //game.matrix.pretty_console_print();

    game.idle();
}