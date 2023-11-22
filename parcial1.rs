
// INTEGRANTES: SOFIA MIERES, IGNACIO MORALES Y ALEXIS HERNANDEZ

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

fn loop_rango() -> i32{
    loop{
        println!("tienes para escoger 4 numeros. \nn1: pregunta 1.\nn2: pregunta 2.\nn3: pregunta 3.\nn4: pregunta 4.\n ");
        let numero = utiles::texto_numero("pregunta".to_string());
        if numero == 1 || numero == 2 || numero == 3 || numero == 4 {
            print_tipo_columna(numero);
            return numero;
        }
    }
}

fn print_tipo_columna(numero:i32) -> (){
    match numero{
        1 => println!("La cantidad de pokemon de tipo agua que hay en la base de datos es: "),
        2 => println!("La cantidad de legendarios que hay dentro de la base de datos es: "),
        3 => println!("Entrega toda la informacion del pokemon"),
        4 => println!("Cantidad de pokemon de 6ta generacion y que sean tipo normal: "),
        _ => (),
    }

}


fn pregunta1() -> () {
    let mut contador_lineas = 0;
    let mut acum = 0;
    //PARA ENTREGA, CAMBIAR EL NOMBRE DEL ARCHIVOOOOOOOO
    if let Ok(lines) = read_lines("./Pokemon.csv") {

        for line in lines {

            if let Ok(ip) = line {
                let ip_copy = ip.clone();
                let split = ip_copy.split(",");
            //  QUEDA DE LA FORMA ***   ****   **  * *** * 
                let mut contador_columnas:u32 = 0;

                for s in split {
                    //CUERPO DEL EJERCICIO
                    if contador_columnas == 2  && contador_lineas >= 1 {
                        let s = s.to_lowercase();
                        if "water" == s{
                            // println!("{}",s);
                            acum += 1;
                        }
                    }
                    if contador_columnas == 3 && contador_lineas >= 1 {
                        // println!("entra a posicion 2 {}",s);
                        if "water" == s.trim().to_lowercase(){
                            acum += 1;
                        }
                    }
                    contador_columnas += 1;

                }
                
            }

            contador_lineas += 1;
        }
    }
//INGRESAR ACUMULADOR AQUI DENTRO
    println!("el acumulador vale {}", acum );

}
 
fn pregunta2() -> () {
    let mut contador_lineas = 0;
    let mut acumulador = 0;
    //PARA ENTREGA, CAMBIAR EL NOMBRE DEL ARCHIVOOOOOOOO
    if let Ok(lines) = read_lines("./Pokemon.csv") {

        for line in lines {
        
            if let Ok(ip) = line {
                let ip_copy = ip.clone();
                let split = ip_copy.split(",");
            //  QUEDA DE LA FORMA ***   ****   **  * *** * 
                let mut contador_columnas:u32 = 0;

                for s in split {
                    //CUERPO DEL EJERCICIO
                    if contador_columnas == 12 && contador_lineas >= 1 {
                        let s = s.to_lowercase();
                        // println!("entra {}",s);
                        if "true" == s{
                            acumulador += 1;
                        } 
                    }
                    contador_columnas += 1;

                }
                
            }

            contador_lineas += 1;
        }
    }
//INGRESAR ACUMULADOR AQUI DENTRO
    println!("El acumulador es: {}",acumulador );
}

