use std::fs;

fn main() {
    let elements = [
        ("H", "Hydrogen", 1, 1, "nonmetal"),   ("He", "Helium", 18, 1, "noble-gas"),
        ("Li", "Lithium", 1, 2, "alkali-metal"),  ("Be", "Beryllium", 2, 2, "alkaline-earth-metal"), ("B", "Boron", 13, 2, "metalloid"),  ("C", "Carbon", 14, 2, "nonmetal"),  ("N", "Nitrogen", 15, 2, "nonmetal"), ("O", "Oxygen", 16, 2, "nonmetal"),  ("F", "Fluorine", 17, 2, "halogen"), ("Ne", "Neon", 18, 2, "noble-gas"),
        ("Na", "Sodium", 1, 3, "alkali-metal"),   ("Mg", "Magnesium", 2, 3, "alkaline-earth-metal"), ("Al", "Aluminum", 13, 3, "metal"),("Si", "Silicon", 14, 3, "metalloid"), ("P", "Phosphorus", 15, 3, "nonmetal"),("S", "Sulfur", 16, 3, "nonmetal"), ("Cl", "Chlorine", 17, 3, "halogen"),("Ar", "Argon", 18, 3, "noble-gas"),
        ("K", "Potassium", 1, 4, "alkali-metal"),("Ca", "Calcium", 2, 4, "alkaline-earth-metal"),  ("Sc", "Scandium", 3, 4, "transition-metal"), ("Ti", "Titanium", 4, 4, "transition-metal"),("V", "Vanadium", 5, 4, "transition-metal"),  ("Cr", "Chromium", 6, 4, "transition-metal"), ("Mn", "Manganese", 7, 4, "transition-metal"),("Fe", "Iron", 8, 4, "transition-metal"),    ("Co", "Cobalt", 9, 4, "transition-metal"),  ("Ni", "Nickel", 10, 4, "transition-metal"), ("Cu", "Copper", 11, 4, "transition-metal"), ("Zn", "Zinc", 12, 4, "transition-metal"),  ("Ga", "Gallium", 13, 4, "metal"), ("Ge", "Germanium", 14, 4, "metalloid"),("As", "Arsenic", 15, 4, "metalloid"),("Se", "Selenium", 16, 4, "nonmetal"),("Br", "Bromine", 17, 4, "halogen"),("Kr", "Krypton", 18, 4, "noble-gas"),
        ("Rb", "Rubidium", 1, 5, "alkali-metal"), ("Sr", "Strontium", 2, 5, "alkaline-earth-metal"), ("Y", "Yttrium", 3, 5, "transition-metal"),  ("Zr", "Zirconium", 4, 5, "transition-metal"),("Nb", "Niobium", 5, 5, "transition-metal"), ("Mo", "Molybdenum", 6, 5, "transition-metal"),("Tc", "Technetium", 7, 5, "transition-metal"),("Ru", "Ruthenium", 8, 5, "transition-metal"),("Rh", "Rhodium", 9, 5, "transition-metal"), ("Pd", "Palladium", 10, 5, "transition-metal"),("Ag", "Silver", 11, 5, "transition-metal"),  ("Cd", "Cadmium", 12, 5, "transition-metal"), ("In", "Indium", 13, 5, "metal"), ("Sn", "Tin", 14, 5, "metal"),    ("Sb", "Antimony", 15, 5, "metalloid"),("Te", "Tellurium", 16, 5, "metalloid"),("I", "Iodine", 17, 5, "halogen"),  ("Xe", "Xenon", 18, 5, "noble-gas"),
        ("Cs", "Cesium", 1, 6, "alkali-metal"),   ("Ba", "Barium", 2, 6, "alkaline-earth-metal"),  ("La", "Lanthanum", 3, 6, "lanthanide"), ("Ce", "Cerium", -1, 9, "lanthanide"),  ("Pr", "Praseodymium", -1, 9, "lanthanide"),("Nd", "Neodymium", -1, 9, "lanthanide"), ("Pm", "Promethium", -1, 9, "lanthanide"),("Sm", "Samarium", -1, 9, "lanthanide"),("Eu", "Europium", -1, 9, "lanthanide"),("Gd", "Gadolinium", -1, 9, "lanthanide"),("Tb", "Terbium", -1, 9, "lanthanide"),("Dy", "Dysprosium", -1, 9, "lanthanide"),("Ho", "Holmium", -1, 9, "lanthanide"),  ("Er", "Erbium", -1, 9, "lanthanide"),  ("Tm", "Thulium", -1, 9, "lanthanide"),  ("Yb", "Ytterbium", -1, 9, "lanthanide"),("Lu", "Lutetium", 18, 6, "lanthanide"),
        ("Fr", "Francium", 1, 7, "alkali-metal"), ("Ra", "Radium", 2, 7, "alkaline-earth-metal"),  ("Ac", "Actinium", 3, 7, "actinide"), ("Th", "Thorium", -1, 10, "actinide"),("Pa", "Protactinium", -1, 10, "actinide"),("U", "Uranium", -1, 10, "actinide"),  ("Np", "Neptunium", -1, 10, "actinide"),("Pu", "Plutonium", -1, 10, "actinide"),("Am", "Americium", -1, 10, "actinide"),("Cm", "Curium", -1, 10, "actinide"),   ("Bk", "Berkelium", -1, 10, "actinide"),("Cf", "Californium", -1, 10, "actinide"),("Es", "Einsteinium", -1, 10, "actinide"),("Fm", "Fermium", -1, 10, "actinide"),  ("Md", "Mendelevium", -1, 10, "actinide"),("No", "Nobelium", -1, 10, "actinide"), ("Lr", "Lawrencium", 18, 7, "actinide"),
        ("Rf", "Rutherfordium", 4, 7, "transition-metal"),("Db", "Dubnium", 5, 7, "transition-metal"), ("Sg", "Seaborgium", 6, 7, "transition-metal"),("Bh", "Bohrium", 7, 7, "transition-metal"),  ("Hs", "Hassium", 8, 7, "transition-metal"),    ("Mt", "Meitnerium", 9, 7, "transition-metal"), ("Ds", "Darmstadtium", 10, 7, "transition-metal"),("Rg", "Roentgenium", 11, 7, "transition-metal"),("Cn", "Copernicium", 12, 7, "transition-metal"),("Nh", "Nihonium", 13, 7, "post-transition-metal"),  ("Fl", "Flerovium", 14, 7, "post-transition-metal"), ("Mc", "Moscovium", 15, 7, "post-transition-metal"), ("Lv", "Livermorium", 16, 7, "post-transition-metal"),("Ts", "Tennessine", 17, 7, "halogen"),("Og", "Oganesson", 18, 7, "noble-gas"),
    ];

    let lanthanides = [
        ("La", "Lanthanum", "lanthanide"), ("Ce", "Cerium", "lanthanide"), ("Pr", "Praseodymium", "lanthanide"), ("Nd", "Neodymium", "lanthanide"), ("Pm", "Promethium", "lanthanide"), ("Sm", "Samarium", "lanthanide"), ("Eu", "Europium", "lanthanide"), ("Gd", "Gadolinium", "lanthanide"), ("Tb", "Terbium", "lanthanide"), ("Dy", "Dysprosium", "lanthanide"), ("Ho", "Holmium", "lanthanide"), ("Er", "Erbium", "lanthanide"), ("Tm", "Thulium", "lanthanide"), ("Yb", "Ytterbium", "lanthanide"), ("Lu", "Lutetium", "lanthanide"),
    ];

    let actinides = [
        ("Ac", "Actinium", "actinide"), ("Th", "Thorium", "actinide"), ("Pa", "Protactinium", "actinide"), ("U", "Uranium", "actinide"), ("Np", "Neptunium", "actinide"), ("Pu", "Plutonium", "actinide"), ("Am", "Americium", "actinide"), ("Cm", "Curium", "actinide"), ("Bk", "Berkelium", "actinide"), ("Cf", "Californium", "actinide"), ("Es", "Einsteinium", "actinide"), ("Fm", "Fermium", "actinide"), ("Md", "Mendelevium", "actinide"), ("No", "Nobelium", "actinide"), ("Lr", "Lawrencium", "actinide"),
    ];

    let html_content = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Stunning Periodic Table</title>
    <style>
        body {{
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            display: flex;
            justify-content: center;
            align-items: center;
            min-height: 100vh;
            background: linear-gradient(135deg, #f0f0f0, #e0e0e0);
            margin: 0;
        }}
        .periodic-table {{
            display: grid;
            grid-template-columns: repeat(18, 55px);
            grid-auto-rows: 55px;
            grid-gap: 8px;
            padding: 30px;
            background-color: #fff;
            border-radius: 12px;
            box-shadow: 0 8px 20px rgba(0, 0, 0, 0.15);
        }}
        .element {{
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;
            background-color: #a8dadc;
            border: 1px solid #79a6a8;
            border-radius: 8px;
            text-align: center;
            cursor: pointer;
            transition: transform 0.15s ease-in-out, box-shadow 0.15s ease-in-out;
        }}
        .element:hover {{
            transform: scale(1.1);
            box-shadow: 0 10px 25px rgba(0, 0, 0, 0.2);
        }}
        .element strong {{
            display: block;
            font-size: 1.4em;
            color: #1d3557;
            margin-bottom: 4px;
        }}
        .element span {{
            font-size: 0.8em;
            color: #457b9d;
        }}
        .lanthanides-actinides-wrapper {{
            display: flex;
            flex-direction: column;
            align-items: center;
            margin-top: 40px;
        }}
        .lanthanides, .actinides {{
            display: grid;
            grid-template-columns: repeat(15, 55px);
            grid-gap: 8px;
            margin-top: 10px;
        }}
        .alkali-metal {{ background-color: #f9844a; border-color: #e07741; color: #fff; }}
        .alkaline-earth-metal {{ background-color: #f9c74f; border-color: #e0b745; color: #373737; }}
        .transition-metal {{ background-color: #90be6d; border-color: #7cb162; color: #fff; }}
        .post-transition-metal {{ background-color: #43aa8b; border-color: #3a9a7d; color: #fff; }}
        .metalloid {{ background-color: #4d908e; border-color: #44827f; color: #fff; }}
        .nonmetal {{ background-color: #577590; border-color: #4e6a81; color: #fff; }}
        .halogen {{ background-color: #277da1; border-color: #226e8c; color: #fff; }}
        .noble-gas {{ background-color: #f8961e; border-color: #e3861a; color: #fff; }}
        .lanthanide {{ background-color: #c6dabf; border-color: #b4c9ae; color: #373737; }}
        .actinide {{ background-color: #e9c46a; border-color: #d3b15f; color: #373737; }}
        /* Specific positions for elements */
        {}
    </style>
</head>
<body>
    <h1>The Periodic Table</h1>
    <div class="periodic-table">
        {}
    </div>
    <div class="lanthanides-actinides-wrapper">
        <h2>Lanthanides</h2>
        <div class="lanthanides">
            {}
        </div>
        <h2>Actinides</h2>
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

fn generate_elements_html(elements: &[(&str, &str, i32, i32, &str)]) -> String {
    let mut html = String::new();
    for &(symbol, name, group, period, element_type) in elements {
        if group > 0 {
            html.push_str(&format!(r#"<div class="element {}" style="grid-column: {}; grid-row: {};"><strong title="{}">{}</strong><span>{}</span></div>"#, element_type, group, period, name, symbol, name));
        }
    }
    html
}

fn generate_elements_html_secondary(elements: &[(&str, &str, &str)]) -> String {
    let mut html = String::new();
    for &(symbol, name, element_type) in elements {
        html.push_str(&format!(r#"<div class="element {}"><strong title="{}">{}</strong><span>{}</span></div>"#, element_type, name, symbol, name));
    }
    html
}

fn generate_specific_positions(elements: &[(&str, &str, i32, i32, &str)]) -> String {
    let mut css = String::new();
    for &(symbol, _, group, period, _) in elements {
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
