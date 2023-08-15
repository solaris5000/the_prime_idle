#[derive(Debug)]
/// Стуктура, описывающая себе значение клетки на поле
struct Boxy {
    value : u32,
}
#[derive(Debug)]
/// Структура, описывающая ноду матрицы игрового поля
struct MatrixNode {
    aviable : bool,
    filler : Option<Boxy>
}


#[derive(Debug)]
/// Структура, описывающую матрицу игрового поля, состоящую из нодов
struct GameMatrix ([[MatrixNode; 4];4]);


#[derive(Debug)]
/// Описывает объект, который содержит параметры для спавна новой ноды на поле по истечению таймера
struct Spawner {
    upper_limit : u32,
    cooldown : std::time::Duration,
}

#[derive(Debug)]
/// Структура, описывающая игрока и все необходимые для него данные
struct Player {
    name : String,
    score : u32,
    wealth : u32,
    collected_primes : Vec<u32>,
}
#[derive(Debug)]

/// Центральная структура, пакующая в себе все необходимые компоненты для работы игры
struct Game {
    player : Player,
    spawner : Spawner,
    matrix : GameMatrix,
}
fn main() {
    println!("Hello, world!");
}
