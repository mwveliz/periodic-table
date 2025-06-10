use std::fs;

fn main() {
    let elements = [
        ("H", "Hydrogen", 1, 1),   ("He", "Helium", 18, 1),
        ("Li", "Lithium", 1, 2),  ("Be", "Beryllium", 2, 2), ("B", "Boron", 13, 2),  ("C", "Carbon", 14, 2),  ("N", "Nitrogen", 15, 2), ("O", "Oxygen", 16, 2),  ("F", "Fluorine", 17, 2), ("Ne", "Neon", 18, 2),
        ("Na", "Sodium", 1, 3),   ("Mg", "Magnesium", 2, 3), ("Al", "Aluminum", 13, 3),("Si", "Silicon", 14, 3), ("P", "Phosphorus", 15, 3),("S", "Sulfur", 16, 3), ("Cl", "Chlorine", 17, 3),("Ar", "Argon", 18, 3),
        ("K", "Potassium", 1, 4),("Ca", "Calcium", 2, 4),  ("Sc", "Scandium", 3, 4), ("Ti", "Titanium", 4, 4),("V", "Vanadium", 5, 4),  ("Cr", "Chromium", 6, 4), ("Mn", "Manganese", 7, 4),("Fe", "Iron", 8, 4),    ("Co", "Cobalt", 9, 4),  ("Ni", "Nickel", 10, 4), ("Cu", "Copper", 11, 4), ("Zn", "Zinc", 12, 4),  ("Ga", "Gallium", 13, 4), ("Ge", "Germanium", 14, 4),("As", "Arsenic", 15, 4),("Se", "Selenium", 16, 4),("Br", "Bromine", 17, 4),("Kr", "Krypton", 18, 4),
        ("Rb", "Rubidium", 1, 5), ("Sr", "Strontium", 2, 5), ("Y", "Yttrium", 3, 5),  ("Zr", "Zirconium", 4, 5),("Nb", "Niobium", 5, 5), ("Mo", "Molybdenum", 6, 5),("Tc", "Technetium", 7, 5),("Ru", "Ruthenium", 8, 5),("Rh", "Rhodium", 9, 5), ("Pd", "Palladium", 10, 5),("Ag", "Silver", 11, 5),  ("Cd", "Cadmium", 12, 5), ("In", "Indium", 13, 5), ("Sn", "Tin", 14, 5),    ("Sb", "Antimony", 15, 5),("Te", "Tellurium", 16, 5),("I", "Iodine", 17, 5),  ("Xe", "Xenon", 18, 5),
        ("Cs", "Cesium", 1, 6),   ("Ba", "Barium", 2, 6),  ("La", "Lanthanum", 3, 6), ("Ce", "Cerium", -1, 9),  ("Pr", "Praseodymium", -1, 9),("Nd", "Neodymium", -1, 9), ("Pm", "Promethium", -1, 9),("Sm", "Samarium", -1, 9),("Eu", "Europium", -1, 9),("Gd", "Gadolinium", -1, 9),("Tb", "Terbium", -1, 9),("Dy", "Dysprosium", -1, 9),("Ho", "Holmium", -1, 9),  ("Er", "Erbium", -1, 9),  ("Tm", "Thulium", -1, 9),  ("Yb", "Ytterbium", -1, 9),("Lu", "Lutetium", 18, 6),
        ("Fr", "Francium", 1, 7), ("Ra", "Radium", 2, 7),  ("Ac", "Actinium", 3, 7), ("Th", "Thorium", -1, 10),("Pa", "Protactinium", -1, 10),("U", "Uranium", -1, 10),  ("Np", "Neptunium", -1, 10),("Pu", "Plutonium", -1, 10),("Am", "Americium", -1, 10),("Cm", "Curium", -1, 10),   ("Bk", "Berkelium", -1, 10),("Cf", "Californium", -1, 10),("Es", "Einsteinium", -1, 10),("Fm", "Fermium", -1, 10),  ("Md", "Mendelevium", -1, 10),("No", "Nobelium", -1, 10), ("Lr", "Lawrencium", 18, 7),
        ("Rf", "Rutherfordium", 4, 7),("Db", "Dubnium", 5, 7), ("Sg", "Seaborgium", 6, 7),("Bh", "Bohrium", 7, 7),  ("Hs", "Hassium", 8, 7),    ("Mt", "Meitnerium", 9, 7), ("Ds", "Darmstadtium", 10, 7),("Rg", "Roentgenium", 11, 7),("Cn", "Copernicium", 12, 7),("Nh", "Nihonium", 13, 7),  ("Fl", "Flerovium", 14, 7), ("Mc", "Moscovium", 15, 7), ("Lv", "Livermorium", 16, 7),("Ts", "Tennessine", 17, 7),("Og", "Oganesson", 18, 7),
    ];

    let lanthanides = [
        ("La", "Lanthanum"), ("Ce", "Cerium"), ("Pr", "Praseodymium"), ("Nd", "Neodymium"), ("Pm", "Promethium"), ("Sm", "Samarium"), ("Eu", "Europium"), ("Gd", "Gadolinium"), ("Tb", "Terbium"), ("Dy", "Dysprosium"), ("Ho", "Holmium"), ("Er", "Erbium"), ("Tm", "Thulium"), ("Yb", "Ytterbium"), ("Lu", "Lutetium"),
    ];

    let actinides = [
        ("Ac", "Actinium"), ("Th", "Thorium"), ("Pa", "Protactinium"), ("U", "Uranium"), ("Np", "Neptunium"), ("Pu", "Plutonium"), ("Am", "Americium"), ("Cm", "Curium"), ("Bk", "Berkelium"), ("Cf", "Californium"), ("Es", "Einsteinium"), ("Fm", "Fermium"), ("Md", "Mendelevium"), ("No", "Nobelium"), ("Lr", "Lawrencium"),
    ];

    let html_content = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Periodic Table</title>
    <style>
        body {{
            font-family: sans-serif;
            display: flex;
            justify-content: center;
            align-items: center;
            min-height: 100vh;
            background-color: #f4f4f4;
            margin: 0;
        }}
        .periodic-table {{
            display: grid;
            grid-template-columns: repeat(18, 1fr);
            grid-gap: 5px;
            padding: 20px;
            background-color: #fff;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
        }}
        .element {{
            background-color: #e0e0e0;
            border: 1px solid #ccc;
            padding: 10px;
            text-align: center;
            border-radius: 4px;
            cursor: pointer;
            transition: transform 0.2s ease-in-out;
        }}
        .element:hover {{
            transform: scale(1.1);
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
        }}
        .element strong {{
            display: block;
            font-size: 1.2em;
        }}
        .lanthanides-actinides-wrapper {{
            display: flex;
            flex-direction: column;
            align-items: center;
            margin-top: 20px;
        }}
        .lanthanides, .actinides {{
            display: grid;
            grid-template-columns: repeat(15, 1fr);
            grid-gap: 5px;
            margin-top: 5px;
        }}
        /* Specific positions for elements */
        {}
    </style>
</head>
<body>
    <div class="periodic-table">
        {}
    </div>
    <div class="lanthanides-actinides-wrapper">
        <div class="lanthanides">
            {}
        </div>
        <div class="actinides">
            {}
        </div>
    </div>
</body>
</html>
"#,
        generate_specific_positions(&elements),
        generate_elements_html(&elements),
        generate_elements_html_secondary(&lanthanides),
        generate_elements_html_secondary(&actinides),
    );

    fs::write("index.html", html_content)
        .expect("Unable to write to file");

    println!("index.html file generated successfully!");
}

