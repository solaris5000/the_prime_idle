use rand::Rng;

use crate::modules::player::*;
use crate::modules::consts::{GAMEFIELD_SIZE_X, GAMEFIELD_SIZE_Y, SPAWNING_ANIMATION_TICS};

enum MatrixNodesMoveDirection {
    ToLeft,
    ToRight,
    ToTop,
    ToBottom
}

struct Displacement {
    forward : bool,
    displasment : (i32, i32),
}


#[derive(Clone, Copy)]
enum GameObjectAnimation {
    Idle,
    Spawning(usize),
}

pub struct GameField {
    field : [[GameFieldNode; GAMEFIELD_SIZE_X];GAMEFIELD_SIZE_Y],
}

impl GameField {
    /// Функция, проверяюащая каждый элемент матрицы игрового поля на предмет возможности совместиться, в случае если это возможно
    /// увеличивает счёт игрока, если собралось новое уникальное простое число, так же увеличивается богатство игрока
    fn is_there_conjoins(&mut self, player : &mut Player) -> bool {

        let matrix = self;

        // Флаги того, какие элементы будут слиты
        let mut f_top = false;
        let mut f_bot = false;
        let mut f_lef = false;
        let mut f_rig = false;

        let mut return_conjoining_result = false;

        // проходимся по каждой строке
        for row in 0..GAMEFIELD_SIZE_Y {
            // проходимся по каждой клетке
            for col in 0..GAMEFIELD_SIZE_X {
                // Обнуление флагов
                f_top = false;
                f_bot = false;
                f_lef = false;
                f_rig = false;

                let conjoined = 
                'conjoiner : loop  {
                    if matrix.field[row][col].aviable {
                        match &matrix.field[row][col].filler {
                            None => { break false; }, // Заканчиваем работу с данной ячейкой, возвращаем false что она не слилась
                            Some(operating_cell) => {
                                // Проверяем, что данная клетка не находится в анимации, если нахъодится - не проверяем её на слияние
                                match operating_cell.animation { 
                                    GameObjectAnimation::Idle => {},
                                    _ => {//break false
                                    },
                                };
                            
                                // Создаём массив из возможных соседей клетки
                                let mut operating_cell_neighbours : [Option<&GameFieldNode>;4] = [None; 4];

                                // Заполняем массив соседей
                                for it in 0usize..=3usize {
                                    match it {
                                        // Верх
                                        0usize => {
                                            if row != 0 {
                                                operating_cell_neighbours[it] = Some(&matrix.field[row-1usize][col]);
                                            }
                                        },
                                        // Лево
                                        1usize => {
                                            if col != 0 {
                                                operating_cell_neighbours[it] = Some(&matrix.field[row][col-1usize]);
                                            }
                                        },
                                        // Право
                                        2usize => {
                                            if col != GAMEFIELD_SIZE_X-1 {
                                                operating_cell_neighbours[it] = Some(&matrix.field[row][col+1usize]);
                                            }
                                        },
                                        // Низ
                                        3usize => {
                                            if row != GAMEFIELD_SIZE_Y-1 {
                                                operating_cell_neighbours[it] = Some(&matrix.field[row+1usize][col]);
                                            }
                                        },
                                        _ => { unreachable!("По идее сюда нельзя добраться, т.к. соседей у клетки максимум 4 и массив ток на 4 элемента")},
                                    }
                                } //  for neighbour_inner in operating_cell_neighbours.into_iter().enumerate() end
                                // Обрабатываем поведение слияния, вливаем слияние внутрь


                                // Проверка что все 4 
                    let mut kinda_new_prime = operating_cell.value;
                    //Проходка по всем соседям
                    for it in 0..=3usize {
                        match operating_cell_neighbours[it] {
                            None => {},
                            Some(nei) => {
                                match &nei.filler {
                                    None => {},
                                    Some(val) => {
                                        match val.animation {
                                            GameObjectAnimation::Idle => {},
                                            _ => {
                                                //continue;
                                            }
                                        }
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
                                        match &nei.filler {
                                            None => {},
                                            Some(val) => {
                                                match val.animation {
                                                    GameObjectAnimation::Idle => {},
                                                    _ => {continue;}
                                                }
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
                            }
                        }
                    }

                    
                };// loop { match  end

                // обработка флагов и нового значения
                if conjoined {
                    println!("Here we are");
                    return_conjoining_result = true;
                    //matrix.pretty_console_print();
                    if f_top {
                        matrix.field[row][col].filler.as_mut().unwrap().value += matrix.field[row-1][col].filler.as_ref().unwrap().value; 
                        matrix.field[row-1][col].filler = None;
                    }

                    if f_lef {
                        matrix.field[row][col].filler.as_mut().unwrap().value += matrix.field[row][col-1].filler.as_ref().unwrap().value; 
                        matrix.field[row][col-1].filler = None;
                    }

                    if f_rig {
                        matrix.field[row][col].filler.as_mut().unwrap().value += matrix.field[row][col+1].filler.as_ref().unwrap().value; 
                        matrix.field[row][col+1].filler = None;
                    }

                    if f_bot {
                        matrix.field[row][col].filler.as_mut().unwrap().value += matrix.field[row+1][col].filler.as_ref().unwrap().value; 
                        matrix.field[row+1][col].filler = None;
                    }
                    // Le classique debug lines
                    //println!("Conjoing new prime {} into the {} {}", matrix.0[row].0[col].filler.unwrap().value, row, col);
                     //           dbg!(f_top.clone());
                    //            dbg!(f_lef.clone());
                    //            dbg!(f_rig.clone());
                    //            dbg!(f_bot.clone());
                    player.add_score_wealth(matrix.field[row][col].filler.as_ref().unwrap().value);
                    //matrix.pretty_console_print();
                }
            

            }
            
        }
        return_conjoining_result
    }


    fn pretty_console_print(&self) {
        
        let mut _tmp = String::new();
        for row in 0..GAMEFIELD_SIZE_Y {
            for col in 0..GAMEFIELD_SIZE_X {
                match &self.field[row][col].filler {
                    None => {_tmp += "  X "},
                    Some(val) => {_tmp += &format!("{:^4}", val.value)}
                }
            }
            _tmp += "\n";
        }
        _tmp += "\nInput direction or command (T B L R collected):\n";
        print!("{_tmp}");
    }

    fn init() -> GameField {
        let (x, y) = GameField::get_random_node_coords();
        //dbg!((x,y).clone());

        let mut initializator = GameField { 
            field: 
            [[GameFieldNode{aviable : false, filler : None, x : 0usize, y : 0usize}; 4];4]
        };

        for row in 0..GAMEFIELD_SIZE_Y {
            for col in 0..GAMEFIELD_SIZE_X {
                initializator.field[row][col].aviable = true;
                initializator.field[row][col].x = col;
                initializator.field[row][col].y = row;
                initializator.field[row][col].filler = None;
            }
        };

        initializator.field[y][x].filler = Some(GameObject::spawn(1));
        initializator
    }

    /// Функиця создания нового элемента    
    fn spawn(&mut self, spawner : &super::spawner::Spawner) -> bool {
        if self.gameover_check() {
            return false;
        }

        loop {
            let point = GameField::get_random_node_coords(); // Вот эту фкнцию надо переписать чтобы она брала только пустые клетки в расчёт

            match self.field[point.0][point.1].filler {
                Some(_) => {},
                None => {
                    self.field[point.0][point.1].x = point.1;
                    self.field[point.0][point.1].y = point.0;

                    self.field[point.0][point.1].filler = Some(GameObject::spawn(spawner.get_limit()));
                    return true;
                },
            }
        }
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
            for row in 0..GAMEFIELD_SIZE_Y {
                for col in 0..GAMEFIELD_SIZE_X {

                    // проверяем что текущее значение не является None, чтобы не двигать пустоту зазря
                    match self.field[row][col].filler {
                        None => {continue;},
                        Some(_) => {}
                    };
                    if displacement.displasment.0 == -1 {
                        // Условия для отсечки пограничных значений верха и лево
                        if  row == 0 {
                            continue;
                        } else {
                            for rw in (1..=row).rev() {

                                match self.field[(rw as i32 + displacement.displasment.0) as usize][col].filler {
                                    None => {
                                        self.field[(rw as i32 + displacement.displasment.0) as usize][col].filler = Some(
                                            GameObject { value: self.field[rw][col].filler.clone().unwrap().value, animation: GameObjectAnimation::Idle }
                                        );
                                        self.field[rw][col].filler = None;
                                    },
                                    Some(_) => {break;}
                                }
                            }
                        };
                    };

                    if displacement.displasment.1 == -1 {
                        if col == 0 {
                        continue;
                    } else {
                        for cl in (1..=col).rev() {
                            match self.field[row][(cl as i32 + displacement.displasment.1) as usize].filler {
                                None => {
                                    self.field[row][(cl as i32 + displacement.displasment.1) as usize].filler = Some(
                                        GameObject { value: self.field[row][(cl as i32 + displacement.displasment.0) as usize].filler.clone().unwrap().value, animation: GameObjectAnimation::Idle }
                                    );
                                    self.field[row][cl].filler = None;
                                },
                                Some(_) => {break;}
                            }
                        }
                    };
                }
                }

            } 
        }   // displacment.forward == false =>
        else {
            for row in (0..GAMEFIELD_SIZE_Y).rev() {
                for col in (0..GAMEFIELD_SIZE_X).rev() {

                    // проверяем что текущее значение не является None, чтобы не двигать пустоту зазря
                    match self.field[row][col].filler {
                        None => {continue;},
                        Some(_) => {}
                    };
                    if displacement.displasment.0 == 1 {
                        // Условия для отсечки пограничных значений верха и лево
                        if  row == 3 {
                            continue;
                        } else {
                            for rw in row..3usize {
                                match self.field[(rw as i32 + displacement.displasment.0) as usize][col].filler {
                                    None => {
                                        self.field[(rw as i32 + displacement.displasment.0) as usize][col].filler = Some(
                                            GameObject { value: self.field[rw][col].filler.clone().unwrap().value, animation: GameObjectAnimation::Idle }
                                        );
                                        self.field[rw][col].filler = None;
                                    },
                                    Some(_) => {break;}
                                }
                                };
                            }
                        };

                    if displacement.displasment.1 == 1 {
                        if col == 3 {
                        continue;
                    } else {
                        for cl in col..3usize {
                             match self.field[row][(cl as i32 + displacement.displasment.1) as usize].filler {
                                None => {
                                    self.field[row][(cl as i32 + displacement.displasment.1) as usize].filler = Some(
                                        GameObject { value: self.field[row][(cl as i32 + displacement.displasment.0) as usize].filler.clone().unwrap().value, animation: GameObjectAnimation::Idle }
                                    );
                                    self.field[row][cl].filler = None;
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

    /// Генерирует случайные координаты для матрицы, возвращая их в виде кортежа из 2 элементов
    fn get_random_node_coords() -> (usize, usize) {
        ((rand::random::<usize>() % 4usize), (rand::random::<usize>() % 4usize))
    }


    /// Проверяет все доступные клетки игрового поля
    fn gameover_check(&self) -> bool {
        for row in 0..GAMEFIELD_SIZE_Y {
            for col in 0..GAMEFIELD_SIZE_X {
                match self.field[row][col].filler {
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
}

#[derive(Clone, Copy)]
pub struct GameFieldNode {
    x : usize,
    y : usize,
    aviable : bool,
    filler : Option<GameObject>
}


#[derive(Clone, Copy)]
pub struct  GameObject {
    value : u64,
    animation : GameObjectAnimation
}  

impl GameObject {
    fn spawn(upper_limit : u64) -> GameObject {
        let mut rng = rand::thread_rng();
        let value = rng.gen_range(1..=upper_limit);
        GameObject { value , animation: GameObjectAnimation::Spawning(SPAWNING_ANIMATION_TICS) }
    }
}

#[cfg(test)]
mod tests {
    use crate::modules::player::Player;

    use super::{GameField, GameFieldNode, GameObject};


    #[test]
    fn exploration() {
        let gn = GameFieldNode {
            x : 0usize,
            y : 0usize,
            aviable : true,
            filler : None
        };

        let ga = GameFieldNode {
            x : 0usize,
            y : 0usize,
            aviable : true,
            filler : Some(GameObject::spawn(1))
        };

        let mut gf = GameField{
            field : [[gn;4];4]
        };

        gf.field[1][1] = ga;
        gf.field[1][2] = ga;
        gf.pretty_console_print();

        gf.lean(super::MatrixNodesMoveDirection::ToLeft);
        
        let mut player = Player::new();

        gf.is_there_conjoins(&mut player);

        gf.lean(super::MatrixNodesMoveDirection::ToRight);
        gf.lean(super::MatrixNodesMoveDirection::ToBottom);

        gf.pretty_console_print();
    }
}