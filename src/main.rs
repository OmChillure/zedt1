use std::collections::HashMap;
use std::fmt;

// --- Enums ---

#[derive(Debug, Clone, PartialEq)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Rectangle(w, h) => w * h,
            Shape::Triangle(a, b, c) => {
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }

    fn perimeter(&self) -> f64 {
        match self {
            Shape::Circle(r) => 2.0 * std::f64::consts::PI * r,
            Shape::Rectangle(w, h) => 2.0 * (w + h),
            Shape::Triangle(a, b, c) => a + b + c,
        }
    }
}

impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Shape::Circle(r) => write!(f, "Circle(radius={:.2})", r),
            Shape::Rectangle(w, h) => write!(f, "Rectangle({}x{})", w, h),
            Shape::Triangle(a, b, c) => write!(f, "Triangle({}, {}, {})", a, b, c),
        }
    }
}

// --- Structs ---

#[derive(Debug, Clone)]
struct Student {
    name: String,
    grades: Vec<f64>,
}

impl Student {
    fn new(name: &str, grades: Vec<f64>) -> Self {
        Student {
            name: name.to_string(),
            grades,
        }
    }

    fn average(&self) -> f64 {
        if self.grades.is_empty() {
            return 0.0;
        }
        self.grades.iter().sum::<f64>() / self.grades.len() as f64
    }

    fn highest(&self) -> Option<f64> {
        self.grades.iter().cloned().reduce(f64::max)
    }

    fn lowest(&self) -> Option<f64> {
        self.grades.iter().cloned().reduce(f64::min)
    }
}

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Student({}, avg={:.2})", self.name, self.average())
    }
}

// --- Traits ---

trait Describable {
    fn describe(&self) -> String;
}

impl Describable for Shape {
    fn describe(&self) -> String {
        format!(
            "{} | area={:.2}, perimeter={:.2}",
            self,
            self.area(),
            self.perimeter()
        )
    }
}

impl Describable for Student {
    fn describe(&self) -> String {
        format!(
            "{} | high={:.2}, low={:.2}",
            self,
            self.highest().unwrap_or(0.0),
            self.lowest().unwrap_or(0.0)
        )
    }
}

// --- Generic Function ---

fn print_descriptions<T: Describable>(items: &[T]) {
    for item in items {
        println!("  {}", item.describe());
    }
}

// --- Fibonacci Iterator ---

struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { a: 0, b: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.a + self.b;
        self.a = self.b;
        self.b = next;
        Some(self.a)
    }
}

// --- Error Handling ---

#[derive(Debug)]
enum AppError {
    DivisionByZero,
    NegativeInput(f64),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DivisionByZero => write!(f, "Error: division by zero"),
            AppError::NegativeInput(v) => write!(f, "Error: negative input {}", v),
        }
    }
}

fn safe_sqrt(x: f64) -> Result<f64, AppError> {
    if x < 0.0 {
        Err(AppError::NegativeInput(x))
    } else {
        Ok(x.sqrt())
    }
}

fn safe_divide(a: f64, b: f64) -> Result<f64, AppError> {
    if b == 0.0 {
        Err(AppError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

// --- Word Frequency Counter ---

fn word_frequency(text: &str) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let clean: String = word.chars().filter(|c| c.is_alphabetic()).collect();
        if !clean.is_empty() {
            *map.entry(clean.to_lowercase()).or_insert(0) += 1;
        }
    }
    map
}

// --- main ---

fn main() {
    // Shapes
    println!("=== Shapes ===");
    let shapes = vec![
        Shape::Circle(5.0),
        Shape::Rectangle(4.0, 6.0),
        Shape::Triangle(3.0, 4.0, 5.0),
    ];
    print_descriptions(&shapes);

    // Students
    println!("\n=== Students ===");
    let students = vec![
        Student::new("Alice", vec![92.0, 88.0, 95.0, 79.0]),
        Student::new("Bob", vec![70.0, 85.0, 60.0, 90.0]),
        Student::new("Carol", vec![100.0, 98.0, 97.0, 99.0]),
    ];
    print_descriptions(&students);

    // Fibonacci
    println!("\n=== Fibonacci (first 10) ===");
    let fibs: Vec<u64> = Fibonacci::new().take(10).collect();
    println!("  {:?}", fibs);

    // Iterators & functional style
    println!("\n=== Even squares up to 10 ===");
    let even_squares: Vec<u32> = (1..=10).filter(|x| x % 2 == 0).map(|x| x * x).collect();
    println!("  {:?}", even_squares);

    // Error handling
    println!("\n=== Error Handling ===");
    for val in [16.0, -4.0, 0.0_f64] {
        match safe_sqrt(val) {
            Ok(r) => println!("  sqrt({}) = {:.4}", val, r),
            Err(e) => println!("  sqrt({}) -> {}", val, e),
        }
    }
    for (a, b) in [(10.0, 2.0), (5.0, 0.0)] {
        match safe_divide(a, b) {
            Ok(r) => println!("  {}/{} = {:.4}", a, b, r),
            Err(e) => println!("  {}/{} -> {}", a, b, e),
        }
    }

    // Word frequency
    println!("\n=== Word Frequency ===");
    let text = "the quick brown fox jumps over the lazy dog the fox";
    let freq = word_frequency(text);
    let mut freq_sorted: Vec<_> = freq.iter().collect();
    freq_sorted.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    for (word, count) in freq_sorted.iter().take(5) {
        println!("  {:>10} : {}", word, count);
    }

    // Closures & higher-order functions
    println!("\n=== Temperature Conversion ===");
    let celsius_temps = vec![0.0, 20.0, 37.0, 100.0];
    let to_fahrenheit = |c: f64| c * 9.0 / 5.0 + 32.0;
    let fahrenheit_temps: Vec<f64> = celsius_temps.iter().map(|&c| to_fahrenheit(c)).collect();
    for (c, f) in celsius_temps.iter().zip(fahrenheit_temps.iter()) {
        println!("  {:.1}°C = {:.1}°F", c, f);
    }

    // HashMap usage
    println!("\n=== Capital Cities ===");
    let mut capitals: HashMap<&str, &str> = HashMap::new();
    capitals.insert("France", "Paris");
    capitals.insert("Japan", "Tokyo");
    capitals.insert("Brazil", "Brasília");
    capitals.insert("India", "New Delhi");
    let mut entries: Vec<_> = capitals.iter().collect();
    entries.sort_by_key(|(k, _)| *k);
    for (country, capital) in entries {
        println!("  {} -> {}", country, capital);
    }

    println!("\nDone!");
}