fn generate_elements_html(elements: &[(&str, &str, i32, i32)]) -> String {
    let mut html = String::new();
    for &(symbol, name, group, period) in elements {
        if group > 0 {
            html.push_str(&format!(r#"<div class="element" style="grid-column: {}; grid-row: {};"><strong title="{}">{}</strong></div>"#, group, period, name, symbol));
        }
    }
    html
}

fn generate_elements_html_secondary(elements: &[(&str, &str)]) -> String {
    let mut html = String::new();
    for &(symbol, name) in elements {
        html.push_str(&format!(r#"<div class="element"><strong title="{}">{}</strong></div>"#, name, symbol));
    }
    html
}

fn generate_specific_positions(elements: &[(&str, &str, i32, i32)]) -> String {
    let mut css = String::new();
    for &(symbol, _, group, period) in elements {
        if group == -1 {
            let grid_column_start = match symbol {
                "La" | "Ac" => 3,
                "Ce" | "Th" => 4,
                "Pr" | "Pa" => 5,
                "Nd" | "U" => 6,
                "Pm" | "Np" => 7,
                "Sm" | "Pu" => 8,
                "Eu" | "Am" => 9,
                "Gd" | "Cm" => 10,
                "Tb" | "Bk" => 11,
                "Dy" | "Cf" => 12,
                "Ho" | "Es" => 13,
                "Er" | "Fm" => 14,
                "Tm" | "Md" => 15,
                "Yb" | "No" => 16,
                "Lu" | "Lr" => 17,
                _ => 1,
            };
            let grid_row = if symbol == "La" || symbol == "Ce" || symbol == "Pr" || symbol == "Nd" || symbol == "Pm" || symbol == "Sm" || symbol == "Eu" || symbol == "Gd" || symbol == "Tb" || symbol == "Dy" || symbol == "Ho" || symbol == "Er" || symbol == "Tm" || symbol == "Yb" || symbol == "Lu" {
                6
            } else {
                7
            };
            css.push_str(&format!(r#".periodic-table .element:contains("{}") {{ grid-column: {} !important; grid-row: {} !important; }}"#, symbol, grid_column_start, grid_row));
        }
    }
    css
}
