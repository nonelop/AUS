# AUS Documentation

> Version of AUS: 0.4.0-alpha

## Contents

---

#### 1. Documentation of Geometry  
[1.1 Geometry Units](#1-geometry-units)  
[1.2 Geometry Shapes](#2-geometry-shapes)  
[1.3 Vector3](#3-vector3)  

#### 2. Documentation of Physics  
[2.1 Shape and Body](#1-shape-and-body)  
[2.2 Collide](#2-collide)  
[2.3 Move](#3-move)  

#### 3. Simulation Objects  
[3.1 Body](#1-body)  
[3.2 Object](#2-object)

---

English version of documentation comming later

---

## 1. Geometry
### 1. Geometry Units
```GeometryUnits``` - Это enum, в котором перечисляются еденицы измерения, помеченные типом f64  

Пример поля:  
```Nanometer(f64)```  

То есть когда мы пишем Nanometer(10.0), мы просто подставляем f64 под тип из enum'а.  

Методы, реализуемые этим enum'ом, иногда работают с так называемым "base_unit". Base Unit берется из конфига, где он задается так:  
```pub const BASE_GEOMETRY_UNIT: GeometryUnits = Nanometer(0.0); // Либо любая другая еденица```  

```GeometryUnits``` реализует методы конвертации в любую еденицу из всего enum.  

Методы, предоставляемые для конвертации:  
```to_base_unit(&self) to_meters(&self) to_nanos(&self) to_micros(&self) to_millis(&self) to_centis(&self) to_kilos(&self)```  

Ничего не принимают, возвращают обьект типа ```GeometryUnits```  

Под капотом любой метод конвертации кроме ```to_meters(&self)``` и ```to_base_unit(&self)``` конвертирует сначала в метры с помощью ```to_meters(&self)```, а затем по своей формуле приводит к нужной еденице.  

```to_base_unit(&self)``` сначала проверяет какой еденицей является BASE_GEOMETRY_UNIT, а затем вызывает функцию конвертации из списка выше.

```get_value_in_meters(&self)``` и ```get_value_in_base(&self)``` возвращают значение v в f64 из ```GeometryUnits::Some(v)```

Сам ```GeometryUnits``` нужен в основном для API и удобства, и не применяется внутри движка для расчетов. методы которые его принимают зачастую моментально конвертируют его в f64.  

Константа ```pub const POINT_SIZE: GeometryUnits``` задает то, какого маштаба будет 1.0 типа f64. Например если константа равна Nanometer(1.0), то расстояние между 0.0 и 1.0, выраженными f64, будет равно 1 Нанометру.  

### 2. Geometry Shapes.

Внутри подмодуля shape хранятся несколько основных обьектов. Один из них enum ```Shape```, остальные - сами фигуры  
#### ```Shape```
enum ```Shape``` - это обертка для удобного доступа ко всем фигурам сразу через один обьект. он имеет поля одноименные тому что они представляют, вот как это выглядит

```
pub enum Shape {
    Cube(Cube),
    Sphere(Sphere)
    /* И другие */
}
```

Я сделал его потому, что некоторые методы или поля могут принимать просто "какой нибудь Shape", а не что то конкретное типо Sphere.

#### Сами shape's
На данный момент доступен по факту 1 Обьект. Хотя куб и существует как структура, он пока не реализует ничего, поэтому задокументирую только сферу.

Сфера - структура. имеет радиус и центр. как это выглядит:
```
pub struct Sphere {
    pub center: Vector3,
    pub radius: f64
}
```

Стандартный impl включает в себя только ```new(position: Vector3, radius: GeometryUnits) -> Result<Self, GeometryErrors>```  

Возвращает ```InvalidSizeValue``` если радиус <= 0.0.  
Принимает радиус в GeometryUnits, который как я и говорил используется для внешнего мира. Внутри переводит его в f64.  

В модуле ```physics``` написан ```impl CollidableShape for Sphere```, благодаря чему сфера имеет больше методов чем просто new (подробнее про ```physics``` см. ниже)

### 3. Vector3
Трехмерный вектор. является структурой с 3 полями, как это выглядит:

```
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
```
Для создания экземпляра можно как использовать синтаксис ```Vector3 {x, y, z}```, так и использовать метод ```new```, созданный для упрощения записи.

#### Методы
```pub fn zero() -> Vector3```  
Возвращает ```Vector3 {0.0, 0.0, 0.0}```

```pub fn new(x: f64, y: f64, z: f64) -> Self```  
Возвращает ```Vector3 {x, y, z}```

```pub fn normalize(&self) -> Self```  
Возвращает нормированный вектор

```pub fn length(&self) -> f64```  
Возвращает длинну вектора по пифагору

Кроме того, можно выполнять операции + - / * += Между двумя Vector3, а также умножать его на f64

---

## 2. Physics
### 1. Shape and Body
Просто для понимания, Shape это лишь "Обертка", которая должна Регестрировать коллизию, но она не может реализовывать метод самого столкновения. для неё создан трейт ```CollidableShape```. Body же наоборот не регестрируют коллизию, а отвечают на столкновение действием. они имеют velocity и прочие поля, в отличии от Shape, что имеет только поля которые вообще делают его фигурой.

### 2. Collide
И Shape и Body считаются Collidable, но имеют два разных трейта  
```CollidableShape``` и ```CollidableBody```  
оба содержат методы для обработки столкновений.

#### Методы ```CollidableShape```
```fn is_colliding(&self, with: &Shape) -> bool;```  
Быстрая проверка на столкновение с другим Shape.

```fn get_collision_info(&self, with: &Shape) -> Option<CollisionInfo>;```  
Попытка получить информацию о столкновении с другим Shape. внутри использует метод ```is_colliding(&self, with: &Shape)```, если столкновения не было вернет None. Если оно было, возвращает обьект ```CollisionInfo```

#### Методы ```CollidableBody```
```fn collide(&mut self, with: &mut dyn Body, info: CollisionInfo);```  
Обрабатывает столкновение. Дефолтная реализация читает CollisionInfo и по его параметрам решает столкновение.

### 3. Move
Moveable может быть только Body. Трейт ```MoveableBody``` имеет методы  
```fn move_with_velocity(&mut self)```  
Дефолтная реализация прибавляет к позиции обьекта его velocity умноженное на ```DELTA_TIME```

---

## 3. Simulation Objects
### 1. Body 
Обьекты в AUS это структуры (```struct```) реализующие методы трейта ```Body```. Именно их реализация отличает Обьекты от чего либо другого, и позволяет работать с ними.  

В целом, обьекты это надстройка, собранная над законами физики и над просчетом геометрии. В будущем из обьектов будет состоять вся симуляция.  

Благодаря ним существует грань между Движком и Симуляцией.  

#### Методы ```Body```:  
```get_shape(&self) -> Shape```  
Возвращает ```Shape``` обьекта  

```get_velocity(&self) -> Vector3```  
Возвращает ```velovity``` обьекта  

```get_position(&self) -> Vector3```  
Возвращает позицию обьекта. Есть дефолтная реализация.

```set_shape(&mut self, shape: Shape)```  
Устанавливает ```Shape``` обьекту  

```set_velocity(&mut self, velocity: Vector3)```  
Устанавливает ```velocity``` обьекту  

```set_position(&mut self, position: Vector3)```  
Устанавливает позицию обьекта. Есть дефолтная реализация.  

Итак, если структура реализует все эти методы, то она может считаться обьектом. На примере Atom - ему достаточно возвращать и устанавливать свои поля:  
```rust
impl Body for Atom {
    fn get_shape(&self) -> Shape {self.shape}
    fn set_shape(&mut self, shape: Shape) {self.shape = shape}

    fn get_velocity(&self) -> Vector3 {self.velocity}
    fn set_velocity(&mut self, velocity: Vector3) {self.velocity = velocity}
}
```  

Но эти методы не дают обработки столкновений, движения и прочего. Для всего этого созданы отдельные подтрейты.  


```CollidableBody``` это подтрейт, зависим от супертрейта ```Body```. То есть, чтобы обьект мог реализовывать ```CollidableBody```, он должен реализовывать ```Body```.  

#### Методы ```CollidableBody```:  
```collide(&mut self, with: &mut dyn Body, info: CollisionInfo)```  
Обработка столкновения с другим обьектом, обозначенным with, с использованием информации из ```CollisionInfo```. Имеет дефолтную реализацию  

То есть благодаря добавлению реализации этого подтрейта в свой обьект, появится доступ к базовым функциям коллизий.  

```MoveableBody``` это подтрейт, зависим от супертрейта ```Body```.   

#### Методы ```MoveableBody```: 
```move_with_velocity(&mut self)```  
Обработка движения обьекта на основе ```velocity```. Имеет дефолтную реализацию, прибавляющую к позиции ```velocity``` * ```DELTA_TIME```  

Благодаря добавлению реализации этого подтрейта в свой обьект, появится доступ к базовым функциям движения.  

Существует также ```pub trait PhysicsBody: Body + CollidableBody + MoveableBody {}```  
Как видите, он просто совмещает в себя все выше перечисленные трейты Body. Не имеет методов, можно добавить пустую реализацию в обьект. Используется для обобщений в коде, например ```Vec<dyn PhysicsBody>``` - Вектор обьектов реализующих основные физические методы.

### 2. Object  
Итак, собрать обьект из всего этого легко. Сначала надо обьявить саму ```struct``` обьекта. Она должна иметь поля для его Shape и velocity. Например:  
``` rust
struct MyObject {
    shape: Shape,
    velocity: Vector3
}
```  
Это лишь вариант представления значений shape и velocity. на самом деле их можно поместить куда угодно, главное корректно отдавать их из геттеров.  

Далее надо реализовать трейт ```Body```, а именно геттеры и сеттеры для shape и velocity. Пример реализации:

```rust
impl Body for MyObject {
    fn get_shape(&self) -> Shape {self.shape}
    fn set_shape(&mut self, shape: Shape) {self.shape = shape}

    fn get_velocity(&self) -> Vector3 {self.velocity}
    fn set_velocity(&mut self, velocity: Vector3) {self.velocity = velocity}
}
```  

Далее можно подгрузить дефолтные методы Collide и Move, а также обернуть обьект в PhysicsBody:

```rust
impl CollidableBody for MyObject {}
impl MoveableBody for MyObject {}

impl PhysicsBody for MyObject {}
```  

Как видите, мы сами не обьявляем никакие методы. В этом случае берутся дефолтные, описанные выше. Теперь мы можем вызывать ```collide()``` и ```move_with_velocity()```.  

Конечно, вы можете писать свои методы и добавлять свои поля для обьекта. используйте ```impl MyObject {}``` и пишите в нем свои собственные методы, либо пишите свои реализации для методов из ```MoveableBody``` и ```CollidableBody```. Вы свободны обращатся с обьектом как захотите.  

Примером такого собранного обьекта можно поставить Atom, его код в src/objects/atom.rs