fn pregunta3() -> () {
        //esta variable cambiara a true si se encuentra el pokemon que ingreso el usuario
        //si lo ingreso mal o este no esta en la base, se le pedira denuevo
    let mut control_pikashu:bool= false;

    loop 
    {
        let nombre_pokemon = utiles::ingreso_texto("Ingresa el nombre del pokemon a buscar".to_string());
        if let Ok(lines) = read_lines("./Pokemon.csv") {

            for line in lines {
                //en este arreglo se alojara la informacion de el pokemon
                let mut informacion_pokemon: [String;14] = 
                [String::new(),String::new(),String::new(),String::new(),String::new(),String::new(),
                String::new(),String::new(),String::new(),String::new(),String::new(),String::new(),
                String::new(),String::new()];
                if let Ok(ip) = line {
                    let ip_copy = ip.clone();
                    let split = ip_copy.split(",");
                //  QUEDA DE LA FORMA ***   ****   **  * *** * 

                //este contador sirve para dar la posicion en el arreglo
                    let mut contador_columnas:usize = 0;

                    for elemento in split {
                        //eki se va poniendo toda la info de el arreglo en cada posicion
                        informacion_pokemon[contador_columnas] = elemento.to_string().to_lowercase();
                        

                        contador_columnas += 1;
                    }

                    //si el nombre que ingreso el usuario es == con el string q esta en la posicion 1 del arreglo
                    //donde deberia de estar el nombre de el pokemon actual, se imprimira toda su infomracion
                    if nombre_pokemon == informacion_pokemon[1]{
                        println!("\n~~Se encontro a su Pokemon~~\n");
                        println!("#{:?}\nNombre~ {:?}\nType1~ {:?}\nType2~ {:?}\nTotal~ {:?}\nHp~ {:?}\nAttack~ {:?}\nDefense~ {:?}\nSp. Attack~ {:?}\nSp. Defense~ {:?}\nSpeed~ {:?}\nGeneration~ {:?}\nLegendary~ {:?}", informacion_pokemon[0], informacion_pokemon[1], informacion_pokemon[2], informacion_pokemon[3], informacion_pokemon[4], informacion_pokemon[5], informacion_pokemon[6], informacion_pokemon[7], informacion_pokemon[8], informacion_pokemon[9], informacion_pokemon[10], informacion_pokemon[11], informacion_pokemon[12]);

                        //si se encontro la variable control pasa a tru
                        control_pikashu = true;
                    }
                }
            }

            //si se encontro el pokemon que se buscaba se para la busqueda
            if control_pikashu{
                break;
            } else {
                //si se recorrio todo el arreglo y no se encontro al pokemon se le vuelve a pedir al usuario que ingrese un nombre valido
                println!("No se encontro el pokemon en el dataset, Porfavor ingrese otro denuevo");

            }
        }
    }
}

fn pregunta4() -> () {
    let mut cum = 0;
    let mut verificador:bool = false;
    //PARA ENTREGA, CAMBIAR EL NOMBRE DEL ARCHIVOOOOOOOO
    if let Ok(lines) = read_lines("./Pokemon.csv") {

        for line in lines {
            // println!("CAMBIO DE LINEA \n");

            if let Ok(ip) = line {
                let ip_copy = ip.clone();
                let split = ip_copy.split(",");
            //  QUEDA DE LA FORMA ***   ****   **  * *** * 
                let mut contador_columnas:u32 = 0;

                for s in split {
                    //CUERPO DEL EJERCICIO
                    if contador_columnas == 2{
                        // println!("reviso primer tipo");
                        let s  = s.trim().to_string();
                        if s == "Normal" {
                            // println!("es verda, son iguales");
                            verificador = true;
                        } else {
                            // println!("es mentira, no son iguales");
                        }
                    }

                    if contador_columnas == 3{
                        // println!("reviso segundo tipo");
                        let s  = s.trim().to_string();
                        if s == "Normal" {
                            // println!("es verda, son iguales");
                            verificador = true;
                        } else {
                            // println!("es mentira, no son iguales");
                        }
                    }

                    if contador_columnas == 11 && verificador == true{

                        let s:u32 = match s.parse(){
                            Ok(s)=>s,
                            Err(_)=>continue,
                        };

                        let generacion_pokemon = 6;
                        
                        if s == generacion_pokemon{
                            // println!("Pertenece a la 6ma generación y es de tipo normal\n");
                            cum += 1; 
                        }
                        verificador = false
                    }
                    
                    contador_columnas += 1;

                }
                
            }

        }
    }
    //INGRESAR ACUMULADOR AQUI DENTRO
    println!("La cantidad de pokemon son: {}", cum);


}
 



fn todo_proceso() -> () {
    loop {
        let pregunta = loop_rango();
        if pregunta == 1{
            pregunta1();
        }
        if pregunta == 2{
            pregunta2();
        }

        if pregunta == 3{
            pregunta3();
        }
        if pregunta == 4{
            pregunta4();
        }
        println!("\nDesea salir del programa?");
        if utiles::si_no(){
            break;
        }
    }
}




fn main() {
    todo_proceso();
}