use std::io::BufReader;
use std::io::{self, BufRead};
use std::path::Path;
use std::u32;
use std::fs::File;

mod utiles;

//USAR UNA MUESTRA, SI FUNCIONA CON ELLO, FUNCIONA CON MILES DE DATOS

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn loop_rango() -> u32{
    loop{
        println!("tienes para escoger 4 numeros. \nn1: .\nn2: .\nn3: .\nn4: .\n ");
        let numero = utiles::texto_numero("pregunta".to_string());
        if numero == 1 || numero == 2 || numero == 3 || numero == 4 {
            print_tipo_columna(numero);
            return numero;
        }
    }
}

fn print_tipo_columna(numero:u32) -> (){
    match numero{
        1 => println!("La cantidad de pokemon de tipo agua que hay en la base de datos es: "),
        2 => println!("La cantidad de legendarios que hay dentro de la base de datos es: "),
        3 => println!("NOSE Q PONER PERO ES LA PREGUNTA 3"),
        4 => println!("Cantidad de pokemon de 7ma generacion: "),
        _ => (),
    }

}


fn pregunta1() -> () {
    let mut contador_lineas = 0;
    //PARA ENTREGA, CAMBIAR EL NOMBRE DEL ARCHIVOOOOOOOO
    if let Ok(lines) = read_lines("./data_test.txt") {

        for line in lines {
            let mut arreglo_provisional: [String;3] = [String::new(),String::new(),String::new()];

            if let Ok(ip) = line {
                let ip_copy = ip.clone();
                let split = ip_copy.split(",");
            //  QUEDA DE LA FORMA ***   ****   **  * *** * 
                let mut contador_columnas:u32 = 0;

                for s in split {
                    //CUERPO DEL EJERCICIO

                    contador_columnas += 1;

                }
                
            }

            contador_lineas += 1;
        }
    }
//INGRESAR ACUMULADOR AQUI DENTRO
    println!("{:?}", );

}

fn pregunta2() -> () {

    let mut contador_lineas = 0;
    //PARA ENTREGA, CAMBIAR EL NOMBRE DEL ARCHIVOOOOOOOO
    if let Ok(lines) = read_lines("./data_test.txt") {

        for line in lines {
            let mut arreglo_provisional: [String;3] = [String::new(),String::new(),String::new()];

            if let Ok(ip) = line {
                let ip_copy = ip.clone();
                let split = ip_copy.split(",");
            //  QUEDA DE LA FORMA ***   ****   **  * *** * 
                let mut contador_columnas:u32 = 0;

                for s in split {
                    //CUERPO DEL EJERCICIO

                    contador_columnas += 1;

                }
                
            }

            contador_lineas += 1;
        }
    }
//INGRESAR ACUMULADOR AQUI DENTRO
    println!("{:?}", );
}

fn pregunta3() -> {

    let mut contador_lineas = 0;
    //PARA ENTREGA, CAMBIAR EL NOMBRE DEL ARCHIVOOOOOOOO
    if let Ok(lines) = read_lines("./data_test.txt") {

        for line in lines {
            let mut arreglo_provisional: [String;3] = [String::new(),String::new(),String::new()];

            if let Ok(ip) = line {
                let ip_copy = ip.clone();
                let split = ip_copy.split(",");
            //  QUEDA DE LA FORMA ***   ****   **  * *** * 
                let mut contador_columnas:u32 = 0;

                for s in split {
                    //CUERPO DEL EJERCICIO

                    contador_columnas += 1;

                }
                
            }

            contador_lineas += 1;
        }
    }
//INGRESAR ACUMULADOR AQUI DENTRO
    println!("{:?}", );

}

fn pregunta4() -> () {
    let mut contador_lineas = 0;
    //PARA ENTREGA, CAMBIAR EL NOMBRE DEL ARCHIVOOOOOOOO
    if let Ok(lines) = read_lines("./data_test.txt") {

        for line in lines {
            let mut arreglo_provisional: [String;3] = [String::new(),String::new(),String::new()];

            if let Ok(ip) = line {
                let ip_copy = ip.clone();
                let split = ip_copy.split(",");
            //  QUEDA DE LA FORMA ***   ****   **  * *** * 
                let mut contador_columnas:u32 = 0;

                for s in split {
                    //CUERPO DEL EJERCICIO

                    contador_columnas += 1;

                }
                
            }

            contador_lineas += 1;
        }
    }
//INGRESAR ACUMULADOR AQUI DENTRO
    println!("{:?}", );


}


fn todo_proceso() -> () {
    let mut num_max: u32 = 0;

    let pregunta = loop_rango();
    if pregunta == 1{
        pregunta1();
        return;
    }
    if pregunta == 2{
        pregunta2();
        return;
    }
    if pregunta == 3{
        pregunta3();
        return;
    }
    if pregunta == 4{
        pregunta4();
        return;
    }
}



fn main() {
    todo_proceso();
}