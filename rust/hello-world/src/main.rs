
use std::thread;
use std::time::Duration;
 
fn ex_thread() {
   println!("El thread {:?} se está ejecutando", std::thread::current().id());
   thread::sleep(Duration::from_millis(1));
}

// Funcicón principal
fn main() {

    /* Tipos de datos
    bool	: The boolean type.
    char	: A character type.
    i8	: The 8-bit signed integer type.
    i16	: The 16-bit signed integer type.
    i32	: The 32-bit signed integer type.
    i64	: The 64-bit signed integer type.
    isize	: The pointer-sized signed integer type.
    u8 : The 8-bit unsigned integer type.
    u16 : The 16-bit unsigned integer type.
    u32 : The 32-bit unsigned integer type.
    u64 : The 64-bit unsigned integer type.
    usize : The pointer-sized unsigned integer type.
    f32	: The 32-bit floating point type.
    f64	: The 64-bit floating point type.
    array	: A fixed-size array, denoted [T; N], for the element type, T, and the non-negative compile-time constant size, N.
    slice	: A dynamically-sized view into a contiguous sequence, [T].
    str	: String slices.[let str = "Hello! I'm a str";]
    tuple	: A finite heterogeneous sequence, (T, U, ..).
    */

    //Mis datos
    let mut nombre = "Geoffrey"; //variable mutable (puede cambiar de valor)
    //let nombre = "Geoffrey"; //variable inmutable (puede cambiar de valor)
    let lenguaje = "Rust";
    const ANIO: i32 = 2019;
    nombre = "Plivia";

    //Variables tipo numeros
    let suma = 5 + 10;
    let resta = 95.5 - 4.3;
    let multiplicacion = 4 * 30;
    let division = 56.7 / 32.2;
    let modulo = 43 % 5;

    //variables tipo char
    let vchar: char = 'c';

    //bolleanos
    let mut vtrue = true;
    let vf: bool = false;

    //tuplas: estructura de datos que no cambian una vez definidas
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    //arreglos/vector
    let a = [1, 2, 3];
    let mut am = [1, 2, 3];
    let mut vector = vec![];

    //condicionales
    if vtrue {
        println!("Hola Mundo, Soy {} y el lenguaje {} en {}!",nombre,lenguaje,ANIO);
    }
    else {
         nombre = "Geoffrey";
         println!("Hola Mundo, Soy {} y el lenguaje {} en {}!",nombre,lenguaje,ANIO);
    }

    while vtrue {
        println!("Rust");
        vtrue = false;
    }

    //Ciclos con tam conocido
    for i in 0..3 {
        println!("{}", a[i]);
    }
    println!("----------------");
    //Ciclos con orden inverso
    for n in (0..3).rev() {
        println!("{}", a[n]);
    }
    println!("----------------");
    //Ciclos con tam no conocido
    for i in a.iter() {
        println!("{}", i);
    }

    println!("----------------");
 
    for x in 1..5 {
      vector.push( thread::spawn(|| { ex_thread(); } ) );
    }
 
    println!("Proceso concurrente.");
 
    for child in vector {
      match child.join() {
         Ok(_) => (),
         Err(porque) => println!("Fallo conjunto {:?}", porque),
      };
    }
    
}

/*fn imprime(arg: Type) -> ReturnType {
    statements;
}

fn suma(a: i32, b: i32) {
    let c = a + b;
    println!(“La suma de {} + {} es: {}”, a, b, c);
}*/