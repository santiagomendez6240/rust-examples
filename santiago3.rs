trait Empleado {
    fn calcular_aguinaldo(&self) -> f64;
    fn obtener_nombre(&self) -> String;
    fn obtener_salario(&self) -> f64;
    fn descripcion(&self) -> String;
}

struct Persona {
    nombre: String,
    edad: u32,
}

struct TrabajadorBase {
    persona: Persona,
    salario: f64,
    departamento: String,
    meses_trabajados: u32,
}

struct Gerente {
    persona: Persona,
    salario: f64,
    departamento: String,
    meses_trabajados: u32,
    bono_gerencia: f64,
}

struct Nomina {
    empleados: Vec<Box<dyn Empleado>>,
}

impl Persona {
    fn new(nombre: String, edad: u32) -> Self {
        Persona { nombre, edad }
    }
}

impl TrabajadorBase {
    fn new(persona: Persona, salario: f64, departamento: String) -> Self {
        TrabajadorBase {
            persona,
            salario,
            departamento,
            meses_trabajados: 0,
        }
    }
    
    fn agregar_meses_trabajo(&mut self, meses: u32) {
        self.meses_trabajados += meses;
    }
}

impl Gerente {
    fn new(persona: Persona, salario: f64, departamento: String, bono_gerencia: f64) -> Self {
        Gerente {
            persona,
            salario,
            departamento,
            meses_trabajados: 0,
            bono_gerencia,
        }
    }
    
    fn agregar_meses_trabajo(&mut self, meses: u32) {
        self.meses_trabajados += meses;
    }
}

impl Empleado for TrabajadorBase {
    fn calcular_aguinaldo(&self) -> f64 {
        let meses = if self.meses_trabajados > 12 {
            12
        } else {
            self.meses_trabajados
        };
        (self.salario * meses as f64) / 12.0
    }
    
    fn obtener_nombre(&self) -> String {
        self.persona.nombre.clone()
    }
    
    fn obtener_salario(&self) -> f64 {
        self.salario
    }
    
    fn descripcion(&self) -> String {
        format!(
            "Trabajador Base - {} - Departamento: {} - Salario: ${:.2}",
            self.persona.nombre, self.departamento, self.salario
        )
    }
}

impl Empleado for Gerente {
    fn calcular_aguinaldo(&self) -> f64 {
        let meses = if self.meses_trabajados > 12 {
            12
        } else {
            self.meses_trabajados
        };
        ((self.salario + self.bono_gerencia) * meses as f64) / 12.0
    }
    
    fn obtener_nombre(&self) -> String {
        self.persona.nombre.clone()
    }
    
    fn obtener_salario(&self) -> f64 {
        self.salario + self.bono_gerencia
    }
    
    fn descripcion(&self) -> String {
        format!(
            "Gerente - {} - Departamento: {} - Salario: ${:.2} + Bono: ${:.2}",
            self.persona.nombre, self.departamento, self.salario, self.bono_gerencia
        )
    }
}

impl Nomina {
    fn new() -> Self {
        Nomina {
            empleados: Vec::new(),
        }
    }
    
    fn agregar_empleado(&mut self, empleado: Box<dyn Empleado>) {
        self.empleados.push(empleado);
    }
    
    fn calcular_aguinaldo_total(&self) -> f64 {
        self.empleados.iter().map(|emp| emp.calcular_aguinaldo()).sum()
    }
    
    fn mostrar_nomina(&self) {
        println!("=== NÓMINA DE EMPLEADOS ===");
        for empleado in &self.empleados {
            println!("{}", empleado.descripcion());
            println!("Aguinaldo: ${:.2}\n", empleado.calcular_aguinaldo());
        }
        println!("Aguinaldo total a pagar: ${:.2}", self.calcular_aguinaldo_total());
    }
}

fn main() {
    let mut nomina = Nomina::new();
    
    // Crear trabajadores base
    let persona1 = Persona::new(String::from("Carlos López"), 28);
    let mut trabajador1 = TrabajadorBase::new(persona1, 25000.0, String::from("TI"));
    trabajador1.agregar_meses_trabajo(10);
    
    let persona2 = Persona::new(String::from("Ana Martínez"), 32);
    let mut trabajador2 = TrabajadorBase::new(persona2, 28000.0, String::from("Ventas"));
    trabajador2.agregar_meses_trabajo(12);
    
    // Crear gerentes
    let persona3 = Persona::new(String::from("Roberto Díaz"), 45);
    let mut gerente1 = Gerente::new(persona3, 50000.0, String::from("TI"), 10000.0);
    gerente1.agregar_meses_trabajo(12);
    
    let persona4 = Persona::new(String::from("Laura Sánchez"), 38);
    let mut gerente2 = Gerente::new(persona4, 48000.0, String::from("Finanzas"), 8000.0);
    gerente2.agregar_meses_trabajo(8);
    
    // Agregar empleados a la nómina
    nomina.agregar_empleado(Box::new(trabajador1));
    nomina.agregar_empleado(Box::new(trabajador2));
    nomina.agregar_empleado(Box::new(gerente1));
    nomina.agregar_empleado(Box::new(gerente2));
    
    // Mostrar resultados
    nomina.mostrar_nomina();
}